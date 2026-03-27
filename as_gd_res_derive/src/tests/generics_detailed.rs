use super::expand_as_gd_res;
use super::{assert_eq, parse_quote, quote};

// NOTE: These tests document the current behavior of the nested generic type substitution.
// The `substitute_type` function currently only handles top-level type paths, not nested generics.
// This means that types like `Vec<T>` where T is a generic parameter will NOT have T substituted.

/// Test that simple generic type substitution works
#[test]
fn test_simple_generic_substitution() {
    let input: syn::DeriveInput = parse_quote! {
        #[as_gd_res_types(T = i32)]
        pub struct SimpleGeneric<T> {
            pub field: T,
        }
    };
    let actual = expand_as_gd_res(input);
    let expected = quote! {
        impl ::as_gd_res::AsGdRes for SimpleGeneric<i32> {
            type ResType = ::godot::prelude::OnEditor<::godot::obj::Gd<SimpleGenericResource>>;
        }

        impl ::as_gd_res::AsGdResOpt for SimpleGeneric<i32> {
            type GdOption = Option<::godot::obj::Gd<SimpleGenericResource>>;
        }

        impl ::as_gd_res::AsGdResArray for SimpleGeneric<i32> {
            type GdArray = ::godot::prelude::Array<::godot::obj::Gd<SimpleGenericResource>>;
        }

        #[derive(::godot::prelude::GodotClass)]
        #[class(tool,init,base=Resource)]
        pub struct SimpleGenericResource {
            #[base]
            base: ::godot::obj::Base<::godot::classes::Resource>,
            #[export]
            pub field: <i32 as ::as_gd_res::AsGdRes>::ResType,
        }

        impl ::as_gd_res::ExtractGd for SimpleGenericResource {
            type Extracted = SimpleGeneric<i32>;
            fn extract(&self) -> Self::Extracted {
                use ::as_gd_res::ExtractGd;
                Self::Extracted {
                    field: self.field.extract(),
                }
            }
        }
    };
    assert_eq!(actual.to_string(), expected.to_string());
}

/// Test that multiple generic parameters work correctly
#[test]
fn test_multiple_generic_parameters() {
    let input: syn::DeriveInput = parse_quote! {
        #[as_gd_res_types(A = i32, B = f32, C = bool)]
        pub struct MultiGeneric<A, B, C> {
            pub field_a: A,
            pub field_b: B,
            pub field_c: C,
        }
    };
    let actual = expand_as_gd_res(input);
    let expected = quote! {
        impl ::as_gd_res::AsGdRes for MultiGeneric<i32, f32, bool> {
            type ResType = ::godot::prelude::OnEditor<::godot::obj::Gd<MultiGenericResource>>;
        }

        impl ::as_gd_res::AsGdResOpt for MultiGeneric<i32, f32, bool> {
            type GdOption = Option<::godot::obj::Gd<MultiGenericResource>>;
        }

        impl ::as_gd_res::AsGdResArray for MultiGeneric<i32, f32, bool> {
            type GdArray = ::godot::prelude::Array<::godot::obj::Gd<MultiGenericResource>>;
        }

        #[derive(::godot::prelude::GodotClass)]
        #[class(tool,init,base=Resource)]
        pub struct MultiGenericResource {
            #[base]
            base: ::godot::obj::Base<::godot::classes::Resource>,
            #[export]
            pub field_a: <i32 as ::as_gd_res::AsGdRes>::ResType,
            #[export]
            pub field_b: <f32 as ::as_gd_res::AsGdRes>::ResType,
            #[export]
            pub field_c: <bool as ::as_gd_res::AsGdRes>::ResType,
        }

        impl ::as_gd_res::ExtractGd for MultiGenericResource {
            type Extracted = MultiGeneric<i32, f32, bool>;
            fn extract(&self) -> Self::Extracted {
                use ::as_gd_res::ExtractGd;
                Self::Extracted {
                    field_a: self.field_a.extract(),
                    field_b: self.field_b.extract(),
                    field_c: self.field_c.extract(),
                }
            }
        }
    };
    assert_eq!(actual.to_string(), expected.to_string());
}

