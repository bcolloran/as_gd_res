use as_gd_res::AsGdRes;
use as_gd_res::ExtractGd;
use godot::builtin::GString;
use godot::obj::Base;
use godot::obj::DynGd;
use godot::obj::Gd;
use godot::obj::GodotClass;
use godot::obj::OnEditor;

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
    test_simple_res: OnEditor<DynGd<SimpleDataResource>>,
}

#[godot_api]
impl INode for TestNode {
    // Called when the node is ready in the scene tree.
    fn ready(&mut self) {
        godot_print!("TestNode is ready!");
    }
}

#[derive(AsGdRes)]
pub struct SimpleData {
    pub name: String,
    pub value: i32,
    pub data: Vec<u8>,
}

#[derive(AsGdRes)]
pub struct MoneyData {
    pub value: i32,
}

#[derive(AsGdRes)]
pub struct PowerUpData {
    pub cost: i32,
    pub name: String,
}

#[derive(AsGdRes)]
pub struct HealData {
    pub hp: i32,
    pub duration: i32,
}

#[derive(AsGdRes)]
pub enum Pickup {
    Money(MoneyData),
    PowerUp(PowerUpData),
    Heal(HealData),
}
