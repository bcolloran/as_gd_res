use super::expand_as_gd_res;
use pretty_assertions::assert_eq;
use quote::quote;
use syn::parse_quote;

// # STRUCTS
// For each field in a composite struct, if it includes any `#[export(...)]` or `#[init(...)]` attributes,
// the generated `ResType` should include those attributes on the generated struct.
// If the field has no attributes, we must add the `#[export]` attribute to the generated struct.
// ## Notes:
// - The derive macro should not support generics, so the input struct should not have any generic parameters.
//
// ## The `#[as_gd_res(post_init = fn_name)]` attribute
// Resource structs are normally created with `#[class(tool,init,base=Resource)]`, which generates a constructor that initializes the resource with default values.
// However, if the input struct has a `#[as_gd_res(post_init = fn_name)]` attribute, the generated resource struct should not have the `init` attribute (just ``#[class(tool,base=Resource)]`). In this case, we must generate a custom `init` impl for `IResource` that calls `fn_name` after setting the default values, which must be passed through after taking into account any `#[init(...)]` attributes on the fields.
//
// # ENUMS
// `#[derive(::as_gd_res::AsGdRes)]` does not support enums, use `#[derive(::as_gd_res::AsSimpleGdEnum)]` instead.
//
// In any other case, the macro should emit an error saying that these conditions have not been met
//
// # NOTES:
// There are limitations upstream in *godot-rust* (or really: in Godot itself) that prevent the representation of certain types. You'll need work arounds in at least these cases:
// - `Option<{enum types}>`: If you want an "optional" enum, include a `None` variant in the enum itself, and set that as the default value.
// - `Array<{enum types}>` are also not supported

#[test]
fn test_simple() {
    let input: syn::DeriveInput = parse_quote! {
        pub struct SimpleStructParams {
            a: f32,
            b: f32,
        }
    };
    let actual = expand_as_gd_res(input);
    let expected = quote! {
      impl ::as_gd_res::AsGdRes for SimpleStructParams {
          type ResType = ::godot::prelude::OnEditor<::godot::obj::Gd<SimpleStructParamsResource>>;
      }

      impl ::as_gd_res::AsGdResOpt for SimpleStructParams {
          type GdOption = Option<::godot::obj::Gd<SimpleStructParamsResource>>;
      }

      impl ::as_gd_res::AsGdResArray for SimpleStructParams {
          type GdArray = ::godot::prelude::Array<::godot::obj::Gd<SimpleStructParamsResource>>;
      }

      #[derive(::godot::prelude::GodotClass)]
      #[class(tool,init,base=Resource)]
      pub struct SimpleStructParamsResource {
          #[base]
          base: ::godot::obj::Base<::godot::classes::Resource>,
          #[export]
          pub a: <f32 as ::as_gd_res::AsGdRes>::ResType,
          #[export]
          pub b: <f32 as ::as_gd_res::AsGdRes>::ResType,
      }

      impl ::as_gd_res::ExtractGd for SimpleStructParamsResource {
          type Extracted = SimpleStructParams;
          fn extract(&self) -> Self::Extracted {
              use ::as_gd_res::ExtractGd;
              Self::Extracted {
                  a: self.a.extract(),
                  b: self.b.extract(),
              }
          }
      }
    };
    assert_eq!(actual.to_string(), expected.to_string());
}

