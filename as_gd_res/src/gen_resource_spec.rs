use godot::prelude::*;
use godot::{meta::ArrayElement, obj::Gd};

use crate::{AsGdRes, ExtractGd, OnEditorInit, PackedScenePath};

//////////////
// NOTE: this file compiles as-is, and is meant to check that the patterns we're
// using for the `#[derive(AsGdRes)]` macro are correct.
// However, the `#[derive(AsGdRes)]` macro is not implemented yet, so
// the places where `#[derive(AsGdRes)]` is commented out are examples of
// how we'd like to be able to use it (and likewise for the `#[export(...)]`
// and `#[init(...)]` attributes on structs targeted by the derive).
//
////////////////

//////////////
// impls for core copyable types
//////////////

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

//////////////
// Manual impls for core types
//
// Example manual impls impls for demonstration scenario.
// These are kind of "base" types for which we'll need to implement
// `ExtractGd` and `AsGdRes` manually.
////////////////

impl<T> AsGdRes for OnEditorInit<T>
where
    T: AsGdRes,
{
    type ResType = OnEditor<T::ResType>;
}

impl<T> AsGdRes for Option<T>
where
    T: AsGdRes,
{
    type ResType = Option<T::ResType>;
}

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

impl<T> AsGdRes for Vec<T>
where
    T: AsGdRes,
    T::ResType: ArrayElement,
{
    type ResType = Array<T::ResType>;
}

////////////////
// Example of a composite structs that we'll want to be able to
// use `#[derive(AsGdRes)]` on.
//
// For each field in the composite struct, if it includes any `#[export(...)]` or `#[init(...)]` attributes,
// the genereated `ResType` should include those attributes on the generated struct.
// If the field has no attribets, we must add the `#[export]` attribute to the generated struct.
////////////////

// #[derive(AsGdRes)] // commented out because this is not implemented yet, this is an example of what we want to be able to do
pub struct DropParams2 {
    // #[export(range = (0.0, 500.0))] // commented out for now, but `derive(AsGdRes)` should pass this through to the generated struct
    // #[init(val = 20.0)] // commented out for now, but `derive(AsGdRes)` should pass this through to the generated struct
    pub total_value: f32,

    // #[export(range = (0.0, 500.0))] // commented out for now, but `derive(AsGdRes)` should pass this through to the generated struct
    // #[init(val = 20.0)] // commented out for now, but `derive(AsGdRes)` should pass this through to the generated struct
    pub max_value_per_coin: f32,

    // this field has no attrs, `derive(AsGdRes)` should add the `#[export]` attribute
    pub coin_scene_1: Option<PackedScenePath>,
    // this field has no attrs, `derive(AsGdRes)` should add the `#[export]` attribute
    pub coin_scene_2: OnEditorInit<PackedScenePath>,
}

// ******** example #[derive(AsGdRes)] output, start ********
impl AsGdRes for DropParams2 {
    // `#[derive(AsGdRes)]` on a type named `#{name}` should generate a ResType
    // by appending "Resource" at the end of the input name, like:
    // `type ResType = Gd<{#name}Resource>;`
    type ResType = Gd<DropParams2Resource>;
}

#[derive(GodotClass)]
#[class(tool, init, base=Resource)]
pub struct DropParams2Resource {
    // we will always add the `base: Base<Resource>` field to the generated struct,
    // and always with the `#[base]` attribute
    #[base]
    base: Base<Resource>,

    #[export(range = (0.0, 500.0))]
    #[init(val = 20.0)]
    pub total_value: <f32 as AsGdRes>::ResType,

    #[export(range = (0.0, 500.0))]
    #[init(val = 5.0)]
    pub max_value_per_coin: <f32 as AsGdRes>::ResType,

    #[export]
    pub coin_scene_1: <Option<PackedScenePath> as AsGdRes>::ResType,
    #[export]
    pub coin_scene_2: <OnEditorInit<PackedScenePath> as AsGdRes>::ResType,
}

impl ExtractGd for DropParams2Resource {
    type Extracted = DropParams2;
    fn extract(&self) -> Self::Extracted {
        Self::Extracted {
            total_value: self.total_value.extract(),
            max_value_per_coin: self.max_value_per_coin.extract(),
            coin_scene_1: self.coin_scene_1.extract(),
            coin_scene_2: self.coin_scene_2.extract(),
        }
    }
}
// ******** example #[derive(AsGdRes)] output, end ********

////////////////
// Another example of a composite structs that we'll want to be able to
// use `#[derive(AsGdRes)]` on.
////////////////

#[derive(Copy, Clone)]
// #[derive(AsGdRes)] // commented out because this is not implemented yet, this is an example of what we want to be able to do
pub struct TankBrainParams {
    // #[export(range = (100.0, 300.0))] // commented out for now, but `derive(AsGdRes)` should pass this through to the generated struct
    // #[init(val = 220.0)] // commented out for now, but `derive(AsGdRes)` should pass this through to the generated struct
    pub speed: f32,

