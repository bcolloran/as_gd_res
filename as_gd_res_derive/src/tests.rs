use super::expand_as_gd_res;
use pretty_assertions::assert_eq;
use quote::quote;
use syn::parse_quote;

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
      impl AsGdRes for SimpleStructParams {
          type ResType = ::godot::obj::Gd<SimpleStructParamsResource>;
      }

      #[derive(::godot::obj::GodotClass)]
      #[class(tool, init, base=::godot::classes::Resource)]
      pub struct SimpleStructParamsResource {
          #[base]
          base: ::godot::obj::Base<::godot::classes::Resource>,
          #[export]
          pub a: <f32 as AsGdRes>::ResType,
          #[export]
          pub b: <f32 as AsGdRes>::ResType,
      }

      impl ExtractGd for SimpleStructParamsResource {
          type Extracted = SimpleStructParams;
          fn extract(&self) -> Self::Extracted {
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

            impl AsGdRes for DropParams2 {
                type ResType = ::godot::obj::Gd<DropParams2Resource>;
            }

            #[derive(::godot::obj::GodotClass)]
            #[class(tool, init, base=::godot::classes::Resource)]
            pub struct DropParams2Resource {
                #[base]
                base: ::godot::obj::Base<::godot::classes::Resource>,
                #[export]
                pub total_value: <f32 as AsGdRes>::ResType,
                #[export]
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

    };
    assert_eq!(actual.to_string(), expected.to_string());
}

#[test]
fn test_attr_pass_through() {
    let input: syn::DeriveInput = parse_quote! {
        pub struct DropParams2 {
          #[export(range = (100.0, 500.0))]
          #[init(val = 200.0)]
          pub total_value: f32,

          #[export(range = (0.0, 5.0))]
          #[init(val = 3.0)]
          pub max_value_per_coin: f32,
          pub coin_scene_1: Option<PackedScenePath>,
          pub coin_scene_2: OnEditorInit<PackedScenePath>,
      }
    };

    let expected = quote! {
      impl AsGdRes for DropParams2 {
          type ResType = ::godot::obj::Gd<DropParams2Resource>;
      }

      #[derive(::godot::obj::GodotClass)]
      #[class(tool, init, base=::godot::classes::Resource)]
      pub struct DropParams2Resource {
          #[base]
          base: ::godot::obj::Base<::godot::classes::Resource>,
          #[export(range = (100.0, 500.0))]
          #[init(val = 200.0)]
          pub total_value: <f32 as AsGdRes>::ResType,
          #[export(range = (0.0, 5.0))]
          #[init(val = 3.0)]
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

    };

    assert_eq!(expand_as_gd_res(input).to_string(), expected.to_string());
}

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
            impl AsGdRes for DamageTeam {
                type ResType = DamageTeam;
            }

            impl ExtractGd for DamageTeam {
                type Extracted = DamageTeam;
                fn extract(&self) -> Self::Extracted {
                    self.clone()
                }
            }

    };

    assert_eq!(expand_as_gd_res(input).to_string(), expected.to_string());
}

#[test]
fn test_enum_with_data_variants() {
    let input: syn::DeriveInput = parse_quote! {
            pub enum BrainParams {
                    Roomba(RoombaBrainParams),
                    Tank(TankBrainParams),
                }
    };

    let expected = quote! {
            pub trait BrainParamsDynEnumResource {
                fn extract_enum_data(&self) -> BrainParams;
            }
            impl BrainParamsDynEnumResource for RoombaBrainParamsResource {
                fn extract_enum_data(&self) -> BrainParams {
                    BrainParams::Roomba(self.extract())
                }
            }
            impl BrainParamsDynEnumResource for TankBrainParamsResource {
                fn extract_enum_data(&self) -> BrainParams {
                    BrainParams::Tank(self.extract())
                }
            }

            impl AsGdRes for BrainParams {
                type ResType = ::godot::obj::DynGd<::godot::classes::Resource, dyn BrainParamsDynEnumResource>;
            }

            impl ExtractGd for ::godot::obj::DynGd<::godot::classes::Resource, dyn BrainParamsDynEnumResource> {
                type Extracted = BrainParams;
                fn extract(&self) -> Self::Extracted {
                    self.dyn_bind().extract_enum_data()
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

        impl AsGdRes for EnemyParams {
            type ResType = ::godot::obj::Gd<EnemyParamsResource>;
        }

        #[derive(::godot::obj::GodotClass)]
        #[class(tool, init, base=::godot::classes::Resource)]
        pub struct EnemyParamsResource {
            #[base]
            base: ::godot::obj::Base<::godot::classes::Resource>,

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
