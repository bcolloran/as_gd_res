pub mod copyable_base_type_impls;

pub use as_gd_res_derive::*;
use godot::meta::ArrayElement;

use std::ops::Deref;

use godot::obj::{bounds, Bounds, Gd, GodotClass};
use godot::prelude::*;

pub trait AsGdRes {
    type ResType: ExtractGd;
}

pub trait ExtractGd {
    type Extracted: Sized;
    fn extract(&self) -> Self::Extracted;
}

//////////////
// godot-rust smart pointers
//////////////

/////// DynGd //////////

impl<T> ExtractGd for Gd<T>
where
    T: ExtractGd + GodotClass + Bounds<Declarer = bounds::DeclUser>,
{
    type Extracted = T::Extracted;
    fn extract(&self) -> Self::Extracted {
        T::extract(&self.bind())
    }
}

/////// DynGd //////////

impl<T> ExtractGd for DynGd<Resource, T>
where
    T: ExtractGd,
{
    type Extracted = T::Extracted;
    fn extract(&self) -> Self::Extracted {
        self.dyn_bind().extract()
    }
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

/////// String <-> GString //////////

impl AsGdRes for String {
    type ResType = GString;
}
impl ExtractGd for GString {
    type Extracted = String;
    fn extract(&self) -> Self::Extracted {
        self.to_string()
    }
}

/////// PackedScenePath <-> Gd<PackedScene> //////////

pub struct PackedScenePath(pub String);

impl AsGdRes for PackedScenePath {
    type ResType = Gd<PackedScene>;
}

impl ExtractGd for Gd<PackedScene> {
    type Extracted = PackedScenePath;
    fn extract(&self) -> Self::Extracted {
        let path = self.get_path();
        PackedScenePath(path.to_string())
    }
}

/////// OnEditorInit <-> OnEditor //////////

impl<T> AsGdRes for OnEditorInit<T>
where
    T: AsGdRes,
{
    type ResType = OnEditor<T::ResType>;
}

pub struct OnEditorInit<T>(pub T);
impl<T> Deref for OnEditorInit<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
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

/////// OPTION //////////

impl<T> AsGdRes for Option<T>
where
    T: AsGdRes,
{
    type ResType = Option<T::ResType>;
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

/////// Vec <-> Array //////////

impl<T> AsGdRes for Vec<T>
where
    T: AsGdRes,
    T::ResType: ArrayElement,
{
    type ResType = Array<T::ResType>;
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
