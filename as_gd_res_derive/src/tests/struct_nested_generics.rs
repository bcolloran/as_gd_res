use super::expand_as_gd_res;
use super::{assert_eq, parse_quote, quote};

/// Test struct with nested generic struct fields — generic type parameters
/// inside the nested struct type arguments must be substituted.
#[test]
fn test_nested_generic_struct_fields() {
    let input: syn::DeriveInput = parse_quote! {
        #[as_gd_res_types(T1 = i32, T2 = String)]
        pub struct Parent<T1, T2> {
            pub field1: T1,
            pub field2: T2,
            pub nested: ChildStruct<T1, T2>,
        }
    };
    let actual = expand_as_gd_res(input);
    let expected = quote! {
        impl ::as_gd_res::AsGdRes for Parent<i32, String> {
            type ResType = ::godot::prelude::OnEditor<::godot::obj::Gd<ParentResource>>;
        }

        impl ::as_gd_res::AsGdResOpt for Parent<i32, String> {
            type GdOption = Option<::godot::obj::Gd<ParentResource>>;
        }

        impl ::as_gd_res::AsGdResArray for Parent<i32, String> {
            type GdArray = ::godot::prelude::Array<::godot::obj::Gd<ParentResource>>;
        }

        #[derive(::godot::prelude::GodotClass)]
        #[class(tool,init,base=Resource)]
        pub struct ParentResource {
            #[base]
            base: ::godot::obj::Base<::godot::classes::Resource>,
            #[export]
            pub field1: <i32 as ::as_gd_res::AsGdRes>::ResType,
            #[export]
            pub field2: <String as ::as_gd_res::AsGdRes>::ResType,
            #[export]
            pub nested: <ChildStruct<i32, String> as ::as_gd_res::AsGdRes>::ResType,
        }

        impl ::as_gd_res::ExtractGd for ParentResource {
            type Extracted = Parent<i32, String>;
            fn extract(&self) -> Self::Extracted {
                use ::as_gd_res::ExtractGd;
                Self::Extracted {
                    field1: self.field1.extract(),
                    field2: self.field2.extract(),
                    nested: self.nested.extract(),
                }
            }
        }
    };
    assert_eq!(actual.to_string(), expected.to_string());
}

/// Test struct with multiple nested generic struct fields.
#[test]
fn test_multiple_nested_generic_struct_fields() {
    let input: syn::DeriveInput = parse_quote! {
        #[as_gd_res_types(T1 = i32, T2 = String)]
        pub struct Parent<T1, T2> {
            pub field1: T1,
            pub field2: T2,
            pub nested1: ChildA<T1, T2>,
            pub nested2: ChildB<T1, T2>,
        }
    };
    let actual = expand_as_gd_res(input);
    let expected = quote! {
        impl ::as_gd_res::AsGdRes for Parent<i32, String> {
            type ResType = ::godot::prelude::OnEditor<::godot::obj::Gd<ParentResource>>;
        }

        impl ::as_gd_res::AsGdResOpt for Parent<i32, String> {
            type GdOption = Option<::godot::obj::Gd<ParentResource>>;
        }

        impl ::as_gd_res::AsGdResArray for Parent<i32, String> {
            type GdArray = ::godot::prelude::Array<::godot::obj::Gd<ParentResource>>;
        }

        #[derive(::godot::prelude::GodotClass)]
        #[class(tool,init,base=Resource)]
        pub struct ParentResource {
            #[base]
            base: ::godot::obj::Base<::godot::classes::Resource>,
            #[export]
            pub field1: <i32 as ::as_gd_res::AsGdRes>::ResType,
            #[export]
            pub field2: <String as ::as_gd_res::AsGdRes>::ResType,
            #[export]
            pub nested1: <ChildA<i32, String> as ::as_gd_res::AsGdRes>::ResType,
            #[export]
            pub nested2: <ChildB<i32, String> as ::as_gd_res::AsGdRes>::ResType,
        }

        impl ::as_gd_res::ExtractGd for ParentResource {
            type Extracted = Parent<i32, String>;
            fn extract(&self) -> Self::Extracted {
                use ::as_gd_res::ExtractGd;
                Self::Extracted {
                    field1: self.field1.extract(),
                    field2: self.field2.extract(),
                    nested1: self.nested1.extract(),
                    nested2: self.nested2.extract(),
                }
            }
        }
    };
    assert_eq!(actual.to_string(), expected.to_string());
}

/// Test nested generics wrapped in Option — `Option<Child<T1, T2>>` must
/// become `Option<Child<i32, String>>`.
/// We parse the nested type via `parse_quote!` so `to_tokens` spacing matches.
#[test]
fn test_option_wrapped_nested_generic() {
    let input: syn::DeriveInput = parse_quote! {
        #[as_gd_res_types(T1 = i32, T2 = String)]
        pub struct Parent<T1, T2> {
            pub field1: T1,
            pub nested_opt: Option<Child<T1, T2>>,
        }
    };
    let actual = expand_as_gd_res(input);

    let opt_child_ty: syn::Type = parse_quote! { Option<Child<i32, String>> };

    let expected = quote! {
        impl ::as_gd_res::AsGdRes for Parent<i32, String> {
            type ResType = ::godot::prelude::OnEditor<::godot::obj::Gd<ParentResource>>;
        }

        impl ::as_gd_res::AsGdResOpt for Parent<i32, String> {
            type GdOption = Option<::godot::obj::Gd<ParentResource>>;
        }

        impl ::as_gd_res::AsGdResArray for Parent<i32, String> {
            type GdArray = ::godot::prelude::Array<::godot::obj::Gd<ParentResource>>;
        }

        #[derive(::godot::prelude::GodotClass)]
        #[class(tool,init,base=Resource)]
        pub struct ParentResource {
            #[base]
            base: ::godot::obj::Base<::godot::classes::Resource>,
            #[export]
            pub field1: <i32 as ::as_gd_res::AsGdRes>::ResType,
            #[export]
            pub nested_opt: <#opt_child_ty as ::as_gd_res::AsGdRes>::ResType,
        }

        impl ::as_gd_res::ExtractGd for ParentResource {
            type Extracted = Parent<i32, String>;
            fn extract(&self) -> Self::Extracted {
                use ::as_gd_res::ExtractGd;
                Self::Extracted {
                    field1: self.field1.extract(),
                    nested_opt: self.nested_opt.extract(),
                }
            }
        }
    };
    assert_eq!(actual.to_string(), expected.to_string());
}

