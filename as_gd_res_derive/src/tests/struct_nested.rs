use super::expand_as_gd_res;
use super::{assert_eq, quote, parse_quote};

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
