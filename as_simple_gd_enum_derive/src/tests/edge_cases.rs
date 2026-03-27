use super::expand_as_gd_res;
use super::{assert_eq, parse_quote, quote};

/// Test empty unit enum (edge case - all_unit is vacuously true for empty iterator)
#[test]
fn test_empty_enum() {
    let input: syn::DeriveInput = parse_quote! {
        pub enum EmptyEnum {}
    };

    let actual = expand_as_gd_res(input);
    // An empty enum has no variants, so all are vacuously unit
    let expected = quote! {
        pub use mod_emptyenumasgdenum::*;
        mod mod_emptyenumasgdenum {
            use super::EmptyEnum;
            use ::godot::prelude::GString;

            #[derive(::godot::prelude::GodotConvert, ::godot::prelude::Var, ::godot::prelude::Export, Clone, Copy, Debug, PartialEq, Eq)]
            #[godot(via = GString)]
            pub enum EmptyEnumAsGdEnum {
            }

            impl ::as_gd_res::AsGdEnumSimple for EmptyEnum {
                type GdEnumType = EmptyEnumAsGdEnum;
            }

            impl ::as_gd_res::ExtractGd for EmptyEnumAsGdEnum {
                type Extracted = EmptyEnum;
                fn extract(&self) -> Self::Extracted {
                    (*self).into()
                }
            }

            impl From<EmptyEnum> for EmptyEnumAsGdEnum {
                fn from(value: EmptyEnum) -> EmptyEnumAsGdEnum {
                    match value {
                    }
                }
            }
            impl From<EmptyEnumAsGdEnum> for EmptyEnum {
                fn from(value: EmptyEnumAsGdEnum) -> EmptyEnum {
                    match value {
                    }
                }
            }
            impl Default for EmptyEnumAsGdEnum {
                fn default() -> Self {
                    EmptyEnum::default().into()
                }
            }
        }
    };

    assert_eq!(actual.to_string(), expected.to_string());
}

/// Test single variant enum
#[test]
fn test_single_variant_enum() {
    let input: syn::DeriveInput = parse_quote! {
        pub enum SingleVariant {
            Only,
        }
    };

    let expected = quote! {
        pub use mod_singlevariantasgdenum::*;
        mod mod_singlevariantasgdenum {
            use super::SingleVariant;
            use ::godot::prelude::GString;

            #[derive(::godot::prelude::GodotConvert, ::godot::prelude::Var, ::godot::prelude::Export, Clone, Copy, Debug, PartialEq, Eq)]
            #[godot(via = GString)]
            pub enum SingleVariantAsGdEnum {
                Only,
            }

            impl ::as_gd_res::AsGdEnumSimple for SingleVariant {
                type GdEnumType = SingleVariantAsGdEnum;
            }

            impl ::as_gd_res::ExtractGd for SingleVariantAsGdEnum {
                type Extracted = SingleVariant;
                fn extract(&self) -> Self::Extracted {
                    (*self).into()
                }
            }

            impl From<SingleVariant> for SingleVariantAsGdEnum {
                fn from(value: SingleVariant) -> SingleVariantAsGdEnum {
                    match value {
                        SingleVariant::Only => SingleVariantAsGdEnum::Only,
                    }
                }
            }
            impl From<SingleVariantAsGdEnum> for SingleVariant {
                fn from(value: SingleVariantAsGdEnum) -> SingleVariant {
                    match value {
                        SingleVariantAsGdEnum::Only => SingleVariant::Only,
                    }
                }
            }
            impl Default for SingleVariantAsGdEnum {
                fn default() -> Self {
                    SingleVariant::default().into()
                }
            }
        }
    };

    assert_eq!(expand_as_gd_res(input).to_string(), expected.to_string());
}

