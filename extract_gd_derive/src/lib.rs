use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Lit, Meta, NestedMeta};

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
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let vis = &input.vis;

    // Look for #[extract_to = "Name"] or #[extract_to(Name)]
    let mut target_name: Option<syn::Ident> = None;
    for attr in &input.attrs {
        if attr.path.is_ident("extract_to") {
            match attr.parse_meta() {
                // name-value: #[extract_to = "Foo"]
                Ok(Meta::NameValue(nv)) if matches!(nv.lit, Lit::Str(_)) => {
                    if let Lit::Str(lit) = nv.lit {
                        target_name = Some(syn::Ident::new(&lit.value(), lit.span()));
                    }
                }
                // list: #[extract_to(Foo)]
                Ok(Meta::List(list)) => {
                    if let Some(NestedMeta::Meta(Meta::Path(path))) = list.nested.first() {
                        if let Some(ident) = path.get_ident() {
                            target_name = Some(ident.clone());
                        }
                    }
                }
                _ => {}
            }
        }
    }

    // Gather fields if we need to generate a new struct
    let (field_idents, field_types) = if target_name.is_some() {
        if let Data::Struct(data_struct) = &input.data {
            if let Fields::Named(named) = &data_struct.fields {
                named
                    .named
                    .iter()
                    .filter_map(|f| {
                        let skip = f
                            .attrs
                            .iter()
                            .any(|a| a.path.is_ident("extract_ignore") || a.path.is_ident("base"));
                        if skip {
                            None
                        } else {
                            let ident = f.ident.as_ref().unwrap();
                            let ty = &f.ty;
                            Some((ident.clone(), ty.clone()))
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

    // Generate the code
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
        // Fallback clone impl
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

    TokenStream::from(expanded)
}
