use super::expand_as_gd_res;
use super::{assert_eq, parse_quote, quote};

/// Test that an empty struct generates valid code
/// (This is an edge case that wasn't previously covered)
#[test]
fn test_empty_struct() {
    let input: syn::DeriveInput = parse_quote! {
        pub struct EmptyStruct {}
    };
    let actual = expand_as_gd_res(input);
    let expected = quote! {
        impl ::as_gd_res::AsGdRes for EmptyStruct {
            type ResType = ::godot::prelude::OnEditor<::godot::obj::Gd<EmptyStructResource>>;
        }

        impl ::as_gd_res::AsGdResOpt for EmptyStruct {
            type GdOption = Option<::godot::obj::Gd<EmptyStructResource>>;
        }

        impl ::as_gd_res::AsGdResArray for EmptyStruct {
            type GdArray = ::godot::prelude::Array<::godot::obj::Gd<EmptyStructResource>>;
        }

        #[derive(::godot::prelude::GodotClass)]
        #[class(tool,init,base=Resource)]
        pub struct EmptyStructResource {
            #[base]
            base: ::godot::obj::Base<::godot::classes::Resource>,
        }

        impl ::as_gd_res::ExtractGd for EmptyStructResource {
            type Extracted = EmptyStruct;
            fn extract(&self) -> Self::Extracted {
                use ::as_gd_res::ExtractGd;
                Self::Extracted {
                }
            }
        }
    };
    assert_eq!(actual.to_string(), expected.to_string());
}

/// Test that an empty enum with data variants generates appropriate error
#[test]
fn test_empty_enum() {
    let input: syn::DeriveInput = parse_quote! {
        pub enum EmptyEnum {}
    };
    let actual = expand_as_gd_res(input);
    // Empty enum has no variants, so all_unit = true (vacuously)
    // This should produce the error for unit enums
    let expected = quote! {
        compile_error!(
            "`derive(AsGdRes)` only supports enums with single-tuple variants, not unit variants. Did you mean to use `derive(AsGdEnumSimple)`?"
        );
    };
    assert_eq!(actual.to_string(), expected.to_string());
}

