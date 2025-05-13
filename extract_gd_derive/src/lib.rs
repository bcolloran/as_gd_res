use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Lit, Meta};

/// A derive macro to implement `ExtractGd` and optionally generate an `Extracted` struct.
///
/// - `#[extract_to = "Name"]` on the struct tells the macro to:
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
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let vis = &input.vis;

    // Look for #[extract_to = "TargetName"] on the type
    let mut target_name: Option<syn::Ident> = None;
    for attr in &input.attrs {
        if attr.path.is_ident("extract_to") {
            if let Ok(Meta::NameValue(nv)) = attr.parse_meta() {
                if let Lit::Str(lit) = nv.lit {
                    target_name = Some(syn::Ident::new(&lit.value(), lit.span()));
                }
            }
        }
    }

    // If we have an extract_to, gather its fields (ignoring #[extract_ignore] and #[base])
    let (field_idents, field_types): (Vec<_>, Vec<_>) = if target_name.is_some() {
        match &input.data {
            Data::Struct(data_struct) => match &data_struct.fields {
                Fields::Named(named) => named
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
                    .unzip(),
                _ => (Vec::new(), Vec::new()),
            },
            _ => (Vec::new(), Vec::new()),
        }
    } else {
        (Vec::new(), Vec::new())
    };

    // Generate the output tokens
    let expanded = if let Some(extracted_struct_name) = target_name {
        // 1) Define the extracted struct
        // 2) Implement ExtractGd for the original type
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
        // Clone-based fallback impl
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