/// Test that generics with complex type mappings work
#[test]
fn test_generic_with_complex_type() {
    let input: syn::DeriveInput = parse_quote! {
        #[as_gd_res_types(T = SomeComplexType)]
        pub struct WithComplexType<T> {
            pub field: T,
        }
    };
    let actual = expand_as_gd_res(input);
    let expected = quote! {
        impl ::as_gd_res::AsGdRes for WithComplexType<SomeComplexType> {
            type ResType = ::godot::prelude::OnEditor<::godot::obj::Gd<WithComplexTypeResource>>;
        }

        impl ::as_gd_res::AsGdResOpt for WithComplexType<SomeComplexType> {
            type GdOption = Option<::godot::obj::Gd<WithComplexTypeResource>>;
        }

        impl ::as_gd_res::AsGdResArray for WithComplexType<SomeComplexType> {
            type GdArray = ::godot::prelude::Array<::godot::obj::Gd<WithComplexTypeResource>>;
        }

        #[derive(::godot::prelude::GodotClass)]
        #[class(tool,init,base=Resource)]
        pub struct WithComplexTypeResource {
            #[base]
            base: ::godot::obj::Base<::godot::classes::Resource>,
            #[export]
            pub field: <SomeComplexType as ::as_gd_res::AsGdRes>::ResType,
        }

        impl ::as_gd_res::ExtractGd for WithComplexTypeResource {
            type Extracted = WithComplexType<SomeComplexType>;
            fn extract(&self) -> Self::Extracted {
                use ::as_gd_res::ExtractGd;
                Self::Extracted {
                    field: self.field.extract(),
                }
            }
        }
    };
    assert_eq!(actual.to_string(), expected.to_string());
}

/// Test that NESTED generic types are NOT substituted (documents current limitation)
/// For example, Vec<T> where T is generic - T will NOT be substituted
#[test]
fn test_nested_generic_not_substituted() {
    let input: syn::DeriveInput = parse_quote! {
        #[as_gd_res_types(T = i32)]
        pub struct NestedGeneric<T> {
            // NOTE: Vec<T> is not substituted - T inside Vec stays as T
            pub field: Vec<T>,
        }
    };
    let actual = expand_as_gd_res(input);
    // This shows the CURRENT (arguably incorrect) behavior:
    // Vec<T> is NOT substituted to Vec<i32>
    let expected = quote! {
        impl ::as_gd_res::AsGdRes for NestedGeneric<i32> {
            type ResType = ::godot::prelude::OnEditor<::godot::obj::Gd<NestedGenericResource>>;
        }

        impl ::as_gd_res::AsGdResOpt for NestedGeneric<i32> {
            type GdOption = Option<::godot::obj::Gd<NestedGenericResource>>;
        }

        impl ::as_gd_res::AsGdResArray for NestedGeneric<i32> {
            type GdArray = ::godot::prelude::Array<::godot::obj::Gd<NestedGenericResource>>;
        }

        #[derive(::godot::prelude::GodotClass)]
        #[class(tool,init,base=Resource)]
        pub struct NestedGenericResource {
            #[base]
            base: ::godot::obj::Base<::godot::classes::Resource>,
            #[export]
            // NOTE: This is Vec<T> not Vec<i32> - showing the limitation
            pub field: <Vec<T> as ::as_gd_res::AsGdRes>::ResType,
        }

        impl ::as_gd_res::ExtractGd for NestedGenericResource {
            type Extracted = NestedGeneric<i32>;
            fn extract(&self) -> Self::Extracted {
                use ::as_gd_res::ExtractGd;
                Self::Extracted {
                    field: self.field.extract(),
                }
            }
        }
    };
    assert_eq!(actual.to_string(), expected.to_string());
}

