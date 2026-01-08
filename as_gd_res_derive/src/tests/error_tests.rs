use super::expand_as_gd_res;
use super::{assert_eq, quote, parse_quote};

#[test]
fn test_union_error() {
    let input: syn::DeriveInput = parse_quote! {
        pub union Foo {
            a: u32,
            b: f32,
        }
    };
    let expected = quote! {
        compile_error!("`derive(AsGdRes)` only supports structs with named fields, enums with unit variants, or enums with single-tuple variants");
    };
    assert_eq!(expand_as_gd_res(input).to_string(), expected.to_string());
}

#[test]
fn test_tuple_struct_error() {
    let input: syn::DeriveInput = parse_quote! {
        pub struct Foo(u32);
    };
    let expected = quote! {
        compile_error!("`derive(AsGdRes)` only supports structs with named fields");
    };
    assert_eq!(expand_as_gd_res(input).to_string(), expected.to_string());
}

#[test]
fn test_generic_struct_error() {
    let input: syn::DeriveInput = parse_quote! {
        pub struct Bar<T> { val: T }
    };
    let expected = quote! {
        compile_error!("`derive(AsGdRes)` does not support generics");
    };
    assert_eq!(expand_as_gd_res(input).to_string(), expected.to_string());
}
