use as_gd_res::engine_type_impls::PackedScenePath;
use as_gd_res::engine_type_impls::RustCurve;
use as_gd_res::AsGdRes;
use as_gd_res::AsSimpleGdEnum;
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
    test_simple_res: <SimpleData as AsGdRes>::ResType,
    #[export]
    test_simple_enum: <SimpleEnum as AsGdRes>::ResType,
    #[export]
    enum_with_data: <Pickup as AsGdRes>::ResType,
    #[export]
    crazy_nested_resource: <Complicated as AsGdRes>::ResType,
    #[export]
    calculated_resource_default_in_editor: <JumpParams as AsGdRes>::ResType,
    #[export]
    calculated_resource_changed_in_editor: <JumpParams as AsGdRes>::ResType,
}

#[godot_api]
impl INode for TestNode {
    // Called when the node is ready in the scene tree.
    fn ready(&mut self) {
        godot_print!("--- Resource Extract Test ---");
        godot_print!("test_simple_res: {:?}", self.test_simple_res.extract());
        godot_print!("enum_with_data: {:?}", self.enum_with_data.extract());
        godot_print!(
            "crazy_nested_resource: {:?}",
            self.crazy_nested_resource.extract()
        );
        godot_print!(
            "calculated_resource_default_in_editor: {:?}",
            self.calculated_resource_default_in_editor.extract()
        );
        godot_print!(
            "calculated_resource_changed_in_editor: {:?}",
            self.calculated_resource_changed_in_editor.extract()
        );

        self.base().get_tree().map(|mut tree| tree.quit());
    }
}

/////////// Simple struct
#[derive(AsGdRes, Debug, Clone)]
pub struct SimpleData {
    pub name: String,
    pub value: i32,
    pub int_vec: Vec<u8>,
}

/////////// Simple enum
// NOTE: godot-rust currently doesn't support wrapping enums in Array<_> or Option<_>. (If you want an enum to be optional, you can include a 'None' variant)

#[derive(AsSimpleGdEnum, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimpleEnum {
    #[default]
    Fire,
    Water,
    Earth,
    Air,
}

/////////// Enum with data

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

/////////// Complicated struct
#[derive(AsGdRes, Clone, Debug)]
pub struct Complicated {
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

    pub simple_enum: SimpleEnum,
}

#[derive(AsGdRes, Clone, Debug)]
#[as_gd_res(post_init = calculate_jump_params)]
pub struct JumpParams {
    #[export(range = (0.0, 10.0))]
    #[var(get, set = set_height)]
    #[init(val = 5.0)]
    pub height: f32,

    #[export(range = (0.0, 10.0))]
    #[var(get, set = set_time_up)]
    #[init(val = 1.0)]
    pub time_up: f32,

    #[export(range = (0.0, 10.0))]
    #[var(get, set = set_time_down)]
    #[init(val = 0.5)]
    pub time_down: f32,

    #[export(range = (0.0, 1.0))]
    #[init(val = 0.25)]
    pub jump_vel_end_cut: f32,

    #[export(range = (0.0, 3.0))]
    #[init(val = 1.5)]
    #[var(get, set = set_terminal_vel_fall_mult)]
    pub terminal_vel_fall_mult: f32,

    // Non-exported variables that are calculated based on the above parameters.
    #[var]
    pub jump_vel: f32,
    #[var]
    pub grav_ascent_acc: f32,
    #[var]
    pub grav_falling_acc: f32,
    #[var]
    pub jump_landing_vel: f32,
    #[var]
    pub terminal_vel: f32,
}

// // Recursive expansion of AsGdRes macro
// // =====================================

// #[derive(Clone, Debug)]
// pub struct JumpParams {
//     pub height: f32,

//     pub time_up: f32,

//     pub time_down: f32,

//     pub jump_vel_end_cut: f32,

//     pub terminal_vel_fall_mult: f32,

//     pub jump_vel: f32,

//     pub grav_ascent_acc: f32,

//     pub grav_falling_acc: f32,

//     pub jump_landing_vel: f32,

//     pub terminal_vel: f32,
// }

