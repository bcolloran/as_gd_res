use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Expr, ExprLit, Fields, Ident, Lit, Meta};

#[cfg(test)]
mod tests;

/// A derive macro to implement `ExtractGd` and optionally generate an `Extracted` struct.
///
/// - `#[extract_to = "Name"]` or `#[extract_to(Name)]` on the struct tells the macro to:
///   1. Declare `pub struct Name { ... }` with all non-ignored fields (ignores `#[extract_ignore]` and `#[base]`).
///   2. Implement `ExtractGd` on the original type, with `type Extracted = Name`, and an `extract` that
///      calls `.extract()` on each field.
/// - Without `#[extract_to]`, emits a clone-based impl:
///   ```ignore
///   impl ExtractGd for T where T: Clone {
///       type Extracted = T;
///       fn extract(&self) -> Self::Extracted { self.clone() }
///   }
///   ```
#[proc_macro_derive(ExtractGd, attributes(extract_to, extract_ignore, base))]
pub fn extract_gd_derive(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let expanded = derive_inner(derive_input);
    TokenStream::from(expanded)
}

fn derive_inner(input: DeriveInput) -> proc_macro2::TokenStream {
    // let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let vis = &input.vis;

    // Determine optional target name
    let mut target_name: Option<Ident> = None;
    for attr in &input.attrs {
        if attr.path().is_ident("extract_to") {
            // Parentheses style: #[extract_to(Name)]
            if let Ok(ident) = attr.parse_args::<Ident>() {
                target_name = Some(ident);
            }
            // Name-value style: #[extract_to = "Name"]
            else if let Meta::NameValue(nv) = &attr.meta {
                if let Expr::Lit(ExprLit {
                    lit: Lit::Str(lit_str),
                    ..
                }) = &nv.value
                {
                    target_name = Some(Ident::new(&lit_str.value(), lit_str.span()));
                }
            }
        }
    }

    // Collect fields for extracted struct, if any
    let (field_idents, field_types) = if target_name.is_some() {
        if let Data::Struct(data_struct) = &input.data {
            if let Fields::Named(named) = &data_struct.fields {
                named
                    .named
                    .iter()
                    .filter_map(|f| {
                        let skip = f.attrs.iter().any(|a| {
                            a.path().is_ident("extract_ignore") || a.path().is_ident("base")
                        });
                        if skip {
                            None
                        } else {
                            Some((f.ident.clone().unwrap(), f.ty.clone()))
                        }
                    })
                    .unzip()
            } else {
                (Vec::new(), Vec::new())
            }
        } else {
            (Vec::new(), Vec::new())
        }
    } else {
        (Vec::new(), Vec::new())
    };

    // Generate the output tokens
    let expanded = if let Some(extracted_struct_name) = target_name {
        quote! {
            #vis struct #extracted_struct_name {
                #(
                    pub #field_idents: <#field_types as ExtractGd>::Extracted,
                )*
            }

            impl ExtractGd for #name {
                type Extracted = #extracted_struct_name;
                fn extract(&self) -> Self::Extracted {
                    #extracted_struct_name {
                        #(
                            #field_idents: self.#field_idents.extract(),
                        )*
                    }
                }
            }
        }
    } else {
        quote! {
            impl ExtractGd for #name
            where #name: Clone
            {
                type Extracted = #name;
                fn extract(&self) -> Self::Extracted {
                    self.clone()
                }
            }
        }
    };

    expanded.into()
}

// mod tests {
//     use super::derive_inner;
//     use pretty_assertions::assert_eq;
//     use quote::quote;
//     use syn::parse_quote;

//     #[test]
//     fn test_1() {
//         let input: syn::DeriveInput = parse_quote! {
//                 pub enum BrainParams {
//                         Roomba(RoombaBrainParams),
//                         Tank(TankBrainParams),
//                     }
//         };

//         let expected = quote! {
//                 pub trait BrainParamsDynRes {
//                     fn extract_enum_data(&self) -> BrainParams;
//                 }
//                 impl BrainParamsDynRes for RoombaBrainParamsResource {
//                     fn extract_enum_data(&self) -> BrainParams {
//                         BrainParams::Roomba(self.extract())
//                     }
//                 }
//                 impl BrainParamsDynRes for TankBrainParamsResource {
//                     fn extract_enum_data(&self) -> BrainParams {
//                         BrainParams::Tank(self.extract())
//                     }
//                 }

//                 impl AsGdRes for BrainParams {
//                     type ResType = DynGd<Resource, dyn BrainParamsDynRes>;
//                 }

//                 impl ExtractGd for DynGd<Resource, dyn BrainParamsDynRes> {
//                     type Extracted = BrainParams;
//                     fn extract(&self) -> Self::Extracted {
//                         self.dyn_bind().extract_enum_data()
//                     }
//                 }

//                 #[derive(GodotClass)]
//                 #[class(tool, init, base=Resource)]
//                 pub struct BrainParamsResource {
//                     #[base]
//                     base: Base<Resource>,

//                     #[export]
//                     pub brain_params: Option<DynGd<Resource, dyn BrainParamsDynRes>>,
//                 }
//         };

//         assert_eq!(derive_inner(input).to_string(), expected.to_string());
//     }
// }
