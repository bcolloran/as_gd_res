pub mod copyable_base_type_impls;
pub mod engine_type_impls;

pub mod impl_wrapped_as_gd_res;

use std::fmt::Debug;

pub use as_gd_res_derive::*;
pub use as_simple_gd_enum_derive::*;

use godot::obj::{bounds, Bounds, Gd, GodotClass};
use godot::prelude::*;

pub trait ExportWrapper<T: ?Sized>: Export {
    type W;
}

pub trait AsGdRes: Clone {
    type ResType: ExtractGd + ?Sized;
}

pub trait AsSimpleGdEnum: Clone + Copy + Default + Debug + PartialEq + Eq {
    type GdEnumType: ExtractGd + ?Sized;
}

impl<T: AsSimpleGdEnum> AsGdRes for T {
    type ResType = <T as AsSimpleGdEnum>::GdEnumType;
}

pub trait ExtractGd {
    type Extracted;
    fn extract(&self) -> Self::Extracted;
}

//////////////
// godot-rust smart pointers
//////////////

pub trait ExtractGdHelper<D: bounds::Declarer> {
    type InnerExtracted;
    fn extract_inner(&self) -> Self::InnerExtracted;
}

impl<T> ExtractGdHelper<bounds::DeclUser> for Gd<T>
where
    T: GodotClass + Bounds<Declarer = bounds::DeclUser> + ExtractGd,
{
    type InnerExtracted = <T as ExtractGd>::Extracted;
    fn extract_inner(&self) -> Self::InnerExtracted {
        T::extract(&self.bind())
    }
}

pub trait ExtractGdEngineFn {
    type GdType;
    type Extracted;
    fn extract_inner(gd: Self::GdType) -> Self::Extracted;
}

impl<T> ExtractGdHelper<bounds::DeclEngine> for Gd<T>
where
    T: GodotClass + Bounds<Declarer = bounds::DeclEngine> + ExtractGd + ExtractGdEngineFn,
{
    type InnerExtracted = <T as ExtractGd>::Extracted;
    fn extract_inner(&self) -> Self::InnerExtracted {
        T::extract(&self)
    }
}

////////

/////// Gd //////////

impl<T> ExtractGd for Gd<T>
where
    T: GodotClass + Bounds, // T has Bounds::Declarer associated type
    Gd<T>: ExtractGdHelper<<T as Bounds>::Declarer>,
{
    type Extracted = <Gd<T> as ExtractGdHelper<T::Declarer>>::InnerExtracted;
    fn extract(&self) -> Self::Extracted {
        // Delegate to the corresponding helper impl:
        <Gd<T> as ExtractGdHelper<T::Declarer>>::extract_inner(self)
    }
}

/////// DynGd //////////

impl<T: ?Sized> ExtractGd for DynGd<Resource, T>
where
    T: ExtractGd,
{
    type Extracted = T::Extracted;
    fn extract(&self) -> Self::Extracted {
        self.dyn_bind().extract()
    }
}

/////// OnEditor //////////

impl<T> ExtractGd for OnEditor<T>
where
    T: ExtractGd,
{
    type Extracted = T::Extracted;
    fn extract(&self) -> Self::Extracted {
        T::extract(&self)
    }
}

/////// OPTION //////////
pub trait AsGdResOpt: Clone + Sized {
    type GdOption: ExtractGd + Export;
}

impl<T> AsGdRes for Option<T>
where
    T: AsGdResOpt + Sized,
{
    type ResType = T::GdOption;
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
pub trait AsGdResArray: Clone {
    type GdArray: ExtractGd + Export;
}

impl<T> AsGdRes for Vec<T>
where
    T: AsGdResArray,
{
    type ResType = T::GdArray;
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
