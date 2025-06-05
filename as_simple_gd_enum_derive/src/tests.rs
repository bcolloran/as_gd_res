use super::expand_as_gd_res;
use pretty_assertions::assert_eq;
use quote::quote;
use syn::parse_quote;

#[test]
fn test_simple_enum() {
    let input: syn::DeriveInput = parse_quote! {
            pub enum Element {
                #[default]
                Fire,
                Water,
                Earth,
                Air,
            }

    };

    let expected = quote! {

        #[derive(::godot::prelude::GodotConvert, ::godot::prelude::Var, ::godot::prelude::Export, Clone, Copy, Debug, PartialEq, Eq)]
        #[godot(via = GString)]
        pub enum ElementAsGdEnum {
            Fire,
            Water,
            Earth,
            Air,
        }
        impl ::as_gd_res::AsSimpleGdEnum for Element {
            type GdEnumType = ElementAsGdEnum;
        }

        impl ::as_gd_res::ExtractGd for ElementAsGdEnum {
            type Extracted = Element;
            fn extract(&self) -> Self::Extracted {
                (*self).into()
            }
        }

        impl From<Element> for ElementAsGdEnum {
            fn from(value: Element) -> ElementAsGdEnum {
                match value {
                    Element::Fire => ElementAsGdEnum::Fire,
                    Element::Water => ElementAsGdEnum::Water,
                    Element::Earth => ElementAsGdEnum::Earth,
                    Element::Air => ElementAsGdEnum::Air,
                }
            }
        }
        impl From<ElementAsGdEnum> for Element {
            fn from(value: ElementAsGdEnum) -> Element {
                match value {
                    ElementAsGdEnum::Fire => Element::Fire,
                    ElementAsGdEnum::Water => Element::Water,
                    ElementAsGdEnum::Earth => Element::Earth,
                    ElementAsGdEnum::Air => Element::Air,
                }
            }
        }
        impl Default for ElementAsGdEnum {
            fn default() -> Self {
                Element::default().into()
            }
        }

    };

    assert_eq!(expand_as_gd_res(input).to_string(), expected.to_string());
}

#[test]
fn test_enum_with_data_error() {
    let input: syn::DeriveInput = parse_quote! {
            pub enum Element {
                #[default]
                Fire(u32),
                Water(f32),
                Earth,
                Air,
            }

    };

    let expected = quote! {

        compile_error!("`derive(AsSimpleGdEnum)` only supports unit enums. Unsupported variants: Fire(u32), Water(f32).\nDid you mean to derive `AsGdRes`?");

    };

    assert_eq!(expand_as_gd_res(input).to_string(), expected.to_string());
}

#[test]
fn test_struct_error() {
    let input: syn::DeriveInput = parse_quote! {
            pub struct Foo {
                a: u32,
                b: String
            }
    };

    let expected = quote! {

                compile_error!(
                    "AsSimpleGdEnum derive only supports enums with unit variants, not structs. Did you mean to derive `AsGdRes`?"
                );
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

                compile_error!(
                    "AsSimpleGdEnum derive only supports enums with unit variants, not structs. Did you mean to derive `AsGdRes`?"
                );
    };

    assert_eq!(expand_as_gd_res(input).to_string(), expected.to_string());
}

#[test]
fn test_generic_enum_error() {
    let input: syn::DeriveInput = parse_quote! {
        pub enum Generic<T> {
            A,
            B(T),
        }
    };
    let expected = quote! {
        compile_error!("`derive(AsSimpleGdEnum)` does not support generics");
    };
    assert_eq!(expand_as_gd_res(input).to_string(), expected.to_string());
}
