use as_gd_res::engine_type_impls::PackedScenePath;
use as_gd_res::engine_type_impls::RustCurve;
use as_gd_res::AsGdRes;
use as_gd_res::ExtractGd;
// use godot::classes::Curve;
use godot::prelude::*;

struct ResourceTestExtension;

#[gdextension]
unsafe impl ExtensionLibrary for ResourceTestExtension {}

#[derive(GodotClass)]
#[class(init, base=Node)]
struct TestNode {
    base: Base<Node>,
    #[export]
    test_int: i32,

    //
    // #[export]
    // test_int_opt: Option<i32>,
    #[export]
    test_gstring: GString,
    // #[export]
    // // test_simple_res: OnEditor<Gd<SimpleDataResource>>,
    #[export]
    test_simple_res: <SimpleData as AsGdRes>::ResType,
    #[export]
    some_enum: <Pickup as AsGdRes>::ResType,
    #[export]
    crazy_nested_resource: <Complicated as AsGdRes>::ResType,
}

#[godot_api]
impl INode for TestNode {
    // Called when the node is ready in the scene tree.
    fn ready(&mut self) {
        godot_print!("TestNode is ready!");
        godot_print!("test_simple_res: {:?}", self.test_simple_res.extract());
        godot_print!("some_enum: {:?}", self.some_enum.extract());
        godot_print!(
            "crazy_nested_resource: {:?}",
            self.crazy_nested_resource.extract()
        );
    }
}

/////////// Simple example
#[derive(AsGdRes, Debug, Clone)]
pub struct SimpleData {
    pub name: String,
    pub value: i32,
    pub int_vec: Vec<u8>,
}

/////////// Enum example

#[derive(AsGdRes, Debug, Clone)]
pub struct MoneyData {
    pub value: i32,
}

#[derive(AsGdRes, Debug, Clone)]
pub struct PowerUpData {
    pub cost: i32,
    pub name: String,
}

#[derive(AsGdRes, Debug, Clone)]
pub struct HealData {
    pub hp: i32,
    pub duration: i32,
}

#[derive(AsGdRes, Debug, Clone)]
pub enum Pickup {
    Money(MoneyData),
    PowerUp(PowerUpData),
    Heal(HealData),
}

/////////// Complicated example
#[derive(AsGdRes, Clone, Debug)]
pub struct Complicated {
    // pub simple: SimpleData,
    pub value: i32,
    pub int_vec: Vec<u8>,

    pub nested_enum: Pickup,
    pub nested_enum_option_1: Option<Pickup>,
    pub nested_enum_option_2: Option<Pickup>,
    pub array_enums: Vec<Pickup>,

    pub curve: RustCurve,
    pub curve_option: Option<RustCurve>,
    pub curve_array: Vec<RustCurve>,

    pub path: PackedScenePath,
    pub path_option: Option<PackedScenePath>,
    pub path_array: Vec<PackedScenePath>,

    pub nested_simple: SimpleData,
    pub nested_simple_option: Option<SimpleData>,
    pub array_simple: Vec<SimpleData>,
}

// trait GdWrapper {
//     type WrType<T>;
// }

// impl GdWrapper for u8 {
//     type WrType<T> = u8;
// }

// impl GdWrapper for Pickup {
//     type WrType<T> = Option<T>;
// }

// #[derive(GodotClass)]
// #[class(init, base=Resource)]
// pub struct ComplicatedResource {
//     base: Base<Resource>,

//     #[export]
//     value: <i32 as AsGdRes>::ResType,
//     #[export]
//     int_vec: <Vec<u8> as AsGdRes>::ResType,

//     #[export]
//     nested_enum: <Pickup as AsGdRes>::ResType,
//     #[export]
//     nested_enum_option_1: Option<<Pickup as AsGdRes>::ResType>,
//     #[export]
//     nested_enum_option_2: Option<<Pickup as AsGdRes>::ResType>,
//     #[export]
//     array_enums: Array<<Pickup as AsGdRes>::ResType>,

//     #[export]
//     curve: <RustCurve as AsGdRes>::ResType,

//     #[export]
//     nested_simple: OnEditor<<SimpleData as AsGdRes>::ResType>,
//     #[export]
//     nested_simple_option: Option<<SimpleData as AsGdRes>::ResType>,
//     #[export]
//     array_simple: Array<<SimpleData as AsGdRes>::ResType>,
// }

// impl AsGdRes for Complicated {
//     type ResType = OnEditor<Gd<ComplicatedResource>>;
// }

// impl AsGdRes for Option<Complicated> {
//     type ResType = Option<Gd<ComplicatedResource>>;
// }

// // trait OptionalComplicatedResource: AsGdRes<ResType = Option<Gd<ComplicatedResource>>> {}

// // impl<T: OptionalComplicatedResource> AsGdRes for  {
// //     type ResType = Option<Gd<ComplicatedResource>>;
// // }

// impl ExtractGd for ComplicatedResource {
//     type Extracted = Complicated;

//     fn extract(&self) -> Self::Extracted {
//         Complicated {
//             value: self.value.extract(),
//             int_vec: self.int_vec.extract(),

//             nested_enum: self.nested_enum.extract(),
//             nested_enum_option_1: self.nested_enum_option_1.extract(),
//             nested_enum_option_2: self.nested_enum_option_2.extract(),
//             array_enums: self.array_enums.extract(),
//             nested_simple: self.nested_simple.extract(),
//             nested_simple_option: self.nested_simple_option.extract(),
//             array_simple: self.array_simple.extract(),
//         }
//     }
// }
