#[macro_export]
macro_rules! assert_std140_offsets {
    ((size = $size:literal, align = $align:literal) $struct:ident {
        $( $field:ident: $offset:literal, )*
    }) => {
        type Target = <$struct as crevice::std140::AsStd140>::Output;

        let mut fail = false;

        let actual_size = std::mem::size_of::<Target>();
        if actual_size != $size {
            fail = true;
            println!(
                "Invalid size for struct {}\n\
                Expected: {}\n\
                Actual:   {}\n",
                stringify!($struct),
                $size,
                actual_size,
            );
        }

        let actual_alignment = <Target as crevice::std140::Std140>::ALIGNMENT;
        if actual_alignment != $align {
            fail = true;
            println!(
                "Invalid alignment for struct {}\n\
                Expected: {}\n\
                Actual:   {}\n",
                stringify!($struct),
                $align,
                actual_alignment,
            );
        }

        $({
            let actual_offset = memoffset::offset_of!(Target, $field);
            if actual_offset != $offset {
                fail = true;
                println!(
                    "Invalid offset for field {}\n\
                    Expected: {}\n\
                    Actual:   {}\n",
                    stringify!($field),
                    $offset,
                    actual_offset,
                );
            }
        })*

        if fail {
            panic!("Invalid output for {}", stringify!($struct));
        }
    };
}