/// Test struct with only #[var] attribute (no #[export])
#[test]
fn test_struct_var_only_attribute() {
    let input: syn::DeriveInput = parse_quote! {
        pub struct VarOnlyStruct {
            #[var]
            pub field: i32,
        }
    };
    let actual = expand_as_gd_res(input);
    let expected = quote! {
        impl ::as_gd_res::AsGdRes for VarOnlyStruct {
            type ResType = ::godot::prelude::OnEditor<::godot::obj::Gd<VarOnlyStructResource>>;
        }

        impl ::as_gd_res::AsGdResOpt for VarOnlyStruct {
            type GdOption = Option<::godot::obj::Gd<VarOnlyStructResource>>;
        }

        impl ::as_gd_res::AsGdResArray for VarOnlyStruct {
            type GdArray = ::godot::prelude::Array<::godot::obj::Gd<VarOnlyStructResource>>;
        }

        #[derive(::godot::prelude::GodotClass)]
        #[class(tool,init,base=Resource)]
        pub struct VarOnlyStructResource {
            #[base]
            base: ::godot::obj::Base<::godot::classes::Resource>,
            #[var]
            pub field: <i32 as ::as_gd_res::AsGdRes>::ResType,
        }

        impl ::as_gd_res::ExtractGd for VarOnlyStructResource {
            type Extracted = VarOnlyStruct;
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

/// Test struct with #[init] only attribute (no #[export] or #[var])
/// Should add #[export] by default since init is not a display attribute
#[test]
fn test_struct_init_only_attribute() {
    let input: syn::DeriveInput = parse_quote! {
        pub struct InitOnlyStruct {
            #[init(val = 42)]
            pub field: i32,
        }
    };
    let actual = expand_as_gd_res(input);
    let expected = quote! {
        impl ::as_gd_res::AsGdRes for InitOnlyStruct {
            type ResType = ::godot::prelude::OnEditor<::godot::obj::Gd<InitOnlyStructResource>>;
        }

        impl ::as_gd_res::AsGdResOpt for InitOnlyStruct {
            type GdOption = Option<::godot::obj::Gd<InitOnlyStructResource>>;
        }

        impl ::as_gd_res::AsGdResArray for InitOnlyStruct {
            type GdArray = ::godot::prelude::Array<::godot::obj::Gd<InitOnlyStructResource>>;
        }

        #[derive(::godot::prelude::GodotClass)]
        #[class(tool,init,base=Resource)]
        pub struct InitOnlyStructResource {
            #[base]
            base: ::godot::obj::Base<::godot::classes::Resource>,
            #[init(val = 42)]
            pub field: <i32 as ::as_gd_res::AsGdRes>::ResType,
        }

        impl ::as_gd_res::ExtractGd for InitOnlyStructResource {
            type Extracted = InitOnlyStruct;
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

/// Test private struct (visibility should be preserved for the impl)
#[test]
fn test_private_struct() {
    let input: syn::DeriveInput = parse_quote! {
        struct PrivateStruct {
            pub field: i32,
        }
    };
    let actual = expand_as_gd_res(input);
    let expected = quote! {
        impl ::as_gd_res::AsGdRes for PrivateStruct {
            type ResType = ::godot::prelude::OnEditor<::godot::obj::Gd<PrivateStructResource>>;
        }

        impl ::as_gd_res::AsGdResOpt for PrivateStruct {
            type GdOption = Option<::godot::obj::Gd<PrivateStructResource>>;
        }

        impl ::as_gd_res::AsGdResArray for PrivateStruct {
            type GdArray = ::godot::prelude::Array<::godot::obj::Gd<PrivateStructResource>>;
        }

        #[derive(::godot::prelude::GodotClass)]
        #[class(tool,init,base=Resource)]
        pub struct PrivateStructResource {
            #[base]
            base: ::godot::obj::Base<::godot::classes::Resource>,
            #[export]
            pub field: <i32 as ::as_gd_res::AsGdRes>::ResType,
        }

        impl ::as_gd_res::ExtractGd for PrivateStructResource {
            type Extracted = PrivateStruct;
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

/// Test enum with single variant (edge case for enum handling)
#[test]
fn test_single_variant_enum() {
    let input: syn::DeriveInput = parse_quote! {
        pub enum SingleVariant {
            Only(SomeData),
        }
    };
    let actual = expand_as_gd_res(input);
    let expected = quote! {
        pub trait SingleVariantResourceExtractVariant {
            fn extract_enum_variant(&self) -> SingleVariant;
        }

        type SingleVariantResource =
            ::godot::obj::DynGd<::godot::classes::Resource, dyn SingleVariantResourceExtractVariant>;

        impl ::as_gd_res::AsGdRes for SingleVariant {
            type ResType = ::godot::prelude::OnEditor<SingleVariantResource>;
        }
        impl ::as_gd_res::AsGdResOpt for SingleVariant {
            type GdOption = Option<SingleVariantResource>;
        }
        impl ::as_gd_res::AsGdResArray for SingleVariant {
            type GdArray = ::godot::prelude::Array<SingleVariantResource>;
        }

        impl ::as_gd_res::ExtractGd for dyn SingleVariantResourceExtractVariant {
            type Extracted = SingleVariant;
            fn extract(&self) -> Self::Extracted {
                self.extract_enum_variant()
            }
        }

        pub mod mod_singlevariant_only {
            use super::*;
            use ::godot::prelude::godot_dyn;
            #[godot_dyn]
            impl SingleVariantResourceExtractVariant for SomeDataResource {
                fn extract_enum_variant(&self) -> SingleVariant {
                    SingleVariant::Only(self.extract())
                }
            }
        }
    };
    assert_eq!(actual.to_string(), expected.to_string());
}

/// Test post_init struct with empty fields (edge case)
#[test]
fn test_post_init_empty_struct() {
    let input: syn::DeriveInput = parse_quote! {
        #[as_gd_res(post_init = init_method)]
        pub struct EmptyPostInit {}
    };
    let actual = expand_as_gd_res(input);
    let expected = quote! {
        impl ::as_gd_res::AsGdRes for EmptyPostInit {
            type ResType = ::godot::prelude::OnEditor<::godot::obj::Gd<EmptyPostInitResource>>;
        }
        impl ::as_gd_res::AsGdResOpt for EmptyPostInit {
            type GdOption = Option<::godot::obj::Gd<EmptyPostInitResource>>;
        }
        impl ::as_gd_res::AsGdResArray for EmptyPostInit {
            type GdArray = ::godot::prelude::Array<::godot::obj::Gd<EmptyPostInitResource>>;
        }

        #[derive(::godot::prelude::GodotClass)]
        #[class(tool,base = Resource)]
        pub struct EmptyPostInitResource {
            #[base]
            base: ::godot::obj::Base<::godot::classes::Resource>,
        }

        impl ::as_gd_res::ExtractGd for EmptyPostInitResource {
            type Extracted = EmptyPostInit;
            fn extract(&self) -> Self::Extracted {
                use ::as_gd_res::ExtractGd;
                Self::Extracted {
                }
            }
        }

        #[godot_api]
        impl ::godot::prelude::IResource for EmptyPostInitResource {
            fn init(base: ::godot::prelude::Base<::godot::prelude::Resource>) -> Self {
                let mut res = Self {
                    base,
                };
                res.init_method();
                res
            }
        }
    };
    assert_eq!(actual.to_string(), expected.to_string());
}