/// Test that Option<T> with generic T is NOT substituted (documents current limitation)
#[test]
fn test_option_generic_not_substituted() {
    let input: syn::DeriveInput = parse_quote! {
        #[as_gd_res_types(T = i32)]
        pub struct OptionGeneric<T> {
            // NOTE: Option<T> is not substituted - T inside Option stays as T
            pub field: Option<T>,
        }
    };
    let actual = expand_as_gd_res(input);
    // This shows the CURRENT (arguably incorrect) behavior:
    // Option<T> is NOT substituted to Option<i32>
    let expected = quote! {
        impl ::as_gd_res::AsGdRes for OptionGeneric<i32> {
            type ResType = ::godot::prelude::OnEditor<::godot::obj::Gd<OptionGenericResource>>;
        }

        impl ::as_gd_res::AsGdResOpt for OptionGeneric<i32> {
            type GdOption = Option<::godot::obj::Gd<OptionGenericResource>>;
        }

        impl ::as_gd_res::AsGdResArray for OptionGeneric<i32> {
            type GdArray = ::godot::prelude::Array<::godot::obj::Gd<OptionGenericResource>>;
        }

        #[derive(::godot::prelude::GodotClass)]
        #[class(tool,init,base=Resource)]
        pub struct OptionGenericResource {
            #[base]
            base: ::godot::obj::Base<::godot::classes::Resource>,
            #[export]
            // NOTE: This is Option<T> not Option<i32> - showing the limitation
            pub field: <Option<T> as ::as_gd_res::AsGdRes>::ResType,
        }

        impl ::as_gd_res::ExtractGd for OptionGenericResource {
            type Extracted = OptionGeneric<i32>;
            fn extract(&self) -> Self::Extracted {
                use ::as_gd_res::ExtractGd;
                Self::Extracted {
                    field: self.field.extract(),
                }
            }
        }
    };
    assert_eq!(actual.to_string(), expected.to_string());
}

/// Test partial generic mapping (only some generics are mapped)
#[test]
fn test_partial_generic_mapping() {
    let input: syn::DeriveInput = parse_quote! {
        // Only map A, not B - B should remain as generic
        #[as_gd_res_types(A = i32)]
        pub struct PartialGeneric<A, B> {
            pub field_a: A,
            pub field_b: B,
        }
    };
    let actual = expand_as_gd_res(input);
    // With partial mapping, only A is in concrete_type_args
    let expected = quote! {
        impl ::as_gd_res::AsGdRes for PartialGeneric<i32> {
            type ResType = ::godot::prelude::OnEditor<::godot::obj::Gd<PartialGenericResource>>;
        }

        impl ::as_gd_res::AsGdResOpt for PartialGeneric<i32> {
            type GdOption = Option<::godot::obj::Gd<PartialGenericResource>>;
        }

        impl ::as_gd_res::AsGdResArray for PartialGeneric<i32> {
            type GdArray = ::godot::prelude::Array<::godot::obj::Gd<PartialGenericResource>>;
        }

        #[derive(::godot::prelude::GodotClass)]
        #[class(tool,init,base=Resource)]
        pub struct PartialGenericResource {
            #[base]
            base: ::godot::obj::Base<::godot::classes::Resource>,
            #[export]
            pub field_a: <i32 as ::as_gd_res::AsGdRes>::ResType,
            #[export]
            // B is not in map, so stays as B
            pub field_b: <B as ::as_gd_res::AsGdRes>::ResType,
        }

        impl ::as_gd_res::ExtractGd for PartialGenericResource {
            type Extracted = PartialGeneric<i32>;
            fn extract(&self) -> Self::Extracted {
                use ::as_gd_res::ExtractGd;
                Self::Extracted {
                    field_a: self.field_a.extract(),
                    field_b: self.field_b.extract(),
                }
            }
        }
    };
    assert_eq!(actual.to_string(), expected.to_string());
}
