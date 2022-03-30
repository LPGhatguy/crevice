use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{parse_quote, Data, DeriveInput, Fields, Ident, Path, Type};

pub fn emit(
    input: DeriveInput,
    trait_name: &'static str,
    mod_name: &'static str,
    min_struct_alignment: usize,
) -> TokenStream {
    let mod_name = Ident::new(mod_name, Span::call_site());
    let trait_name = Ident::new(trait_name, Span::call_site());

    let mod_path: Path = parse_quote!(::crevice::#mod_name);
    let trait_path: Path = parse_quote!(#mod_path::#trait_name);

    let as_trait_name = format_ident!("As{}", trait_name);
    let as_trait_path: Path = parse_quote!(#mod_path::#as_trait_name);
    let as_trait_method = format_ident!("as_{}", mod_name);
    let from_trait_method = format_ident!("from_{}", mod_name);

    let array_name = format_ident!("{}ArrayItem", trait_name);
    let array_path: Path = parse_quote!(#mod_path::#array_name);

    let visibility = input.vis;
    let input_name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let generated_name = format_ident!("{}{}", trait_name, input_name);

    // Crevice's derive only works on regular structs. We could potentially
    // support transparent tuple structs in the future.
    let fields: Vec<_> = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields.named.iter().collect(),
            Fields::Unnamed(_) => panic!("Tuple structs are not supported"),
            Fields::Unit => panic!("Unit structs are not supported"),
        },
        Data::Enum(_) | Data::Union(_) => panic!("Only structs are supported"),
    };

    // Gives the layout-specific version of the given type.
    let layout_version_of_ty = |ty: &Type| {
        quote! {
            <#ty as #as_trait_path>::Output
        }
    };

    // Gives an expression returning the layout-specific alignment for the type.
    let layout_alignment_of_ty = |ty: &Type| {
        quote! {
            <<#ty as #as_trait_path>::Output as #trait_path>::ALIGNMENT
        }
    };

    let field_alignments = fields.iter().map(|field| layout_alignment_of_ty(&field.ty));
    let struct_alignment = quote! {
        ::crevice::internal::max_arr([
            #min_struct_alignment,
            #(#field_alignments,)*
        ])
    };

    // Generate names for each padding calculation function.
    let pad_fns: Vec<_> = (0..fields.len())
        .map(|index| format_ident!("_{}__{}Pad{}", input_name, trait_name, index))
        .collect();

    // Computes the offset immediately AFTER the field with the given index.
    //
    // This function depends on the generated padding calculation functions to
    // do correct alignment. Be careful not to cause recursion!
    let offset_after_field = |target: usize| {
        let mut output = vec![quote!(0usize)];

        for index in 0..=target {
            let field_ty = &fields[index].ty;
            let layout_ty = layout_version_of_ty(field_ty);

            output.push(quote! {
                + ::core::mem::size_of::<#layout_ty>()
            });

            // For every field except our target field, also add the generated
            // padding. Padding occurs after each field, so it isn't included in
            // this value.
            if index < target {
                let pad_fn = &pad_fns[index];
                output.push(quote! {
                    + #pad_fn()
                });
            }
        }

        output.into_iter().collect::<TokenStream>()
    };

    let pad_fn_impls: TokenStream = pad_fns
        .iter()
        .enumerate()
        .map(|(index, pad_fn)| {
            let starting_offset = offset_after_field(index);

            let next_field_or_self_alignment = fields
                .get(index + 1)
                .map(|next_field| layout_alignment_of_ty(&next_field.ty))
                .unwrap_or(quote!(#struct_alignment));

            quote! {
                /// Tells how many bytes of padding have to be inserted after
                /// the field with index #index.
                #[allow(non_snake_case)]
                const fn #pad_fn() -> usize {
                    // First up, calculate our offset into the struct so far.
                    // We'll use this value to figure out how far out of
                    // alignment we are.
                    let starting_offset = #starting_offset;

                    // We set our target alignment to the larger of the
                    // alignment due to the previous field and the alignment
                    // requirement of the next field.
                    let alignment = #next_field_or_self_alignment;

                    // Using everything we've got, compute our padding amount.
                    ::crevice::internal::align_offset(starting_offset, alignment)
                }
            }
        })
        .collect();

    let generated_struct_fields: TokenStream = fields
        .iter()
        .enumerate()
        .map(|(index, field)| {
            let field_name = field.ident.as_ref().unwrap();
            let field_ty = layout_version_of_ty(&field.ty);
            let pad_field_name = format_ident!("_pad{}", index);
            let pad_fn = &pad_fns[index];

            quote! {
                #field_name: #field_ty,
                #pad_field_name: [u8; #pad_fn()],
            }
        })
        .collect();

    let generated_struct_field_init: TokenStream = fields
        .iter()
        .map(|field| {
            let field_name = field.ident.as_ref().unwrap();

            quote! {
                #field_name: self.#field_name.#as_trait_method(),
            }
        })
        .collect();

    let input_struct_field_init: TokenStream = fields
        .iter()
        .map(|field| {
            let field_name = field.ident.as_ref().unwrap();

            quote! {
                #field_name: #as_trait_path::#from_trait_method(input.#field_name),
            }
        })
        .collect();

    let struct_definition = quote! {
        #[derive(Debug, Clone, Copy)]
        #[repr(C)]
        #[allow(non_snake_case)]
        #visibility struct #generated_name #ty_generics #where_clause {
            #generated_struct_fields
        }
    };

    let debug_methods = if cfg!(feature = "debug-methods") {
        let debug_fields: TokenStream = fields
            .iter()
            .map(|field| {
                let field_name = field.ident.as_ref().unwrap();
                let field_ty = &field.ty;

                quote! {
                    fields.push(Field {
                        name: stringify!(#field_name),
                        size: ::core::mem::size_of::<#field_ty>(),
                        offset: (&zeroed.#field_name as *const _ as usize)
                            - (&zeroed as *const _ as usize),
                    });
                }
            })
            .collect();

        quote! {
            impl #impl_generics #generated_name #ty_generics #where_clause {
                fn debug_metrics() -> String {
                    let size = ::core::mem::size_of::<Self>();
                    let align = <Self as #trait_path>::ALIGNMENT;

                    let zeroed: Self = ::crevice::internal::bytemuck::Zeroable::zeroed();

                    #[derive(Debug)]
                    struct Field {
                        name: &'static str,
                        offset: usize,
                        size: usize,
                    }
                    let mut fields = Vec::new();

                    #debug_fields

                    format!("Size {}, Align {}, fields: {:#?}", size, align, fields)
                }

                fn debug_definitions() -> &'static str {
                    stringify!(
                        #struct_definition
                        #pad_fn_impls
                    )
                }
            }
        }
    } else {
        quote!()
    };

    let array_item_impl = if cfg!(feature = "arrays") {
        quote! {
            unsafe impl #impl_generics #array_path for #generated_name #ty_generics #where_clause {
                type Padding = [u8; 0];
            }
        }
    } else {
        quote!()
    };

    quote! {
        #pad_fn_impls
        #struct_definition

        unsafe impl #impl_generics ::crevice::internal::bytemuck::Zeroable for #generated_name #ty_generics #where_clause {}
        unsafe impl #impl_generics ::crevice::internal::bytemuck::Pod for #generated_name #ty_generics #where_clause {}

        unsafe impl #impl_generics #trait_path for #generated_name #ty_generics #where_clause {
            const ALIGNMENT: usize = #struct_alignment;
        }

        #array_item_impl

        impl #impl_generics #as_trait_path for #input_name #ty_generics #where_clause {
            type Output = #generated_name;

            fn #as_trait_method(&self) -> Self::Output {
                Self::Output {
                    #generated_struct_field_init

                    ..::crevice::internal::bytemuck::Zeroable::zeroed()
                }
            }

            fn #from_trait_method(input: Self::Output) -> Self {
                Self {
                    #input_struct_field_init
                }
            }
        }

        #debug_methods
    }
}
