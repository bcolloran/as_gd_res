use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_quote, Data, DeriveInput, Fields};

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

    #[test]
    fn test_simple_enum() {
        let input: syn::DeriveInput = parse_quote! {
                #[derive(Default, Clone, Copy, GodotConvert, Var, Export)]
                #[godot(via = GString)]
                // #[derive(AsGdRes)] // commented out because this is not implemented yet, this is an example of what we want to be able to do
                pub enum DamageTeam {
                    #[default]
                    Player,
                    Enemy,
                    Environment,
                }

        };

        let expected = quote! {
                impl AsGdRes for DamageTeam {
                    type ResType = DamageTeam;
                }

                impl ExtractGd for DamageTeam {
                    type Extracted = DamageTeam;
                    fn extract(&self) -> Self::Extracted {
                        self.clone()
                    }
                }

        };

        assert_eq!(expand_as_gd_res(input).to_string(), expected.to_string());
    }

    #[test]
    fn test_enum_with_data_variants() {
        let input: syn::DeriveInput = parse_quote! {
                pub enum BrainParams {
                        Roomba(RoombaBrainParams),
                        Tank(TankBrainParams),
                    }
        };

        let expected = quote! {
                pub trait BrainParamsDynRes {
                    fn extract_enum_data(&self) -> BrainParams;
                }
                // impls for the enum variants
                impl BrainParamsDynRes for RoombaBrainParamsResource {
                    fn extract_enum_data(&self) -> BrainParams {
                        BrainParams::Roomba(self.extract())
                    }
                }
                impl BrainParamsDynRes for TankBrainParamsResource {
                    fn extract_enum_data(&self) -> BrainParams {
                        BrainParams::Tank(self.extract())
                    }
                }

                // the `AsGdRes` impl for the enum itself will be a `DynGd<Resource, dyn #{enum_name}DynRes>``
                impl AsGdRes for BrainParams {
                    type ResType = DynGd<Resource, dyn BrainParamsDynRes>;
                }

                // the `ExtractGd` impl for `DynGd<Resource, dyn #{enum_name}DynRes>` will `dyn_bind` the dyn compatible Resouce, and call `extract_enum_data` on to get back the enum variant
                impl ExtractGd for DynGd<Resource, dyn BrainParamsDynRes> {
                    type Extracted = BrainParams;
                    fn extract(&self) -> Self::Extracted {
                        self.dyn_bind().extract_enum_data()
                    }
                }

                #[derive(GodotClass)]
                #[class(tool, init, base=Resource)]
                pub struct BrainParamsResource {
                    // we will always add the `base: Base<Resource>` field to the generated struct,
                    // and always with the `#[base]` attribute
                    #[base]
                    base: Base<Resource>,

                    #[export]
                    pub brain_params: Option<DynGd<Resource, dyn BrainParamsDynRes>>,
                }
        };

        assert_eq!(expand_as_gd_res(input).to_string(), expected.to_string());
    }
}
