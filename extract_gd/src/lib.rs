pub mod gen_resource_spec;

use std::ops::Deref;

use godot::obj::{bounds, Bounds, Gd, GodotClass};
use godot::prelude::*;

pub trait ExtractGd {
    type Extracted: Sized;
    fn extract(&self) -> Self::Extracted;
}

pub struct OnEditorInit<T>(pub T);
impl<T> Deref for OnEditorInit<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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

//////////////
// impls for core cloneable types
//////////////

macro_rules! impl_extract_gd_cloneable {
    ($($t:ty),*) => {
        $(
            impl ExtractGd for $t
            where
                $t: Clone,
            {
                type Extracted = Self;
                fn extract(&self) -> Self::Extracted {
                    self.clone()
                }
            }
        )*
    };
    () => {

    };
}

impl_extract_gd_cloneable!(String);

//////////////
// Manual impls for core types
//////////////
pub struct PackedScenePath(pub String);

impl<T> ExtractGd for Gd<T>
where
    T: ExtractGd + GodotClass + Bounds<Declarer = bounds::DeclUser>,
{
    type Extracted = T::Extracted;
    fn extract(&self) -> Self::Extracted {
        T::extract(&self.bind())
    }
}

impl<T> ExtractGd for OnEditor<T>
where
    T: ExtractGd,
{
    type Extracted = OnEditorInit<T::Extracted>;
    fn extract(&self) -> Self::Extracted {
        OnEditorInit(T::extract(&self))
    }
}

impl<T> ExtractGd for Option<T>
where
    T: ExtractGd,
{
    type Extracted = Option<T::Extracted>;
    fn extract(&self) -> Self::Extracted {
        self.as_ref().map(|v| v.extract())
    }
}

impl<T> ExtractGd for DynGd<Resource, T>
where
    T: ExtractGd,
{
    type Extracted = T::Extracted;
    fn extract(&self) -> Self::Extracted {
        self.dyn_bind().extract()
    }
}

impl<T> ExtractGd for Array<T>
where
    T: ExtractGd + godot::meta::ArrayElement,
{
    type Extracted = Vec<T::Extracted>;
    fn extract(&self) -> Self::Extracted {
        self.iter_shared().map(|v| v.extract()).collect()
    }
}
