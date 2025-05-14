use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_quote, Attribute, Data, DeriveInput, Fields};

/// A derive macro to emit a Godot-compatible resource struct + impls for a pure Rust struct or enum.
///
/// - Always appends `Resource` to the input type name for the generated GodotClass struct.
/// - For structs:
///   - Insert at the top:
///     ```rust
///     #[derive(GodotClass)]
///     #[class(tool, init, base=Resource)]
///     pub struct {Name}Resource {{
///         #[base]
///         base: Base<Resource>,
///         // ... fields ...
///     }}
///     ```
///   - Propagate `#[export(...)]`, `#[init(...)]`, `#[var(...)]` on each input field;
///     if none of those attrs are present, inject `#[export]`.
///   - Map each field type `T` to `<T as AsGdRes>::ResType`.
///   - Emit:
///     ```rust
///     impl AsGdRes for {Name} {{ type ResType = Gd<{Name}Resource>; }}
///     impl ExtractGd for {Name}Resource {{ ... }}
///     ```
///
/// - For enums:
///   1. Unit-only enums: generate `impl AsGdRes for E { type ResType = E; }` and clone-based `ExtractGd`.
///   2. Single-tuple variants only: generate a `trait {E}DynRes` with `extract_enum_data(&self) -> E`;
///      impl that trait for each `{Variant}Resource`, and
///      `impl AsGdRes for E { type ResType = DynGd<Resource, dyn {E}DynRes>; }` plus `ExtractGd` for the dyn.
///   - Otherwise emit `compile_error!("AsGdRes only supports unit enums or single-tuple enums");
#[proc_macro_derive(AsGdRes, attributes(export, init, var))]
pub fn as_gd_res_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // 1. convert to proc_macro2
    let ts2: proc_macro2::TokenStream = input.into();
    // 2. parse into a DeriveInput
    let derive_input = syn::parse2::<syn::DeriveInput>(ts2).expect("failed to parse DeriveInput");
    // 3. expand in proc_macro2 land
    let output2 = expand_as_gd_res(derive_input);
    // 4. convert back for the compiler
    output2.into()
}

fn expand_as_gd_res(input: syn::DeriveInput) -> proc_macro2::TokenStream {
    // let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Resource type name: {Name}Resource
    let res_name = format_ident!("{}Resource", name);

    match input.data {
        Data::Struct(data) => {
            // Collect fields
            let mut fields_tokens = Vec::new();
            for field in data.fields.iter() {
                let ident = field.ident.as_ref().unwrap();
                // Determine which attrs to propagate, clone them to own
                let mut attrs: Vec<Attribute> = field
                    .attrs
                    .iter()
                    .filter(|a| {
                        a.path.is_ident("export")
                            || a.path.is_ident("init")
                            || a.path.is_ident("var")
                    })
                    .cloned()
                    .collect();
                if attrs.is_empty() {
                    // inject owned #[export]
                    attrs.push(parse_quote!(#[export]));
                }
                let ty = &field.ty;
                fields_tokens.push(quote! {
                    #(#attrs)*
                    pub #ident: <#ty as AsGdRes>::ResType,
                });
            }

            let output = quote! {
                // AsGdRes impl for original type
                impl AsGdRes for #name {
                    type ResType = Gd<#res_name>;
                }

                // Generated GodotClass resource struct
                #[derive(GodotClass)]
                #[class(tool, init, base=Resource)]
                pub struct #res_name {
                    #[base]
                    base: Base<Resource>,
                    #(#fields_tokens)*
                }

                // ExtractGd impl for resource -> pure data
                impl ExtractGd for #res_name {
                    type Extracted = #name;
                    fn extract(&self) -> Self::Extracted {
                        #name {
                            #(
                                #fields_tokens
                            )*
                        }
                    }
                }
            };

            // TokenStream::from(output)
            output.into()
        }
        Data::Enum(data) => {
            // Check variants
            let mut unit_only = true;
            let mut single_tuple = true;
            for var in data.variants.iter() {
                match &var.fields {
                    Fields::Unit => {}
                    Fields::Unnamed(u) if u.unnamed.len() == 1 => {
                        unit_only = false;
                    }
                    _ => {
                        unit_only = false;
                        single_tuple = false;
                    }
                }
            }

            let tokens = if unit_only {
                // unit-only enum
                quote! {
                    impl AsGdRes for #name {
                        type ResType = #name;
                    }
                    impl ExtractGd for #name {
                        type Extracted = #name;
                        fn extract(&self) -> Self::Extracted {
                            self.clone()
                        }
                    }
                }
            } else if single_tuple {
                // single-tuple variants only
                let dyn_trait = format_ident!("{}DynRes", name);
                let mut enum_trait_impls = Vec::new();
                for var in data.variants.iter() {
                    let var_ident = &var.ident;
                    let res_var = format_ident!("{}Resource", var_ident);
                    enum_trait_impls.push(quote! {
                        impl #dyn_trait for #res_var {
                            fn extract_enum_data(&self) -> #name {
                                #name::#var_ident(self.extract())
                            }
                        }
                    });
                }
                quote! {
                    // DynRes trait
                    pub trait #dyn_trait { fn extract_enum_data(&self) -> #name; }

                    #(#enum_trait_impls)*

                    impl AsGdRes for #name {
                        type ResType = DynGd<Resource, dyn #dyn_trait>;
                    }
                    impl ExtractGd for DynGd<Resource, dyn #dyn_trait> {
                        type Extracted = #name;
                        fn extract(&self) -> Self::Extracted {
                            self.dyn_bind().extract_enum_data()
                        }
                    }
                }
            } else {
                quote! {
                    compile_error!("AsGdRes only supports unit enums or single-tuple enums");
                }
            };

            tokens.into()
        }
        _ => TokenStream::from(quote! {
            compile_error!("AsGdRes can only be applied to structs or enums");
        })
        .into(),
    }
}

#[test]
fn test_debug() {
    // 1. build a DeriveInput from `syn::parse_quote!`
    let input: syn::DeriveInput = syn::parse_quote! {
        pub struct SimpleStructParams {
            a: f32,
            b: f32,
        }
    };

    // 2. invoke *just* the proc_macro2 helper
    let actual2 = expand_as_gd_res(input);

    // 3. build your expected tokens also in proc_macro2
    let expected2 = quote! {
      #[derive(GodotClass)]
      #[class (tool , init , base = Resource)]
      pub struct SimpleStructParamsResource {
          #[base]
          base: Base<Resource>,
          #[export]
          pub a: <f32 as AsGdRes>::ResType,
          #[export]
          pub b: <f32 as AsGdRes>::ResType,
      }

      impl ExtractGd for SimpleStructParamsResource {
          type Extracted = SimpleStructParams;
          fn extract(&self) -> Self::Extracted {
              SimpleStructParams {
                  a: self.a.extract(),
                  b: self.b.extract(),
              }
          }
      }
    };

    assert_eq!(actual2.to_string(), expected2.to_string());
}
