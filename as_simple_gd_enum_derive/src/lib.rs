use proc_macro::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Data, DeriveInput, Fields};

#[cfg(test)]
mod tests;

/// `#[derive(AsGdEnumSimple)]` only works on enums where all variants are unit variants (i.e. no associated data)
///
/// In any other case, the macro should emit an error saying that this conditions have not been met
///
/// There are limitations upstream in *godot-rust* (or really: in Godot itself) that prevent the representation of certain types. You'll need work arounds in at least these cases:
/// - `Option<{enum types}>`: If you want an "optional" enum, include a `None` variant in the enum itself, and set that as the default value.
/// - `Array<{enum types}>` are also not supported
#[proc_macro_derive(AsGdEnumSimple, attributes(export, init))]
pub fn as_gd_res_derive(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as DeriveInput);
    TokenStream::from(expand_as_gd_res(derive_input))
}

/// For an enum with _only_ unit variants, returns a token stream that:
/// - creates a version of the enum named `{original_enum_name}AsGdEnum` with the same variants as the original enum.
/// - prepends these derives to the to new enum:
/// ```
/// #[derive(GodotConvert, Var, Export, Clone, Copy, Debug, PartialEq, Eq)]
/// #[godot(via = GString)]
/// ```
/// - impls `Into` in both directions between the new and preexisting enum
/// - impls AsGdEnumSimple for the existing enum, with `AsGdEnumSimple<GdEnumType= {the new enum type}>`
/// - impls ExtractGd for the new enum by way of `.into()`
/// - impls Default for the new enum by way of `default().into()` from the existing enum type (the existing enum must already impl Default)

fn expand_as_gd_res(input: DeriveInput) -> proc_macro2::TokenStream {
    if !input.generics.params.is_empty() {
        return quote! { compile_error!("`derive(AsGdEnumSimple)` does not support generics"); };
    }
    let original_name = input.ident;
    let res_name = format_ident!("{}AsGdEnum", original_name);
    let mod_name = format_ident!("mod_{}", res_name.to_string().to_lowercase());

    match input.data {
        Data::Enum(data) => {
            // collect unit‐only variants and any that have data
            let mut unit_variants = Vec::new();
            let mut bad = Vec::new();
            for v in data.variants.iter() {
                match &v.fields {
                    Fields::Unit => {
                        unit_variants.push(&v.ident);
                    }
                    Fields::Unnamed(fields) => {
                        // e.g. `Fire(u32)` or if many: `Foo(u32, f32)`
                        let ty_list = fields
                            .unnamed
                            .iter()
                            .map(|f| f.ty.to_token_stream().to_string())
                            .collect::<Vec<_>>()
                            .join(", ");
                        bad.push(format!("{}({})", v.ident, ty_list));
                    }
                    Fields::Named(fields) => {
                        // e.g. `Bar{x: i32, y: i32}`
                        let nm_list = fields
                            .named
                            .iter()
                            .map(|f| {
                                let name = f.ident.as_ref().unwrap();
                                let ty = f.ty.to_token_stream().to_string();
                                format!("{}: {}", name, ty)
                            })
                            .collect::<Vec<_>>()
                            .join(", ");
                        bad.push(format!("{}{{{}}}", v.ident, nm_list));
                    }
                }
            }

            if !bad.is_empty() {
                let list = bad.join(", ");
                let msg = format!(
                    "`derive(AsGdEnumSimple)` only supports unit enums. Unsupported variants: {}.\nDid you mean to derive `AsGdRes`?",
                    list
                );
                return quote! {
                    compile_error!(#msg);
                };
            }

            // all‐unit case ⇒ emit the “AsGdEnum” + trait impls
            quote! {
                pub use #mod_name::*;
                mod  #mod_name {
                use super:: #original_name ;
                use ::godot::prelude::GString;

                #[derive(::godot::prelude::GodotConvert, ::godot::prelude::Var, ::godot::prelude::Export, Clone, Copy, Debug, PartialEq, Eq)]
                #[godot(via = GString)]
                pub enum #res_name {
                    #( #unit_variants , )*
                }

                impl ::as_gd_res::AsGdEnumSimple for #original_name {
                    type GdEnumType = #res_name;
                }

                impl ::as_gd_res::ExtractGd for #res_name {
                    type Extracted = #original_name;
                    fn extract(&self) -> Self::Extracted {
                        (*self).into()
                    }
                }

                impl From<#original_name> for #res_name {
                    fn from(value: #original_name) -> #res_name {
                        match value {
                            #( #original_name::#unit_variants => #res_name::#unit_variants , )*
                        }
                    }
                }

                impl From<#res_name> for #original_name {
                    fn from(value: #res_name) -> #original_name {
                        match value {
                            #( #res_name::#unit_variants => #original_name::#unit_variants , )*
                        }
                    }
                }

                impl Default for #res_name {
                    fn default() -> Self {
                        #original_name::default().into()
                    }
                }}
            }
        }
        // structs or unions are always wrong
        Data::Struct(_) | Data::Union(_) => {
            quote! {
                compile_error!(
                    "AsGdEnumSimple derive only supports enums with unit variants, not structs. Did you mean to derive `AsGdRes`?"
                );
            }
        }
    }
}
