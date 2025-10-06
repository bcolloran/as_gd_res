use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, Type, parse_quote};

#[cfg(test)]
mod tests;

/// A derive macro to emit a Godot-compatible resource struct + impls for a pure Rust struct.
#[proc_macro_derive(AsGdRes, attributes(export, init, var, as_gd_res))]
pub fn as_gd_res_derive(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as DeriveInput);
    TokenStream::from(expand_as_gd_res(derive_input))
}

fn expand_as_gd_res(mut input: DeriveInput) -> proc_macro2::TokenStream {
    // Detect #[as_gd_res(post_init = METHOD)] on the struct
    let mut post_init_method: Option<proc_macro2::Ident> = None;
    let mut new_attrs = Vec::new();
    for attr in input.attrs.into_iter() {
        if attr.path().is_ident("as_gd_res") {
            // Parse inner tokens for post_init
            if let syn::Meta::List(meta_list) = &attr.meta {
                let mut iter = meta_list.tokens.clone().into_iter();
                while let Some(tok) = iter.next() {
                    if let TokenTree::Ident(id) = &tok {
                        if id == "post_init" {
                            // skip '='
                            let _ = iter.next();
                            if let Some(TokenTree::Ident(method_ident)) = iter.next() {
                                post_init_method = Some(method_ident.clone());
                                break;
                            }
                        }
                    }
                }
            }
            // Do not propagate this attribute
        } else {
            new_attrs.push(attr);
        }
    }
    input.attrs = new_attrs;

    if !input.generics.params.is_empty() {
        return quote! { compile_error!("`derive(AsGdRes)` does not support generics"); };
    }
    let name = input.ident.clone();
    let res_name = format_ident!("{}Resource", name);

    match input.data {
        Data::Struct(data) => {
            if !matches!(data.fields, Fields::Named(_)) {
                return quote! { compile_error!("`derive(AsGdRes)` only supports structs with named fields"); };
            }
            let mut defs = Vec::new();
            let mut extracts = Vec::new();
            // For init code: collect (ident, option<Lit>)
            let mut init_assigns = Vec::new();
            for field in data.fields.iter() {
                if let Some(ident) = &field.ident {
                    // Filter attrs: for post_init, drop init attrs; else keep export/init/var
                    let mut attrs = field
                        .attrs
                        .iter()
                        .filter(|a| {
                            let is_export = a.path().is_ident("export");
                            let is_var = a.path().is_ident("var");
                            let is_init = a.path().is_ident("init");
                            if post_init_method.is_some() {
                                is_export || is_var
                            } else {
                                is_export || is_var || is_init
                            }
                        })
                        .cloned()
                        .collect::<Vec<_>>();
                    if attrs.is_empty() {
                        attrs.push(parse_quote!(#[export]));
                    }
                    let ty = &field.ty;
                    defs.push(quote! {
                        #(#attrs)*
                        pub #ident: <#ty as ::as_gd_res::AsGdRes>::ResType,
                    });
                    extracts.push(quote! {
                        #ident: self.#ident.extract(),
                    });
                    // For init assignments if post_init
                    if post_init_method.is_some() {
                        // Find init attr in original field.attrs
                        let mut init_value = None;
                        for a in field.attrs.iter() {
                            if a.path().is_ident("init") {
                                if let syn::Meta::List(meta_list) = &a.meta {
                                    let mut token_iter = meta_list.tokens.clone().into_iter();
                                    while let Some(tok2) = token_iter.next() {
                                        if let TokenTree::Ident(id2) = &tok2 {
                                            if id2 == "val" {
                                                // skip '='
                                                let _ = token_iter.next();
                                                if let Some(next_tok) = token_iter.next() {
                                                    if let TokenTree::Literal(lit) = &next_tok {
                                                        init_value = Some(quote! { #lit });
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if let Some(val) = init_value {
                            init_assigns.push(quote! { #ident: #val.into(), });
                        } else {
                            init_assigns.push(quote! { #ident: Default::default(), });
                        }
                    }
                }
            }
            // Determine class attribute
            let class_attr = if post_init_method.is_some() {
                quote! { #[class(tool,base = Resource)] }
            } else {
                quote! { #[class(tool,init,base = Resource)] }
            };

            let mut expanded = quote! {
                impl ::as_gd_res::AsGdRes for #name {
                    type ResType = ::godot::prelude::OnEditor<::godot::obj::Gd<#res_name>>;
                }
                impl ::as_gd_res::AsGdResOpt for #name {
                    type GdOption = Option<::godot::obj::Gd<#res_name>>;
                }
                impl ::as_gd_res::AsGdResArray for #name {
                    type GdArray = ::godot::prelude::Array<::godot::obj::Gd<#res_name>>;
                }

                #[derive(::godot::prelude::GodotClass)]
                #class_attr
                pub struct #res_name {
                    #[base]
                    base: ::godot::obj::Base<::godot::classes::Resource>,
                    #(#defs)*
                }

                impl ::as_gd_res::ExtractGd for #res_name {
                    type Extracted = #name;
                    fn extract(&self) -> Self::Extracted {
                        use ::as_gd_res::ExtractGd;
                        Self::Extracted {
                            #(#extracts)*
                        }
                    }
                }
            };

            // If post_init, append IResource impl
            if let Some(method_ident) = post_init_method {
                expanded.extend(quote! {
                    #[godot_api]
                    impl ::godot::prelude::IResource for #res_name {
                        fn init(base: ::godot::prelude::Base<::godot::prelude::Resource>) -> Self {
                            let mut res = Self {
                                base,
                                #(#init_assigns)*
                            };
                            res.#method_ident();
                            res
                        }
                    }
                });
            }
            expanded
        }
        Data::Enum(data) => {
            // Existing enum logic unchanged
            let all_unit = data
                .variants
                .iter()
                .all(|v| matches!(&v.fields, Fields::Unit));
            let all_tuple1 = data
                .variants
                .iter()
                .all(|v| matches!(&v.fields, Fields::Unnamed(u) if u.unnamed.len()==1));

            if all_unit {
                quote! {
                    compile_error!(
                        "`derive(AsGdRes)` only supports enums with single-tuple variants, not unit variants. Did you mean to use `derive(AsGdEnumSimple)`?"
                    );
                }
            } else if all_tuple1 {
                let dyn_trait = format_ident!("{}ResourceExtractVariant", name);
                let mut variant_impls = Vec::new();
                for var in &data.variants {
                    if let Fields::Unnamed(fields) = &var.fields {
                        let var_ident = &var.ident;
                        let ty = &fields.unnamed[0].ty;
                        let variant_res = match ty {
                            Type::Path(tp) => {
                                let seg = tp.path.segments.last().unwrap().ident.clone();
                                format_ident!("{}Resource", seg)
                            }
                            _ => format_ident!("{}Resource", var_ident),
                        };
                        variant_impls.push(quote! {
                            {
                                use ::godot::prelude::godot_dyn;
                            #[godot_dyn]
                            impl #dyn_trait for #variant_res {
                                fn extract_enum_variant(&self) -> #name {
                                    #name::#var_ident(self.extract())
                                }
                            }
                            }
                        });
                    }
                }

                quote! {
                    pub trait #dyn_trait {
                        fn extract_enum_variant(&self) -> #name;
                    }

                    type #res_name = ::godot::obj::DynGd<::godot::classes::Resource, dyn #dyn_trait>;

                    impl ::as_gd_res::AsGdRes for #name {
                        type ResType = ::godot::prelude::OnEditor<#res_name>;
                    }
                    impl ::as_gd_res::AsGdResOpt for #name {
                        type GdOption = Option<#res_name>;
                    }
                    impl ::as_gd_res::AsGdResArray for #name {
                        type GdArray = ::godot::prelude::Array<#res_name>;
                    }

                    impl ::as_gd_res::ExtractGd for dyn #dyn_trait {
                        type Extracted = #name;
                        fn extract(&self) -> Self::Extracted {
                            self.extract_enum_variant()
                        }
                    }

                    #(#variant_impls)*
                }
            } else {
                let invalid = data
                    .variants
                    .iter()
                    .filter_map(|v| match &v.fields {
                        Fields::Unnamed(u) if u.unnamed.len() == 1 => None,
                        Fields::Unit => None,
                        _ => Some(v.ident.to_string()),
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                let msg = format!(
                    "`derive(AsGdRes)` only supports unit enums or single-tuple enums. Unsupported variants: {}",
                    invalid
                );
                quote! { compile_error!(#msg); }
            }
        }
        _ => quote! {
            compile_error!(
                "`derive(AsGdRes)` only supports structs with named fields, enums with unit variants, or enums with single-tuple variants"
            );
        },
    }
}