// impl ::as_gd_res::AsGdRes for JumpParams {
//     type ResType = ::godot::prelude::OnEditor<::godot::obj::Gd<JumpParamsResource>>;
// }
// impl ::as_gd_res::AsGdResOpt for JumpParams {
//     type GdOption = Option<::godot::obj::Gd<JumpParamsResource>>;
// }
// impl ::as_gd_res::AsGdResArray for JumpParams {
//     type GdArray = ::godot::prelude::Array<::godot::obj::Gd<JumpParamsResource>>;
// }
// #[derive(::godot::prelude::GodotClass)]
// #[class(tool,base = Resource)]
// pub struct JumpParamsResource {
//     #[base]
//     base: ::godot::obj::Base<::godot::classes::Resource>,
//     #[export(range = (0.0,10.0))]
//     #[var(get,set = set_height)]
//     // #[init(val = 3.5)]
//     pub height: <f32 as ::as_gd_res::AsGdRes>::ResType,
//     #[export(range = (0.0,10.0))]
//     #[var(get,set = set_time_up)]
//     // #[init(val = 0.5)]
//     pub time_up: <f32 as ::as_gd_res::AsGdRes>::ResType,
//     #[export(range = (0.0,10.0))]
//     #[var(get,set = set_time_down)]
//     // #[init(val = 0.4)]
//     pub time_down: <f32 as ::as_gd_res::AsGdRes>::ResType,
//     #[export(range = (0.0,1.0))]
//     // #[init(val = 0.25)]
//     pub jump_vel_end_cut: <f32 as ::as_gd_res::AsGdRes>::ResType,
//     #[export(range = (0.0,3.0))]
//     // #[init(val = 1.2)]
//     #[var(get,set = set_terminal_vel_fall_mult)]
//     pub terminal_vel_fall_mult: <f32 as ::as_gd_res::AsGdRes>::ResType,
//     #[var]
//     pub jump_vel: <f32 as ::as_gd_res::AsGdRes>::ResType,
//     #[var]
//     pub grav_ascent_acc: <f32 as ::as_gd_res::AsGdRes>::ResType,
//     #[var]
//     pub grav_falling_acc: <f32 as ::as_gd_res::AsGdRes>::ResType,
//     #[var]
//     pub jump_landing_vel: <f32 as ::as_gd_res::AsGdRes>::ResType,
//     #[var]
//     pub terminal_vel: <f32 as ::as_gd_res::AsGdRes>::ResType,
// }
// impl ::as_gd_res::ExtractGd for JumpParamsResource {
//     type Extracted = JumpParams;
//     fn extract(&self) -> Self::Extracted {
//         Self::Extracted {
//             height: self.height.extract(),
//             time_up: self.time_up.extract(),
//             time_down: self.time_down.extract(),
//             jump_vel_end_cut: self.jump_vel_end_cut.extract(),
//             terminal_vel_fall_mult: self.terminal_vel_fall_mult.extract(),
//             jump_vel: self.jump_vel.extract(),
//             grav_ascent_acc: self.grav_ascent_acc.extract(),
//             grav_falling_acc: self.grav_falling_acc.extract(),
//             jump_landing_vel: self.jump_landing_vel.extract(),
//             terminal_vel: self.terminal_vel.extract(),
//         }
//     }
// }

// #[godot_api]
// impl IResource for JumpParamsResource {
//     fn init(base: Base<Resource>) -> Self {
//         let mut res = Self {
//             base,
//             height: 3.5.into(),
//             time_up: 0.5.into(),
//             time_down: 0.4.into(),
//             jump_vel_end_cut: 0.25.into(),
//             terminal_vel_fall_mult: 1.2.into(),
//             jump_vel: Default::default(),
//             grav_ascent_acc: Default::default(),
//             grav_falling_acc: Default::default(),
//             jump_landing_vel: Default::default(),
//             terminal_vel: Default::default(),
//         };
//         res.calculate_jump_params();
//         res
//     }
// }

#[godot_api]
impl JumpParamsResource {
    #[func]
    pub fn set_height(&mut self, value: f32) {
        self.height = value;
        self.calculate_jump_params();
    }
    #[func]
    pub fn set_time_up(&mut self, value: f32) {
        self.time_up = value;
        self.calculate_jump_params();
    }
    #[func]
    pub fn set_time_down(&mut self, value: f32) {
        self.time_down = value;
        self.calculate_jump_params();
    }
    #[func]
    pub fn set_terminal_vel_fall_mult(&mut self, value: f32) {
        self.terminal_vel_fall_mult = value;
        self.calculate_jump_params();
    }
    #[func]
    pub fn calculate_jump_params(&mut self) {
        self.jump_vel = self.height * 2.0 / self.time_up;
        self.grav_ascent_acc = -2.0 * self.height / (self.time_up * self.time_up);
        self.grav_falling_acc = -2.0 * self.height / (self.time_down * self.time_down);
        self.jump_landing_vel = self.grav_falling_acc * self.time_down;
        self.terminal_vel = self.jump_landing_vel * self.terminal_vel_fall_mult;
    }
}
