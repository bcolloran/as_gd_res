use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Expr, ExprLit, Fields, Ident, Lit, Meta};

pub(crate) fn derive_inner(input: DeriveInput) -> TokenStream {
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

    TokenStream::from(expanded)
}
