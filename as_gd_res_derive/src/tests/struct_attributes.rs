use super::expand_as_gd_res;
use super::{assert_eq, quote, parse_quote};

// Spec for attribute pass-through:
// - If the field has any of the following attributes, they should be passed through to the generated struct.
//     - #[var(...)]
//     - #[export(...)]
//     - #[init(...)]
// - If the field has no attributes, we must add the `#[export]` attribute to the generated struct.
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