/// Test enum with many variants to ensure proper handling
#[test]
fn test_many_variants_enum() {
    let input: syn::DeriveInput = parse_quote! {
        pub enum ManyVariants {
            A,
            B,
            C,
            D,
            E,
        }
    };

    let expected = quote! {
        pub use mod_manyvariantsasgdenum::*;
        mod mod_manyvariantsasgdenum {
            use super::ManyVariants;
            use ::godot::prelude::GString;

            #[derive(::godot::prelude::GodotConvert, ::godot::prelude::Var, ::godot::prelude::Export, Clone, Copy, Debug, PartialEq, Eq)]
            #[godot(via = GString)]
            pub enum ManyVariantsAsGdEnum {
                A, B, C, D, E,
            }

            impl ::as_gd_res::AsGdEnumSimple for ManyVariants {
                type GdEnumType = ManyVariantsAsGdEnum;
            }

            impl ::as_gd_res::ExtractGd for ManyVariantsAsGdEnum {
                type Extracted = ManyVariants;
                fn extract(&self) -> Self::Extracted {
                    (*self).into()
                }
            }

            impl From<ManyVariants> for ManyVariantsAsGdEnum {
                fn from(value: ManyVariants) -> ManyVariantsAsGdEnum {
                    match value {
                        ManyVariants::A => ManyVariantsAsGdEnum::A,
                        ManyVariants::B => ManyVariantsAsGdEnum::B,
                        ManyVariants::C => ManyVariantsAsGdEnum::C,
                        ManyVariants::D => ManyVariantsAsGdEnum::D,
                        ManyVariants::E => ManyVariantsAsGdEnum::E,
                    }
                }
            }
            impl From<ManyVariantsAsGdEnum> for ManyVariants {
                fn from(value: ManyVariantsAsGdEnum) -> ManyVariants {
                    match value {
                        ManyVariantsAsGdEnum::A => ManyVariants::A,
                        ManyVariantsAsGdEnum::B => ManyVariants::B,
                        ManyVariantsAsGdEnum::C => ManyVariants::C,
                        ManyVariantsAsGdEnum::D => ManyVariants::D,
                        ManyVariantsAsGdEnum::E => ManyVariants::E,
                    }
                }
            }
            impl Default for ManyVariantsAsGdEnum {
                fn default() -> Self {
                    ManyVariants::default().into()
                }
            }
        }
    };

    assert_eq!(expand_as_gd_res(input).to_string(), expected.to_string());
}

/// Test enum with private visibility
#[test]
fn test_private_enum() {
    let input: syn::DeriveInput = parse_quote! {
        enum PrivateEnum {
            A,
            B,
        }
    };

    let expected = quote! {
        pub use mod_privateenumasgdenum::*;
        mod mod_privateenumasgdenum {
            use super::PrivateEnum;
            use ::godot::prelude::GString;

            #[derive(::godot::prelude::GodotConvert, ::godot::prelude::Var, ::godot::prelude::Export, Clone, Copy, Debug, PartialEq, Eq)]
            #[godot(via = GString)]
            pub enum PrivateEnumAsGdEnum {
                A, B,
            }

            impl ::as_gd_res::AsGdEnumSimple for PrivateEnum {
                type GdEnumType = PrivateEnumAsGdEnum;
            }

            impl ::as_gd_res::ExtractGd for PrivateEnumAsGdEnum {
                type Extracted = PrivateEnum;
                fn extract(&self) -> Self::Extracted {
                    (*self).into()
                }
            }

            impl From<PrivateEnum> for PrivateEnumAsGdEnum {
                fn from(value: PrivateEnum) -> PrivateEnumAsGdEnum {
                    match value {
                        PrivateEnum::A => PrivateEnumAsGdEnum::A,
                        PrivateEnum::B => PrivateEnumAsGdEnum::B,
                    }
                }
            }
            impl From<PrivateEnumAsGdEnum> for PrivateEnum {
                fn from(value: PrivateEnumAsGdEnum) -> PrivateEnum {
                    match value {
                        PrivateEnumAsGdEnum::A => PrivateEnum::A,
                        PrivateEnumAsGdEnum::B => PrivateEnum::B,
                    }
                }
            }
            impl Default for PrivateEnumAsGdEnum {
                fn default() -> Self {
                    PrivateEnum::default().into()
                }
            }
        }
    };

    assert_eq!(expand_as_gd_res(input).to_string(), expected.to_string());
}

/// Test enum with struct variant (named fields) - should error
#[test]
fn test_struct_variant_error() {
    let input: syn::DeriveInput = parse_quote! {
        pub enum WithStruct {
            A,
            B { x: i32, y: i32 },
        }
    };

    let expected = quote! {
        compile_error!("`derive(AsGdEnumSimple)` only supports unit enums. Unsupported variants: B{x: i32, y: i32}.\nDid you mean to derive `AsGdRes`?");
    };

    assert_eq!(expand_as_gd_res(input).to_string(), expected.to_string());
}

/// Test enum with multiple tuple fields - should error
#[test]
fn test_multi_tuple_variant_error() {
    let input: syn::DeriveInput = parse_quote! {
        pub enum WithMultiTuple {
            A,
            B(i32, i32),
        }
    };

    let expected = quote! {
        compile_error!("`derive(AsGdEnumSimple)` only supports unit enums. Unsupported variants: B(i32, i32).\nDid you mean to derive `AsGdRes`?");
    };

    assert_eq!(expand_as_gd_res(input).to_string(), expected.to_string());
}
