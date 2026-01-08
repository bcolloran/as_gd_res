use super::expand_as_gd_res;
use super::{assert_eq, quote, parse_quote};

// NOTE: Option<{enum types}> is not supported, ::as_gd_res::AsGdRes not impled for that
#[test]
fn test_simple_enum() {
    let input: syn::DeriveInput = parse_quote! {
            #[derive(Default, Clone, Copy, GodotConvert, Var, Export)]
            #[godot(via = GString)]
            pub enum DamageTeam {
                #[default]
                Player,
                Enemy,
                Environment,
            }

    };

    let expected = quote! {
        compile_error!(
                    "`derive(AsGdRes)` only supports enums with single-tuple variants, not unit variants. Did you mean to use `derive(AsGdEnumSimple)`?"
                );
    };

    assert_eq!(expand_as_gd_res(input).to_string(), expected.to_string());
}

// For enums with data variants, we do the following:
// - Create a new trait called `{EnumName}ResourceExtractVariant` that has a method `extract_enum_variant`
// - Create a new type for the enum resource called `{EnumName}Resource`, which aliases `DynGd<Resource, dyn {EnumName}ResourceExtractVariant>`
// - Implement `::as_gd_res::AsGdRes` for the enum, which returns the new resource type
// - Implement `ExtractGd` for the new resource type, which extracts the resource back to the input enum
// - For each enum variant, implement the `{EnumName}ResourceExtractVariant>` trait for the resource corresponding to the type in within the variant. It is up to the user to derive `::as_gd_res::AsGdRes` on the type inside each variant, which will create the resource type for that variant. (For example, if the enum has a variant `Money(MoneyData)`, the user must derive `::as_gd_res::AsGdRes` on `MoneyData` to create the resource type `MoneyDataResource`.). Each impl must be annotated with `#[godot_dyn]` for compatibility with `DynGd`.
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

        pub mod mod_pickup_money{
            use super::*;
            use ::godot::prelude::godot_dyn;
            #[godot_dyn]
            impl PickupResourceExtractVariant for MoneyDataResource {
                fn extract_enum_variant(&self) -> Pickup {
                    Pickup::Money(self.extract())
                }
            }
        }

        pub mod mod_pickup_powerup{
            use super::*;
            use ::godot::prelude::godot_dyn;

            #[godot_dyn]
            impl PickupResourceExtractVariant for PowerUpDataResource {
                fn extract_enum_variant(&self) -> Pickup {
                    Pickup::PowerUp(self.extract())
                }
            }
        }

        pub mod mod_pickup_heal{
            use super::*;
            use ::godot::prelude::godot_dyn;
            #[godot_dyn]
            impl PickupResourceExtractVariant for HealDataResource {
                fn extract_enum_variant(&self) -> Pickup {
                    Pickup::Heal(self.extract())
                }
            }
        }
    };

    assert_eq!(expand_as_gd_res(input).to_string(), expected.to_string());
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
