use as_gd_res::engine_type_impls::PackedScenePath;
use as_gd_res::engine_type_impls::RustCurve;
use as_gd_res::AsGdRes;
use as_gd_res::ExtractGd;
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

    #[export]
    test_gstring: GString,
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

/////////// Simple enum example

// #[derive(AsGdRes, Debug, Clone)]
// pub enum SimpleEnum {
//     Fire,
//     Water,
//     Earth,
//     Air,
// }

/////////// Enum with data example

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