/// Test Vec-wrapped nested generic — `Vec<Child<T>>` → `Vec<Child<f64>>`.
/// We parse the nested type via `parse_quote!` so `to_tokens` spacing matches.
#[test]
fn test_vec_wrapped_nested_generic() {
    let input: syn::DeriveInput = parse_quote! {
        #[as_gd_res_types(T = f64)]
        pub struct Parent<T> {
            pub items: Vec<Child<T>>,
        }
    };
    let actual = expand_as_gd_res(input);

    let vec_child_ty: syn::Type = parse_quote! { Vec<Child<f64>> };

    let expected = quote! {
        impl ::as_gd_res::AsGdRes for Parent<f64> {
            type ResType = ::godot::prelude::OnEditor<::godot::obj::Gd<ParentResource>>;
        }

        impl ::as_gd_res::AsGdResOpt for Parent<f64> {
            type GdOption = Option<::godot::obj::Gd<ParentResource>>;
        }

        impl ::as_gd_res::AsGdResArray for Parent<f64> {
            type GdArray = ::godot::prelude::Array<::godot::obj::Gd<ParentResource>>;
        }

        #[derive(::godot::prelude::GodotClass)]
        #[class(tool,init,base=Resource)]
        pub struct ParentResource {
            #[base]
            base: ::godot::obj::Base<::godot::classes::Resource>,
            #[export]
            pub items: <#vec_child_ty as ::as_gd_res::AsGdRes>::ResType,
        }

        impl ::as_gd_res::ExtractGd for ParentResource {
            type Extracted = Parent<f64>;
            fn extract(&self) -> Self::Extracted {
                use ::as_gd_res::ExtractGd;
                Self::Extracted {
                    items: self.items.extract(),
                }
            }
        }
    };
    assert_eq!(actual.to_string(), expected.to_string());
}

/// Test deeper nesting: `Option<Outer<Inner<T1, T2>>>` substitutes at all
/// levels of nesting.
/// We parse the nested types via `parse_quote!` so `to_tokens` spacing matches.
#[test]
fn test_deeply_nested_generics() {
    let input: syn::DeriveInput = parse_quote! {
        #[as_gd_res_types(T1 = i32, T2 = String)]
        pub struct DeepNest<T1, T2> {
            pub field1: T1,
            pub field2: T2,
            pub nested1: Option<ChildA<T1, T2>>,
            pub nested2: Option<ChildB<T1, T2>>,
        }
    };
    let actual = expand_as_gd_res(input);

    let opt_child_a_ty: syn::Type = parse_quote! { Option<ChildA<i32, String>> };
    let opt_child_b_ty: syn::Type = parse_quote! { Option<ChildB<i32, String>> };

    let expected = quote! {
        impl ::as_gd_res::AsGdRes for DeepNest<i32, String> {
            type ResType = ::godot::prelude::OnEditor<::godot::obj::Gd<DeepNestResource>>;
        }

        impl ::as_gd_res::AsGdResOpt for DeepNest<i32, String> {
            type GdOption = Option<::godot::obj::Gd<DeepNestResource>>;
        }

        impl ::as_gd_res::AsGdResArray for DeepNest<i32, String> {
            type GdArray = ::godot::prelude::Array<::godot::obj::Gd<DeepNestResource>>;
        }

        #[derive(::godot::prelude::GodotClass)]
        #[class(tool,init,base=Resource)]
        pub struct DeepNestResource {
            #[base]
            base: ::godot::obj::Base<::godot::classes::Resource>,
            #[export]
            pub field1: <i32 as ::as_gd_res::AsGdRes>::ResType,
            #[export]
            pub field2: <String as ::as_gd_res::AsGdRes>::ResType,
            #[export]
            pub nested1: <#opt_child_a_ty as ::as_gd_res::AsGdRes>::ResType,
            #[export]
            pub nested2: <#opt_child_b_ty as ::as_gd_res::AsGdRes>::ResType,
        }

        impl ::as_gd_res::ExtractGd for DeepNestResource {
            type Extracted = DeepNest<i32, String>;
            fn extract(&self) -> Self::Extracted {
                use ::as_gd_res::ExtractGd;
                Self::Extracted {
                    field1: self.field1.extract(),
                    field2: self.field2.extract(),
                    nested1: self.nested1.extract(),
                    nested2: self.nested2.extract(),
                }
            }
        }
    };
    assert_eq!(actual.to_string(), expected.to_string());
}
