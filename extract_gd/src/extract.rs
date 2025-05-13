use extract_gd_derive::ExtractGd;
use godot::obj::{bounds, Bounds, Gd, GodotClass};
use godot::prelude::*;

pub trait ExtractGd {
    type Extracted: Sized;
    fn extract(&self) -> Self::Extracted;
}

//////////////
// Manual impls for core types
//////////////
pub struct PackedScenePath(pub String);

impl ExtractGd for f32
where
    f32: Copy,
{
    type Extracted = f32;
    fn extract(&self) -> Self::Extracted {
        *self
    }
}

impl ExtractGd for Gd<PackedScene> {
    type Extracted = PackedScenePath;
    fn extract(&self) -> Self::Extracted {
        let path = self.get_path();
        PackedScenePath(path.to_string())
    }
}

impl<T> ExtractGd for Gd<T>
where
    T: GodotClass + ExtractGd + GodotClass + Bounds<Declarer = bounds::DeclUser>,
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
    type Extracted = T::Extracted;
    fn extract(&self) -> Self::Extracted {
        T::extract(&self)
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

impl ExtractGd for DynGd<Resource, dyn BrainParamsDynRes> {
    type Extracted = BrainState;
    fn extract(&self) -> Self::Extracted {
        self.dyn_bind().new_brain_state()
    }
}

//////////////
// Examples of structs that we would want to derive `ExtractGd` for,
// along with the generated structs and impls we would want to generate.
//////////////

#[derive(Copy, Clone)]
pub struct EnumInnerState(pub f32, pub f32);

#[derive(Copy, Clone)]
// #[derive(ExtractGd)]
pub enum BrainState {
    DirectChaser,
    Roomba,
    Tank(EnumInnerState),
}

// WE WANT TO DEFINE THIS STRUCT VIA MACRO!
// For `derive(ExtractGd)` without an `extract_to` attribute,
// we assume the terget type is Clone, and we extract to the same type
// as the source type usint a `.clone()` call.
impl ExtractGd for BrainState {
    type Extracted = BrainState;
    fn extract(&self) -> Self::Extracted {
        self.clone()
    }
}

pub trait BrainParamsDynRes {
    fn new_brain_state(&self) -> BrainState;
}

#[derive(GodotClass)]
#[class(tool, init, base=Resource)]
// #[derive(ExtractGd)] // commented out because this is not implemented yet, this is an example of what we want to be able to do
// #[extract_to=EnemyParamsExtracted] // commented out because this is not implemented yet, this is an example of what we want to be able to do
pub struct EnemyParams {
    #[base]
    // #[extract_ignore] // commented out because this is not implemented yet, this is an example of what we want to be able to do
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
pub struct EnemyParamsExtracted {
    pub brain_params_required:
        <OnEditor<DynGd<Resource, dyn BrainParamsDynRes>> as ExtractGd>::Extracted,
    pub brain_params_optional:
        <Option<DynGd<Resource, dyn BrainParamsDynRes>> as ExtractGd>::Extracted,
    pub drop_params: <Option<Gd<DropParams>> as ExtractGd>::Extracted,
}

/// WE WANT TO DEFINE THIS IMPL VIA MACRO!
impl ExtractGd for EnemyParams {
    type Extracted = EnemyParamsExtracted;
    fn extract(&self) -> Self::Extracted {
        Self::Extracted {
            brain_params_required: self.brain_params_required.extract(),
            brain_params_optional: self.brain_params_optional.extract(),
            drop_params: self.drop_params.extract(),
        }
    }
}

#[derive(GodotClass)]
#[class(tool, init, base=Resource)]
#[derive(ExtractGd)]
// commented out because this is not implemented yet, this is an example of what we want to be able to do
#[extract_to = "DropParamsExtracted"] // commented out because this is not implemented yet, this is an example of what we want to be able to do
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
}

/// WE WANT TO DEFINE THIS STRUCT VIA MACRO!
pub struct DropParamsExtracted {
    pub total_value: <f32 as ExtractGd>::Extracted,
    pub max_value_per_coin: <f32 as ExtractGd>::Extracted,
    pub coin_scene: <OnEditor<Gd<PackedScene>> as ExtractGd>::Extracted,
}

/// WE WANT TO DEFINE THIS IMPL VIA MACRO!
impl ExtractGd for DropParams {
    type Extracted = DropParamsExtracted;
    fn extract(&self) -> Self::Extracted {
        Self::Extracted {
            total_value: self.total_value.extract(),
            max_value_per_coin: self.max_value_per_coin.extract(),
            coin_scene: self.coin_scene.extract(),
        }
    }
}