    // #[export(range = (0.1, 3.14))] // commented out for now, but `derive(AsGdRes)` should pass this through to the generated struct
    // #[init(val = 2.0)] // commented out for now, but `derive(AsGdRes)` should pass this through to the generated struct
    pub turn_speed: f32,
}

// ******** example #[derive(AsGdRes)] output, start ********
impl AsGdRes for TankBrainParams {
    // `#[derive(AsGdRes)]` on a type named `#{name}` should generate a ResType
    // by appending "Resource" at the end of the input name, like:
    // `type ResType = Gd<{#name}Resource>;`
    type ResType = Gd<TankBrainParamsResource>;
}

#[derive(GodotClass)]
#[class(tool, init, base=Resource)]
pub struct TankBrainParamsResource {
    // we will always add the `base: Base<Resource>` field to the generated struct,
    // and always with the `#[base]` attribute
    #[base]
    base: Base<Resource>,

    #[export(range = (100.0, 300.0))]
    #[init(val = 220.0)]
    pub speed: <f32 as AsGdRes>::ResType,

    #[export(range = (0.1, 3.14))]
    #[init(val = 2.0)]
    pub turn_speed: <f32 as AsGdRes>::ResType,
}

impl ExtractGd for TankBrainParamsResource {
    type Extracted = TankBrainParams;
    fn extract(&self) -> Self::Extracted {
        Self::Extracted {
            speed: self.speed.extract(),
            turn_speed: self.turn_speed.extract(),
        }
    }
}
// ******** example #[derive(AsGdRes)] output, end ********

#[derive(Copy, Clone)]
// #[derive(AsGdRes)] // commented out because this is not implemented yet, this is an example of what we want to be able to do
pub struct RoombaBrainParams {
    // #[export(range = (100.0, 300.0))]  // commented out for now, but `derive(AsGdRes)` should pass this through to the generated struct
    // #[init(val = 220.0)] // commented out for now, but `derive(AsGdRes)` should pass this through to the generated struct
    pub speed: f32,

    // #[export(range = (0.1, 3.14))] // commented out for now, but `derive(AsGdRes)` should pass this through to the generated struct
    // #[init(val = 2.0)] // commented out for now, but `derive(AsGdRes)` should pass this through to the generated struct
    pub turn_speed: f32,
}

// ******** example #[derive(AsGdRes)] output, start ********
impl AsGdRes for RoombaBrainParams {
    // `#[derive(AsGdRes)]` on a type non-tuple struct named `#{name}` should
    // generate a ResType
    // by appending "Resource" at the end of the input name, like:
    // `type ResType = Gd<{#name}Resource>;`
    type ResType = Gd<RoombaBrainParamsResource>;
}

#[derive(GodotClass)]
#[class(tool, init, base=Resource)]
pub struct RoombaBrainParamsResource {
    // we will always add the `base: Base<Resource>` field to the generated struct,
    // and always with the `#[base]` attribute
    #[base]
    base: Base<Resource>,

    #[export(range = (100.0, 300.0))]
    #[init(val = 220.0)]
    pub speed: <f32 as AsGdRes>::ResType,

    #[export(range = (0.1, 3.14))]
    #[init(val = 2.0)]
    pub turn_speed: <f32 as AsGdRes>::ResType,
}

impl ExtractGd for RoombaBrainParamsResource {
    type Extracted = RoombaBrainParams;
    fn extract(&self) -> Self::Extracted {
        Self::Extracted {
            speed: self.speed.extract(),
            turn_speed: self.turn_speed.extract(),
        }
    }
}
// ******** example #[derive(AsGdRes)] output, end ********

// ENUMS
//
//
// ////////////////
// NOTE:
// `#[derive(AsGdRes)]` only works on enums where either:
// 1. all variants have a single, unnamed associated data type
// -or-
// 2. all variants are unit variants (i.e. no associated data)
//
// In any other case, the macro should emit an error saying that these conditions have not been met
// ////////////////
//
//

///// SIMPLE ENUMS
/// Note that godot-rust does not support `Option<SomeEnum>`; If you want an "optional" enum, include a `None` variant in the enum itself, and set that as the default value.
/// `Array<SomeEnum>` is also not supported

// for a simple enum with no associated data, we generate an
#[derive(Default, Clone, Copy, GodotConvert, Var, Export, PartialEq)]
#[godot(via = GString)]
// #[derive(AsGdRes)] // commented out because this is not implemented yet, this is an example of what we want to be able to do
pub enum DamageTeam {
    #[default]
    Player,
    Enemy,
    Environment,
}

// For an simple enum with no associated data (i _think_ (but I'm not sure) that this is equivalent to all variants matching `syn::data::Fields::Unit`), we can just use the enum itself as the `ResType`, and as the `Extracted` type.
// ******** example output #[derive(AsGdRes)] for DamageTeam, START ********
impl AsGdRes for DamageTeam {
    type ResType = DamageTeam;
}

