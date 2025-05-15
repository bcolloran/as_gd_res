use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_quote, Data, DeriveInput, Fields};

#[cfg(test)]
mod tests;

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
                        a.path().is_ident("export")
                            || a.path().is_ident("init")
                            || a.path().is_ident("var")
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
                let dyn_trait = format_ident!("{}DynEnumResource", name);
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