#[test]
fn test_2() {
    let input: syn::DeriveInput = parse_quote! {
        pub struct DropParams2 {
            pub total_value: f32,
            pub max_value_per_coin: f32,
            pub coin_scene_1: Option<PackedScenePath>,
            pub coin_scene_2: OnEditorInit<PackedScenePath>,
        }
    };
    let actual = expand_as_gd_res(input);
    let expected = quote! {

            impl ::as_gd_res::AsGdRes for DropParams2 {
                type ResType = ::godot::prelude::OnEditor<::godot::obj::Gd<DropParams2Resource>>;
            }

            impl ::as_gd_res::AsGdResOpt for DropParams2 {
                type GdOption = Option<::godot::obj::Gd<DropParams2Resource>>;
            }

            impl ::as_gd_res::AsGdResArray for DropParams2 {
                type GdArray = ::godot::prelude::Array<::godot::obj::Gd<DropParams2Resource>>;
            }

            #[derive(::godot::prelude::GodotClass)]
            #[class(tool,init,base=Resource)]

            pub struct DropParams2Resource {
                #[base]
                base: ::godot::obj::Base<::godot::classes::Resource>,
                #[export]
                pub total_value: <f32 as ::as_gd_res::AsGdRes>::ResType,
                #[export]
                pub max_value_per_coin: <f32 as ::as_gd_res::AsGdRes>::ResType,
                #[export]
                pub coin_scene_1: <Option<PackedScenePath> as ::as_gd_res::AsGdRes>::ResType,
                #[export]
                pub coin_scene_2: <OnEditorInit<PackedScenePath> as ::as_gd_res::AsGdRes>::ResType,
            }

            impl ::as_gd_res::ExtractGd for DropParams2Resource {
                type Extracted = DropParams2;
                fn extract(&self) -> Self::Extracted {
                    use ::as_gd_res::ExtractGd;
                    Self::Extracted {
                        total_value: self.total_value.extract(),
                        max_value_per_coin: self.max_value_per_coin.extract(),
                        coin_scene_1: self.coin_scene_1.extract(),
                        coin_scene_2: self.coin_scene_2.extract(),
                    }
                }
            }

    };
    assert_eq!(actual.to_string(), expected.to_string());
}

/// Spec for attribute pass-through:
/// - If the field has any of the following attributes, they should be passed through to the generated struct.
///     - #[var(...)]
///     - #[export(...)]
///     - #[init(...)]
/// - If the field has no attributes, we must add the `#[export]` attribute to the generated struct.
#[test]
fn test_attr_pass_through() {
    let input: syn::DeriveInput = parse_quote! {
        pub struct DropParams2 {
          #[export(range = (100.0, 500.0))]
          #[init(val = 200.0)]
          pub total_value: f32,

          #[export(range = (0.0, 5.0))]
          #[init(val = 3.0)]
          #[var(get, set = set_max_value_per_coin)]
          pub max_value_per_coin: f32,

          pub coin_scene_1: Option<PackedScenePath>,
          pub coin_scene_2: OnEditorInit<PackedScenePath>,

          #[var]
          pub non_exported_field: u32,
      }
    };

    let expected = quote! {
        impl ::as_gd_res::AsGdRes for DropParams2 {
            type ResType = ::godot::prelude::OnEditor<::godot::obj::Gd<DropParams2Resource>>;
        }

        impl ::as_gd_res::AsGdResOpt for DropParams2 {
            type GdOption = Option<::godot::obj::Gd<DropParams2Resource>>;
        }

        impl ::as_gd_res::AsGdResArray for DropParams2 {
            type GdArray = ::godot::prelude::Array<::godot::obj::Gd<DropParams2Resource>>;
        }

      #[derive(::godot::prelude::GodotClass)]
      #[class(tool,init,base=Resource)]
      pub struct DropParams2Resource {
          #[base]
          base: ::godot::obj::Base<::godot::classes::Resource>,
          #[export(range = (100.0, 500.0))]
          #[init(val = 200.0)]
          pub total_value: <f32 as ::as_gd_res::AsGdRes>::ResType,

          #[export(range = (0.0, 5.0))]
          #[init(val = 3.0)]
          #[var(get, set = set_max_value_per_coin)]
          pub max_value_per_coin: <f32 as ::as_gd_res::AsGdRes>::ResType,

          #[export]
          pub coin_scene_1: <Option<PackedScenePath> as ::as_gd_res::AsGdRes>::ResType,

          #[export]
          pub coin_scene_2: <OnEditorInit<PackedScenePath> as ::as_gd_res::AsGdRes>::ResType,

          #[var]
          pub non_exported_field: <u32 as ::as_gd_res::AsGdRes>::ResType,
      }

      impl ::as_gd_res::ExtractGd for DropParams2Resource {
          type Extracted = DropParams2;
          fn extract(&self) -> Self::Extracted {
              use ::as_gd_res::ExtractGd;
              Self::Extracted {
                  total_value: self.total_value.extract(),
                  max_value_per_coin: self.max_value_per_coin.extract(),
                  coin_scene_1: self.coin_scene_1.extract(),
                  coin_scene_2: self.coin_scene_2.extract(),
                  non_exported_field: self.non_exported_field.extract(),
              }
          }
      }

    };

    assert_eq!(expand_as_gd_res(input).to_string(), expected.to_string());
}

