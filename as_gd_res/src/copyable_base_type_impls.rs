use crate::{AsGdRes, AsGdResArray, ExtractGd};

//////////////
// impls for core copyable types
//////////////
macro_rules! impl_extract_gd_copyable {
    ($($t:ty),*) => {
        $(
            impl ExtractGd for $t
            where
                $t: Copy,
            {
                type Extracted = Self;
                fn extract(&self) -> Self::Extracted {
                    *self
                }
            }
        )*
    };
    () => {

    };
}

impl_extract_gd_copyable! {
    i8, i16, i32, i64,
    u8, u16, u32,
    f32, f64,
    bool
}

/// NOTE: Option<$t> is not supported for numeric types, so not implemented here.
macro_rules! impl_as_res_gd_for_copyable {
    ($($t:ty),*) => {
        $(
            impl AsGdRes for $t
            where
                $t: Copy,
            {
                type ResType = Self;
            }

            impl AsGdResArray for $t
            where
                $t: Copy,
            {
                type GdArray = ::godot::prelude::Array<$t>;
            }
        )*
    };
    () => {

    };
}

impl_as_res_gd_for_copyable! {
    i8, i16, i32, i64,
    u8, u16, u32,
    f32, f64,
    bool
}
