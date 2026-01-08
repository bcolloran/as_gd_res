use super::expand_as_gd_res;
use super::{assert_eq, quote, parse_quote};

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
