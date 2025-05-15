use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_quote, Data, DeriveInput};

/// A derive macro to emit a Godot-compatible resource struct + impls for a pure Rust struct.
#[proc_macro_derive(AsGdRes, attributes(export, init, var))]
pub fn as_gd_res_derive(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as DeriveInput);
    let expanded = expand_as_gd_res(derive_input);
    TokenStream::from(expanded)
}

fn expand_as_gd_res(input: DeriveInput) -> proc_macro2::TokenStream {
    let name = input.ident;
    let res_name = format_ident!("{}Resource", name);

    match input.data {
        Data::Struct(data) => {
            let mut field_defs = Vec::new();
            let mut extract_fields = Vec::new();

            for field in data.fields.iter() {
                let ident = field.ident.as_ref().unwrap();
                // clone export/init/var attrs or inject #[export]
                let mut attrs = field
                    .attrs
                    .iter()
                    .filter(|a| {
                        a.path.is_ident("export")
                            || a.path.is_ident("init")
                            || a.path.is_ident("var")
                    })
                    .cloned()
                    .collect::<Vec<_>>();
                if attrs.is_empty() {
                    attrs.push(parse_quote!(#[export]));
                }
                let ty = &field.ty;
                // resource struct field
                field_defs.push(quote! {
                    #(#attrs)*
                    pub #ident: <#ty as AsGdRes>::ResType,
                });
                // extraction mapping
                extract_fields.push(quote! {
                    #ident: self.#ident.extract(),
                });
            }

            quote! {
                impl AsGdRes for #name {
                    type ResType = Gd<#res_name>;
                }

                #[derive(GodotClass)]
                #[class(tool, init, base=Resource)]
                pub struct #res_name {
                    #[base]
                    base: Base<Resource>,
                    #(#field_defs)*
                }

                impl ExtractGd for #res_name {
                    type Extracted = #name;
                    fn extract(&self) -> Self::Extracted {
                        Self::Extracted {
                            #(#extract_fields)*
                        }
                    }
                }
            }
        }
        _ => quote! {
            compile_error!("AsGdRes derive only supports plain structs in test mode");
        }
        .into(),
    }
}

#[cfg(test)]
mod tests {
    use super::expand_as_gd_res;
    use pretty_assertions::assert_eq;
    use quote::quote;
    use syn::parse_quote;

    #[test]
    fn test_simple() {
        let input: syn::DeriveInput = parse_quote! {
            pub struct SimpleStructParams {
                a: f32,
                b: f32,
            }
        };
        let actual = expand_as_gd_res(input);
        let expected = quote! {
          impl AsGdRes for SimpleStructParams {
              type ResType = Gd<SimpleStructParamsResource>;
          }

          #[derive(GodotClass)]
          #[class(tool, init, base=Resource)]
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
                  Self::Extracted {
                      a: self.a.extract(),
                      b: self.b.extract(),
                  }
              }
          }
        };
        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn test_2() {
        let input: syn::DeriveInput = parse_quote! {
            pub struct DropParams2 {
                pub total_value: f32,
                pub max_value_per_coin: f32,
                pub coin_scene_1: Option<PackedScenePath>,
                pub coin_scene_2: OnEditorInit<PackedScenePath>,
            }
        };
        let actual = expand_as_gd_res(input);
        let expected = quote! {

                impl AsGdRes for DropParams2 {
                    type ResType = Gd<DropParams2Resource>;
                }

                #[derive(GodotClass)]
                #[class(tool, init, base=Resource)]
                pub struct DropParams2Resource {
                    #[base]
                    base: Base<Resource>,
                    #[export]
                    pub total_value: <f32 as AsGdRes>::ResType,
                    #[export]
                    pub max_value_per_coin: <f32 as AsGdRes>::ResType,
                    #[export]
                    pub coin_scene_1: <Option<PackedScenePath> as AsGdRes>::ResType,
                    #[export]
                    pub coin_scene_2: <OnEditorInit<PackedScenePath> as AsGdRes>::ResType,
                }

                impl ExtractGd for DropParams2Resource {
                    type Extracted = DropParams2;
                    fn extract(&self) -> Self::Extracted {
                        Self::Extracted {
                            total_value: self.total_value.extract(),
                            max_value_per_coin: self.max_value_per_coin.extract(),
                            coin_scene_1: self.coin_scene_1.extract(),
                            coin_scene_2: self.coin_scene_2.extract(),
                        }
                    }
                }

        };
        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn test_attr_pass_through() {
        let input: syn::DeriveInput = parse_quote! {
            pub struct DropParams2 {
              #[export(range = (100.0, 500.0))]
              #[init(val = 200.0)]
              pub total_value: f32,

              #[export(range = (0.0, 5.0))]
              #[init(val = 3.0)]
              pub max_value_per_coin: f32,
              pub coin_scene_1: Option<PackedScenePath>,
              pub coin_scene_2: OnEditorInit<PackedScenePath>,
          }
        };

        let expected = quote! {
          impl AsGdRes for DropParams2 {
              type ResType = Gd<DropParams2Resource>;
          }

          #[derive(GodotClass)]
          #[class(tool, init, base=Resource)]
          pub struct DropParams2Resource {
              #[base]
              base: Base<Resource>,
              #[export(range = (100.0, 500.0))]
              #[init(val = 200.0)]
              pub total_value: <f32 as AsGdRes>::ResType,
              #[export(range = (0.0, 5.0))]
              #[init(val = 3.0)]
              pub max_value_per_coin: <f32 as AsGdRes>::ResType,
              #[export]
              pub coin_scene_1: <Option<PackedScenePath> as AsGdRes>::ResType,
              #[export]
              pub coin_scene_2: <OnEditorInit<PackedScenePath> as AsGdRes>::ResType,
          }

          impl ExtractGd for DropParams2Resource {
              type Extracted = DropParams2;
              fn extract(&self) -> Self::Extracted {
                  Self::Extracted {
                      total_value: self.total_value.extract(),
                      max_value_per_coin: self.max_value_per_coin.extract(),
                      coin_scene_1: self.coin_scene_1.extract(),
                      coin_scene_2: self.coin_scene_2.extract(),
                  }
              }
          }

        };

        assert_eq!(expand_as_gd_res(input).to_string(), expected.to_string());
    }
}
