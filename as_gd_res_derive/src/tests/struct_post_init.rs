use super::expand_as_gd_res;
use super::{assert_eq, quote, parse_quote};

// The `#[as_gd_res(post_init = fn_name)]` attribute
// Resource structs are normally created with `#[class(tool,init,base=Resource)]`, which generates a constructor that initializes the resource with default values.
// However, if the input struct has a `#[as_gd_res(post_init = fn_name)]` attribute, the generated resource struct should not have the `init` attribute (just ``#[class(tool,base=Resource)]`). In this case, we must generate a custom `init` impl for `IResource` that calls `fn_name` after setting the default values, which must be passed through after taking into account any `#[init(...)]` attributes on the fields.
#[test]
fn test_post_init_attr() {
    let input: syn::DeriveInput = parse_quote! {
        #[as_gd_res(post_init = calculate_jump_params)]
        pub struct JumpParams {
            #[export(range = (0.0, 10.0))]
            #[var(get, set = set_height)]
            #[init(val = 3.5)]
            pub height: f32,

            #[export(range = (0.0, 10.0))]
            #[var(get, set = set_time_up)]
            #[init(val = 0.5)]
            pub time_up: f32,

            #[export(range = (0.0, 10.0))]
            #[var(get, set = set_time_down)]
            #[init(val = 0.4)]
            pub time_down: f32,

            #[export(range = (0.0, 1.0))]
            #[init(val = 0.25)]
            pub jump_vel_end_cut: f32,

            #[export(range = (0.0, 3.0))]
            #[init(val = 1.2)]
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
    };
    let expected = quote! {

    impl ::as_gd_res::AsGdRes for JumpParams {
        type ResType = ::godot::prelude::OnEditor<::godot::obj::Gd<JumpParamsResource>>;
    }
    impl ::as_gd_res::AsGdResOpt for JumpParams {
        type GdOption = Option<::godot::obj::Gd<JumpParamsResource>>;
    }
    impl ::as_gd_res::AsGdResArray for JumpParams {
        type GdArray = ::godot::prelude::Array<::godot::obj::Gd<JumpParamsResource>>;
    }
    #[derive(::godot::prelude::GodotClass)]
    // NOTE: `#[as_gd_res(post_init = ...)]` means we do not use the "init" flag in the "#[class(...)]" attribute
    #[class(tool,base = Resource)]
    pub struct JumpParamsResource {
        #[base]
        base: ::godot::obj::Base<::godot::classes::Resource>,

        // NOTE: `#[as_gd_res(post_init = ...)]` means we do NOT pass through `#[init(...)]` attributes
        #[export(range = (0.0,10.0))]
        #[var(get,set = set_height)]
        pub height: <f32 as ::as_gd_res::AsGdRes>::ResType,

        // NOTE: `#[as_gd_res(post_init = ...)]` means we do NOT pass through `#[init(...)]` attributes
        #[export(range = (0.0,10.0))]
        #[var(get,set = set_time_up)]
        pub time_up: <f32 as ::as_gd_res::AsGdRes>::ResType,

        // NOTE: `#[as_gd_res(post_init = ...)]` means we do NOT pass through `#[init(...)]` attributes
        #[export(range = (0.0,10.0))]
        #[var(get,set = set_time_down)]
        pub time_down: <f32 as ::as_gd_res::AsGdRes>::ResType,

        // NOTE: `#[as_gd_res(post_init = ...)]` means we do NOT pass through `#[init(...)]` attributes
        #[export(range = (0.0,1.0))]
        pub jump_vel_end_cut: <f32 as ::as_gd_res::AsGdRes>::ResType,

        // NOTE: `#[as_gd_res(post_init = ...)]` means we do NOT pass through `#[init(...)]` attributes
        #[export(range = (0.0,3.0))]
        #[var(get,set = set_terminal_vel_fall_mult)]

        pub terminal_vel_fall_mult: <f32 as ::as_gd_res::AsGdRes>::ResType,
        #[var]
        pub jump_vel: <f32 as ::as_gd_res::AsGdRes>::ResType,
        #[var]
        pub grav_ascent_acc: <f32 as ::as_gd_res::AsGdRes>::ResType,
        #[var]
        pub grav_falling_acc: <f32 as ::as_gd_res::AsGdRes>::ResType,
        #[var]
        pub jump_landing_vel: <f32 as ::as_gd_res::AsGdRes>::ResType,
        #[var]
        pub terminal_vel: <f32 as ::as_gd_res::AsGdRes>::ResType,
    }
    impl ::as_gd_res::ExtractGd for JumpParamsResource {
        type Extracted = JumpParams;
        fn extract(&self) -> Self::Extracted {
            use ::as_gd_res::ExtractGd;
            Self::Extracted {
                height: self.height.extract(),
                time_up: self.time_up.extract(),
                time_down: self.time_down.extract(),
                jump_vel_end_cut: self.jump_vel_end_cut.extract(),
                terminal_vel_fall_mult: self.terminal_vel_fall_mult.extract(),
                jump_vel: self.jump_vel.extract(),
                grav_ascent_acc: self.grav_ascent_acc.extract(),
                grav_falling_acc: self.grav_falling_acc.extract(),
                jump_landing_vel: self.jump_landing_vel.extract(),
                terminal_vel: self.terminal_vel.extract(),
            }
        }
    }

    // NOTE: `#[as_gd_res(post_init = ...)]` means we need to implement `init`
    // in `IResource` manually (including `#[godot_api]`). This impl sets initial values from the `#[init(...)]`
    // attributes on the fields from the original struct if they exist, or uses the default
    // values otherwise.
    //
    // After contsructing the resource, we call the method `METHOD` from e.g. `#[as_gd_res(post_init = METHOD)]`, (in this case, `calculate_jump_params`) to finalize the resource.
    // It is up to the user to implement this method in the resource struct.
    //
    // TODO: is it possible to get the a useful compile error if the user forgets to implement the method?
    #[godot_api]
    impl ::godot::prelude::IResource for JumpParamsResource {
        fn init(base: ::godot::prelude::Base<::godot::prelude::Resource>) -> Self {
            let mut res = Self {
                base,
                // NOTE: value from the `#[init(val = ...)]` attribute on original struct
                height: 3.5.into(),
                // NOTE: value from the `#[init(val = ...)]` attribute on original struct
                time_up: 0.5.into(),
                // NOTE: value from the `#[init(val = ...)]` attribute on original struct
                time_down: 0.4.into(),
                // NOTE: value from the `#[init(val = ...)]` attribute on original struct
                jump_vel_end_cut: 0.25.into(),
                // NOTE: value from the `#[init(val = ...)]` attribute on original struct
                terminal_vel_fall_mult: 1.2.into(),
                jump_vel: Default::default(),
                grav_ascent_acc: Default::default(),
                grav_falling_acc: Default::default(),
                jump_landing_vel: Default::default(),
                terminal_vel: Default::default(),
            };
            res.calculate_jump_params();
            res
        }
    }
        };

    let actual = expand_as_gd_res(input);

    assert_eq!(actual.to_string(), expected.to_string());
}