// NOTE: Option<{enum types}> is not supported, ::as_gd_res::AsGdRes not impled for that
#[test]
fn test_simple_enum() {
    let input: syn::DeriveInput = parse_quote! {
            #[derive(Default, Clone, Copy, GodotConvert, Var, Export)]
            #[godot(via = ::godot::builtin::GString)]
            pub enum DamageTeam {
                #[default]
                Player,
                Enemy,
                Environment,
            }

    };

    let expected = quote! {
        compile_error!(
                    "`derive(AsGdRes)` only supports enums with single-tuple variants, not unit variants. Did you mean to use `derive(AsSimpleGdEnum)`?"
                );
    };

    assert_eq!(expand_as_gd_res(input).to_string(), expected.to_string());
}

/// For enums with data variants, we do the following:
/// - Create a new trait called `{EnumName}ResourceExtractVariant` that has a method `extract_enum_variant`
/// - Create a new type for the enum resource called `{EnumName}Resource`, which aliases `DynGd<Resource, dyn {EnumName}ResourceExtractVariant>`
/// - Implement `::as_gd_res::AsGdRes` for the enum, which returns the new resource type
/// - Implement `ExtractGd` for the new resource type, which extracts the resource back to the input enum
/// - For each enum variant, implement the `{EnumName}ResourceExtractVariant>` trait for the resource corresponding to the type in within the variant. It is up to the user to derive `::as_gd_res::AsGdRes` on the type inside each variant, which will create the resource type for that variant. (For example, if the enum has a variant `Money(MoneyData)`, the user must derive `::as_gd_res::AsGdRes` on `MoneyData` to create the resource type `MoneyDataResource`.). Each impl must be annotated with `#[godot_dyn]` for compatibility with `DynGd`.
///
/// Note that having
#[test]
fn test_enum_with_data_variants() {
    let input: syn::DeriveInput = parse_quote! {
        pub enum Pickup {
            Money(MoneyData),
            PowerUp(PowerUpData),
            Heal(HealData),
        }
    };

    let expected = quote! {
        pub trait PickupResourceExtractVariant {
            fn extract_enum_variant(&self) -> Pickup;
        }

        type PickupResource =
            ::godot::obj::DynGd<::godot::classes::Resource, dyn PickupResourceExtractVariant>;

        impl ::as_gd_res::AsGdRes for Pickup {
            type ResType = ::godot::prelude::OnEditor<PickupResource>;
        }
        impl ::as_gd_res::AsGdResOpt for Pickup {
            type GdOption = Option<PickupResource>;
        }
        impl ::as_gd_res::AsGdResArray for Pickup {
            type GdArray = ::godot::prelude::Array<PickupResource>;
        }

        impl ::as_gd_res::ExtractGd for dyn PickupResourceExtractVariant {
            type Extracted = Pickup;
            fn extract(&self) -> Self::Extracted {
                self.extract_enum_variant()
            }
        }

        #[godot_dyn]
        impl PickupResourceExtractVariant for MoneyDataResource {
            fn extract_enum_variant(&self) -> Pickup {
                Pickup::Money(self.extract())
            }
        }
        #[godot_dyn]
        impl PickupResourceExtractVariant for PowerUpDataResource {
            fn extract_enum_variant(&self) -> Pickup {
                Pickup::PowerUp(self.extract())
            }
        }

        #[godot_dyn]
        impl PickupResourceExtractVariant for HealDataResource {
            fn extract_enum_variant(&self) -> Pickup {
                Pickup::Heal(self.extract())
            }
        }
    };

    assert_eq!(expand_as_gd_res(input).to_string(), expected.to_string());
}

