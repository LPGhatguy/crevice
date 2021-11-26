use proc_macro2::{Literal, TokenStream};
use quote::quote;
#[cfg(feature = "arrays")]
use syn::TypeArray;
use syn::{parse_quote, Data, DeriveInput, Expr, Fields, Path, Type};

pub fn emit(input: DeriveInput) -> TokenStream {
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields,
            Fields::Unnamed(_) => panic!("Tuple structs are not supported"),
            Fields::Unit => panic!("Unit structs are not supported"),
        },
        Data::Enum(_) | Data::Union(_) => panic!("Only structs are supported"),
    };

    let base_trait_path: Path = parse_quote!(::crevice::glsl::Glsl);
    let struct_trait_path: Path = parse_quote!(::crevice::glsl::GlslStruct);

    let name = input.ident;
    let name_str = Literal::string(&name.to_string());

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let glsl_fields = fields.named.iter().map(|field| {
        let field_ty = &field.ty;
        let (base_ty, array_suffix) = remove_array_layers(field_ty);
        let field_name_str = Literal::string(&field.ident.as_ref().unwrap().to_string());

        quote! {
            ::crevice::glsl::GlslField {
                ty: <#base_ty as ::crevice::glsl::Glsl>::NAME,
                name: #field_name_str,
                dim: #array_suffix,
            }
        }
    });

    quote! {
        unsafe impl #impl_generics #base_trait_path for #name #ty_generics #where_clause {
            const NAME: &'static str = #name_str;
        }

        unsafe impl #impl_generics #struct_trait_path for #name #ty_generics #where_clause {
            const FIELDS: &'static [::crevice::glsl::GlslField] = &[
                #( #glsl_fields, )*
            ];
        }
    }
}

#[cfg(feature = "arrays")]
fn remove_array_layers(mut ty: &Type) -> (&Type, Expr) {
    let mut suffix = quote!("");

    loop {
        match ty {
            &Type::Array(TypeArray {
                ref elem, ref len, ..
            }) => {
                ty = elem.as_ref();
                suffix = quote!(
                    ::crevice::internal::const_format::concatcp!("[", (#len as usize), "]", #suffix)
                );
            }
            _ => break,
        }
    }
    (ty, Expr::Verbatim(suffix))
}
#[cfg(not(feature = "arrays"))]
fn remove_array_layers(ty: &Type) -> (&Type, Expr) {
    (ty, Expr::Verbatim(quote!("")))
}
