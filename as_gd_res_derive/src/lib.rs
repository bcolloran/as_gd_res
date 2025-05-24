use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_quote, Data, DeriveInput, Fields, Type};

#[cfg(test)]
mod tests;

/// A derive macro to emit a Godot-compatible resource struct + impls for a pure Rust struct.
#[proc_macro_derive(AsGdRes, attributes(export, init))]
pub fn as_gd_res_derive(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as DeriveInput);
    TokenStream::from(expand_as_gd_res(derive_input))
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
                // clone export/init attrs or inject #[export]
                let mut attrs = field
                    .attrs
                    .iter()
                    .filter(|a| a.path().is_ident("export") || a.path().is_ident("init"))
                    .cloned()
                    .collect::<Vec<_>>();
                if attrs.is_empty() {
                    attrs.push(parse_quote!(#[export]));
                }
                let ty = &field.ty;
                field_defs.push(quote! {
                    #(#attrs)*
                    pub #ident: <#ty as AsGdRes>::ResType,
                });
                extract_fields.push(quote! {
                    #ident: self.#ident.extract(),
                });
            }

            quote! {
                impl AsGdRes for #name {
                    type ResType = ::godot::obj::Gd<#res_name>;
                }

                #[derive(::godot::prelude::GodotClass)]
                #[class(tool,init,base=Resource)]
                pub struct #res_name {
                    #[base]
                    base: ::godot::obj::Base<::godot::classes::Resource>,
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
            // Determine valid shapes and collect invalid variants
            let mut unit_only = true;
            let mut single_tuple = true;
            let mut invalid = Vec::new();
            for var in data.variants.iter() {
                match &var.fields {
                    Fields::Unit => {}
                    Fields::Unnamed(u) if u.unnamed.len() == 1 => {
                        unit_only = false;
                    }
                    _ => {
                        unit_only = false;
                        single_tuple = false;
                        invalid.push(var.ident.to_string());
                    }
                }
            }

            if unit_only {
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
                let dyn_trait = format_ident!("{}ResourceExtractVariant", name);
                let mut variant_impls = Vec::new();
                for var in data.variants.iter() {
                    if let Fields::Unnamed(fields) = &var.fields {
                        if fields.unnamed.len() == 1 {
                            let var_ident = &var.ident;
                            let inner_ty = &fields.unnamed[0].ty;
                            let res_ident = match inner_ty {
                                Type::Path(type_path) => {
                                    let segment =
                                        type_path.path.segments.last().unwrap().ident.clone();
                                    format_ident!("{}Resource", segment)
                                }
                                _ => format_ident!("{}Resource", var_ident),
                            };
                            variant_impls.push(quote! {
                                #[godot_dyn]
                                impl #dyn_trait for #res_ident {
                                    fn extract_enum_variant(&self) -> #name {
                                        #name::#var_ident(self.extract())
                                    }
                                }
                            });
                        }
                    }
                }
                quote! {
                    pub trait #dyn_trait {
                        fn extract_enum_variant(&self) -> #name;
                    }

                    type #res_name = ::godot::obj::DynGd<::godot::classes::Resource, dyn #dyn_trait>;

                    impl AsGdRes for #name {
                        type ResType = #res_name;
                    }

                    impl ExtractGd for dyn #dyn_trait {
                        type Extracted = #name;
                        fn extract(&self) -> Self::Extracted {
                            self.extract_enum_variant()
                        }
                    }

                    #(#variant_impls)*
                }
            } else {
                let msg = format!(
                    "`derive(AsGdRes)` only supports unit enums or single-tuple enums. Unsupported variants: {}",
                    invalid.join(", ")
                );
                quote! {
                    compile_error!(#msg);
                }
            }
        }
        _ => quote! {
            compile_error!("AsGdRes derive only supports structs with named fields, enums with unit variants, or enums with single-tuple variants");
        }
        .into(),
    }
}
