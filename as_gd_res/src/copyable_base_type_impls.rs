use crate::{AsGdRes, ExtractGd};

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
    u8, u16, u32, u64,
    f32, f64,
    bool
}

macro_rules! impl_as_res_gd_for_copyable {
    ($($t:ty),*) => {
        $(


            impl AsGdRes for $t
            where
                $t: Copy,
            {
                type ResType = Self;
            }
        )*
    };
    () => {

    };
}

impl_as_res_gd_for_copyable! {
    i8, i16, i32, i64,
    u8, u16, u32, u64,
    f32, f64,
    bool
}
