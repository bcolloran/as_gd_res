pub mod gen_resource_spec;
pub mod gen_resource_try_out;

use extract_gd_derive::ExtractGd;
use godot::obj::{bounds, Bounds, Gd, GodotClass};
use godot::prelude::*;

pub trait ExtractGd {
    type Extracted: Sized;
    fn extract(&self) -> Self::Extracted;
}

pub struct OnEditorInit<T>(pub T);

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

////////////////
// Example manual impls impls for demonstration scenario.
// These are kind of "base" types for which we'll need to implement
// things like `ExtractGd` and `AsGdRes` manually, because a 1-1 translation
// may not be possible.
////////////////

impl ExtractGd for DynGd<Resource, dyn BrainParamsDynRes> {
    type Extracted = BrainState;
    fn extract(&self) -> Self::Extracted {
        self.dyn_bind().new_brain_state()
    }
}

// NOTE: need to manually impl `ExtractGd` for all `Gd<T>` where `T` is an engine type,
// i.e. `T: GodotClass + Bounds<Declarer = bounds::DeclUser>`

impl ExtractGd for Gd<PackedScene> {
    type Extracted = PackedScenePath;
    fn extract(&self) -> Self::Extracted {
        let path = self.get_path();
        PackedScenePath(path.to_string())
    }
}

// impl ExtractGd for PackedScene {
//     type Extracted = PackedScenePath;
//     fn extract(&self) -> Self::Extracted {
//         let path = self.get_path();
//         PackedScenePath(path.to_string())
//     }
// }

//////////////
// Examples of structs that we would want to derive `ExtractGd` for,
// along with the generated structs and impls we would want to generate.
//////////////

#[derive(Copy, Clone)]
pub struct EnumInnerState(pub f32, pub f32);

#[derive(Copy, Clone, ExtractGd)]
pub enum BrainState {
    DirectChaser,
    Roomba,
    Tank(EnumInnerState),
}

pub trait BrainParamsDynRes {
    fn new_brain_state(&self) -> BrainState;
}

#[derive(GodotClass)]
#[class(tool, init, base=Resource)]
#[derive(ExtractGd)]
// commented out because this is not implemented yet, this is an example of what we want to be able to do
#[extract_to(EnemyParamsExtracted)] // commented out because this is not implemented yet, this is an example of what we want to be able to do
pub struct EnemyParams {
    #[base]
    base: Base<Resource>,

    #[export]
    pub brain_params_required: OnEditor<DynGd<Resource, dyn BrainParamsDynRes>>,

    #[export]
    pub brain_params_optional: Option<DynGd<Resource, dyn BrainParamsDynRes>>,

    #[export]
    pub drop_params: Option<Gd<DropParams>>,
}

/// WE WANT TO DEFINE THIS STRUCT VIA MACRO!
/// for `derive(ExtractGd)` with an `extract_to` attribute,
/// we generate a new struct named according to the `extract_to` attribute.
/// The new struct will have the same fields as the original struct except
/// for the fields that are marked with `#[extract_ignore]`.
/// The new struct will have the same field names as the original struct,
/// but the field types will be the extracted types.
///
/// The new struct will have a `pub` visibility modifier.
///
/// For each valid field, if the original field type is `#type`,
/// the new field type should be `<#type as ExtractGd>::Extracted`.
// pub struct EnemyParamsExtracted {
//     pub brain_params_required:
//         <OnEditor<DynGd<Resource, dyn BrainParamsDynRes>> as ExtractGd>::Extracted,
//     pub brain_params_optional:
//         <Option<DynGd<Resource, dyn BrainParamsDynRes>> as ExtractGd>::Extracted,
//     pub drop_params: <Option<Gd<DropParams>> as ExtractGd>::Extracted,
// }

// /// WE WANT TO DEFINE THIS IMPL VIA MACRO!
// impl ExtractGd for EnemyParams {
//     type Extracted = EnemyParamsExtracted;
//     fn extract(&self) -> Self::Extracted {
//         Self::Extracted {
//             brain_params_required: self.brain_params_required.extract(),
//             brain_params_optional: self.brain_params_optional.extract(),
//             drop_params: self.drop_params.extract(),
//         }
//     }
// }

#[derive(GodotClass)]
#[class(tool, init, base=Resource)]
#[derive(ExtractGd)]
// commented out because this is not implemented yet, this is an example of what we want to be able to do
#[extract_to(DropParamsExtracted)] // commented out because this is not implemented yet, this is an example of what we want to be able to do
pub struct DropParams {
    #[base]
    #[extract_ignore]
    // commented out because this is not implemented yet, this is an example of what we want to be able to do
    base: Base<Resource>,

    #[export(range = (0.0, 500.0))]
    #[init(val = 20.0)]
    pub total_value: f32,

    #[export(range = (0.0, 500.0))]
    #[init(val = 5.0)]
    pub max_value_per_coin: f32,

    #[export]
    pub coin_scene: OnEditor<Gd<PackedScene>>,

    #[export]
    pub scenes_array: Array<Gd<PackedScene>>,
}

// WE WANT TO DEFINE THIS STRUCT VIA MACRO!
// pub struct DropParamsExtracted {
//     pub total_value: <f32 as ExtractGd>::Extracted,
//     pub max_value_per_coin: <f32 as ExtractGd>::Extracted,
//     pub coin_scene: <OnEditor<Gd<PackedScene>> as ExtractGd>::Extracted,
// }

// /// WE WANT TO DEFINE THIS IMPL VIA MACRO!
// impl ExtractGd for DropParams {
//     type Extracted = DropParamsExtracted;
//     fn extract(&self) -> Self::Extracted {
//         Self::Extracted {
//             total_value: self.total_value.extract(),
//             max_value_per_coin: self.max_value_per_coin.extract(),
//             coin_scene: self.coin_scene.extract(),
//         }
//     }
// }