#[test]
fn test_complex_nested_struct() {
    let input: syn::DeriveInput = parse_quote! {
      pub struct EnemyParams {
          pub brain_params_required: OnEditorInit<BrainParams>,
          pub brain_params_optional: Option<BrainParams>,
          pub brains_vec: Vec<BrainParams>,
          pub drop_params: Option<DropParams2>,
          pub damage_team: DamageTeam,
      }
    };

    let actual = expand_as_gd_res(input);
    let expected = quote! {

        impl ::as_gd_res::AsGdRes for EnemyParams {
            type ResType = ::godot::prelude::OnEditor<::godot::obj::Gd<EnemyParamsResource>>;
        }

        impl ::as_gd_res::AsGdResOpt for EnemyParams {
            type GdOption = Option<::godot::obj::Gd<EnemyParamsResource>>;
        }

        impl ::as_gd_res::AsGdResArray for EnemyParams {
            type GdArray = ::godot::prelude::Array<::godot::obj::Gd<EnemyParamsResource>>;
        }


        #[derive(::godot::prelude::GodotClass)]
        #[class(tool,init,base=Resource)]

        pub struct EnemyParamsResource {
            #[base]
            base: ::godot::obj::Base<::godot::classes::Resource>,

            #[export]
            pub brain_params_required: <OnEditorInit<BrainParams> as ::as_gd_res::AsGdRes>::ResType,
            #[export]
            pub brain_params_optional: <Option<BrainParams> as ::as_gd_res::AsGdRes>::ResType,
            #[export]
            pub brains_vec: <Vec<BrainParams> as ::as_gd_res::AsGdRes>::ResType,

            #[export]
            pub drop_params: <Option<DropParams2> as ::as_gd_res::AsGdRes>::ResType,
            #[export]
            pub damage_team: <DamageTeam as ::as_gd_res::AsGdRes>::ResType,
        }

        impl ::as_gd_res::ExtractGd for EnemyParamsResource {
            type Extracted = EnemyParams;
            fn extract(&self) -> Self::Extracted {
                use ::as_gd_res::ExtractGd;
                Self::Extracted {
                    brain_params_required: self.brain_params_required.extract(),
                    brain_params_optional: self.brain_params_optional.extract(),
                    brains_vec: self.brains_vec.extract(),
                    drop_params: self.drop_params.extract(),
                    damage_team: self.damage_team.extract(),
                }
            }
        }

    };

    assert_eq!(actual.to_string(), expected.to_string());
}
#[test]
fn test_enum_mixed_variants_error() {
    let input: syn::DeriveInput = parse_quote! {
        pub enum Mixed {
            Unit,
            Tuple(u32),
            Struct { x: i32 }
        }
    };
    let expected = quote! {
        compile_error!("`derive(AsGdRes)` only supports unit enums or single-tuple enums. Unsupported variants: Struct");
    };
    assert_eq!(expand_as_gd_res(input).to_string(), expected.to_string());
}

#[test]
fn test_union_error() {
    let input: syn::DeriveInput = parse_quote! {
        pub union Foo {
            a: u32,
            b: f32,
        }
    };
    let expected = quote! {
        compile_error!("`derive(AsGdRes)` only supports structs with named fields, enums with unit variants, or enums with single-tuple variants");
    };
    assert_eq!(expand_as_gd_res(input).to_string(), expected.to_string());
}

#[test]
fn test_tuple_struct_error() {
    let input: syn::DeriveInput = parse_quote! {
        pub struct Foo(u32);
    };
    let expected = quote! {
        compile_error!("`derive(AsGdRes)` only supports structs with named fields");
    };
    assert_eq!(expand_as_gd_res(input).to_string(), expected.to_string());
}

#[test]
fn test_generic_struct_error() {
    let input: syn::DeriveInput = parse_quote! {
        pub struct Bar<T> { val: T }
    };
    let expected = quote! {
        compile_error!("`derive(AsGdRes)` does not support generics");
    };
    assert_eq!(expand_as_gd_res(input).to_string(), expected.to_string());
}

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
    impl IResource for JumpParamsResource {
        fn init(base: Base<Resource>) -> Self {
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