impl ExtractGd for DamageTeam {
    type Extracted = DamageTeam;
    fn extract(&self) -> Self::Extracted {
        self.clone()
    }
}
// ******** example output #[derive(AsGdRes)] for DamageTeam, END ********

///// Enums with associated data

#[derive(Copy, Clone)]
// #[derive(AsGdRes)] // commented out because this is not implemented yet, this is an example of what we want to be able to do
pub enum BrainParams {
    Roomba(RoombaBrainParams),
    Tank(TankBrainParams),
}

// In the case of an enum where all variants have a single associated data type (i _think_ (but I'm not sure) that this is equivalent to all variants matching `syn::data::Fields::Unnamed`), we need to create a new trait `#{enum_name}DynRes` with a method `extract_enum_data`. The associated type for each enum variant will impl this trait, extracting the spcific data type for that variant.
// ******** example output #[derive(AsGdRes)] for BrainParams, START ********
pub trait BrainParamsEnumDynEnumResource {
    fn extract_enum_data(&self) -> BrainParams;
}
// impls for the enum variants
impl BrainParamsEnumDynEnumResource for RoombaBrainParamsResource {
    fn extract_enum_data(&self) -> BrainParams {
        BrainParams::Roomba(self.extract())
    }
}
impl BrainParamsEnumDynEnumResource for TankBrainParamsResource {
    fn extract_enum_data(&self) -> BrainParams {
        BrainParams::Tank(self.extract())
    }
}

// the `AsGdRes` impl for the enum itself will be a `DynGd<Resource, dyn #{enum_name}EnumDynRes>``
impl AsGdRes for BrainParams {
    type ResType = DynGd<Resource, dyn BrainParamsEnumDynEnumResource>;
}

// the `ExtractGd` impl for `DynGd<Resource, dyn #{enum_name}EnumDynRes>` will `dyn_bind` the dyn compatible Resouce, and call `extract_enum_data` on to get back the enum variant
impl ExtractGd for DynGd<Resource, dyn BrainParamsEnumDynEnumResource> {
    type Extracted = BrainParams;
    fn extract(&self) -> Self::Extracted {
        self.dyn_bind().extract_enum_data()
    }
}

// ******** example output #[derive(AsGdRes)] for BrainParams, END ********

//
///////////////////// a complex composite struct that includes enums
//

// #[derive(AsGdRes)] // commented out because this is not implemented yet, this is an example of what we want to be able to do
pub struct EnemyParams {
    pub enemy_name: String,
    // this field has no attrs, `derive(AsGdRes)` should add the `#[export]` attribute
    pub brain_params_required: OnEditorInit<BrainParams>,
    // this field has no attrs, `derive(AsGdRes)` should add the `#[export]` attribute
    pub brain_params_optional: Option<BrainParams>,
    // this field has no attrs, `derive(AsGdRes)` should add the `#[export]` attribute
    pub brains_vec: Vec<BrainParams>,

    // this field has no attrs, `derive(AsGdRes)` should add the `#[export]` attribute
    pub drop_params: Option<DropParams2>,
    // this field has no attrs, `derive(AsGdRes)` should add the `#[export]` attribute
    pub damage_team: DamageTeam,
}

// // ******** example #[derive(AsGdRes)] output, start ********
impl AsGdRes for EnemyParams {
    type ResType = Gd<EnemyParamsResource>;
}

#[derive(GodotClass)]
#[class(tool, init, base=Resource)]
pub struct EnemyParamsResource {
    // we will always add the `base: Base<Resource>` field to the generated struct,
    // and always with the `#[base]` attribute
    #[base]
    base: Base<Resource>,

    #[export]
    pub enemy_name: <String as AsGdRes>::ResType,

    #[export]
    pub brain_params_required: <OnEditorInit<BrainParams> as AsGdRes>::ResType,
    #[export]
    pub brain_params_optional: <Option<BrainParams> as AsGdRes>::ResType,
    #[export]
    pub brains_vec: <Vec<BrainParams> as AsGdRes>::ResType,

    #[export]
    pub drop_params: <Option<DropParams2> as AsGdRes>::ResType,
    #[export]
    pub damage_team: <DamageTeam as AsGdRes>::ResType,
}

impl ExtractGd for EnemyParamsResource {
    type Extracted = EnemyParams;
    fn extract(&self) -> Self::Extracted {
        Self::Extracted {
            enemy_name: self.enemy_name.extract(),
            brain_params_required: self.brain_params_required.extract(),
            brain_params_optional: self.brain_params_optional.extract(),
            brains_vec: self.brains_vec.extract(),
            drop_params: self.drop_params.extract(),
            damage_team: self.damage_team.extract(),
        }
    }
}
// // ******** example #[derive(AsGdRes)] output, end ********
