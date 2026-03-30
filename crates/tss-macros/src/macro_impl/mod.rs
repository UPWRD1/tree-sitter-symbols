// Modified from https://medium.com/@alfred.weirich/the-rust-macro-system-part-1-an-introduction-to-attribute-macros-73c963fd63ea
// extern crate proc_macro;

use proc_macro2::Span;
use quote::{format_ident, quote, quote_spanned};
use syn::{token, Generics};

mod generate;
mod schema;

use crate::macro_impl::generate::DerivedType;

pub fn generate_nodes_impl(
    crate_name: syn::Ident,
    input: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    // Parse input struct and handle parse errors
    let the_enum: syn::ItemEnum = match syn::parse2::<syn::ItemEnum>(input) {
        Ok(the_enum) => the_enum,
        Err(e) => {
            return darling::Error::from(e).write_errors();
        }
    };

    // Retrieve the name of the enum
    let mut new_enum: syn::ItemEnum = the_enum.clone();

    let analysis: DerivedType;
    let mut accum: proc_macro2::TokenStream = quote! {};
    // Check if the enum has ANY variants
    if new_enum.variants.is_empty() {
        match generate::analyze(&crate_name.to_string()) {
            Ok(vm) => {
                analysis = vm;
            }
            Err(e) => {
                let e = e.to_string();
                let msg = format!("Error: {e}");
                let crate_span = crate_name.span();
                return quote_spanned! {crate_span=>
                    compile_error!(#msg);
                };
            }
        }
        for variant in &analysis.variants {
            let variant_ident = syn::Ident::new(&variant.variant_name, Span::mixed_site());
            let fields = if let Some(subtypes) = &variant.subtypes {
                let mut new_type = syn::ItemEnum {
                    attrs: the_enum.attrs.clone(),
                    vis: the_enum.vis.clone(),
                    enum_token: syn::Token![enum](Span::mixed_site()),
                    ident: variant_ident.clone(),
                    generics: Generics {
                        lt_token: None,
                        params: syn::punctuated::Punctuated::new(),
                        gt_token: None,
                        where_clause: None,
                    },
                    brace_token: token::Brace(Span::mixed_site()),
                    variants: syn::punctuated::Punctuated::new(),
                };
                for subtype in &subtypes.variants {
                    new_type.variants.push(syn::Variant {
                        ident: syn::Ident::new(&subtype.variant_name, Span::mixed_site()),
                        attrs: vec![],
                        fields: syn::Fields::Unit,
                        discriminant: None, //TODO Consider investigating if this can be the tree-sitter node ID
                    })
                }
                let subtype_enum = quote! {
                    #new_type
                };
                accum = quote! {
                    #subtype_enum

                    #accum
                };
                syn::Fields::Unnamed(syn::parse_quote! {(#variant_ident)})
            } else {
                syn::Fields::Unit
            };
            new_enum.variants.push(syn::Variant {
                ident: syn::Ident::new(&variant.variant_name, Span::mixed_site()),
                attrs: vec![],
                fields,
                discriminant: None, //TODO Consider investigating if this can be the tree-sitter node ID
            });
        }
    } else {
        return quote! {
            compile_error!("This macro only operates on enums with no variants");
        };
    }

    // Generate conversion functions
    let from_string = generate_from_string(&analysis, &new_enum);
    let display = generate_display(&analysis, &new_enum);

    // Generate the final enum and conversion functions
    quote! {
        #accum

        #new_enum

        #from_string

        #display
    }
}

fn generate_from_string(
    analysis: &DerivedType,
    new_enum: &syn::ItemEnum,
) -> proc_macro2::TokenStream {
    let variants = &analysis.variants;
    let enum_name = &new_enum.ident;
    let mut the_match: syn::ExprMatch = syn::parse_quote! {
        match s {

        }
    };
    for variant in variants {
        if let Some(subtypes) = &variant.subtypes {
            let variant_ident = format_ident!("{}", variant.variant_name);
            for sub in &subtypes.variants {
                let string_rep = &sub.original_name;
                let subtype_ident = format_ident!("{}", sub.variant_name);
                let arm: syn::Arm = syn::parse_quote! {
                    #string_rep => {return std::result::Result::Ok(#enum_name::#variant_ident(#variant_ident::#subtype_ident))},

                };
                the_match.arms.push(arm);
            }
        } else {
            let variant_ident = format_ident!("{}", variant.variant_name);
            let string_rep = &variant.original_name;
            let arm: syn::Arm = syn::parse_quote! {
                #string_rep => {return std::result::Result::Ok(#enum_name::#variant_ident)},
            };
            the_match.arms.push(arm);
        }
    }
    // Wildcard case _ => return err
    {
        let wildcard_arm: syn::Arm = syn::parse_quote! {
            err => {panic!("Unknown token name: '{err}'")},
        };
        the_match.arms.push(wildcard_arm);
    }

    quote! {
        impl std::str::FromStr for #enum_name {
            type Err = String;

            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                #the_match
            }
        }
    }
}

fn generate_display(analysis: &DerivedType, new_enum: &syn::ItemEnum) -> proc_macro2::TokenStream {
    let variants = &analysis.variants;
    let enum_name = &new_enum.ident;
    let mut the_match: syn::ExprMatch = syn::parse_quote! {
        match self {

        }
    };
    for variant in variants {
        let variant_ident = format_ident!("{}", variant.variant_name);
        if let Some(subtypes) = &variant.subtypes {
            for sub in &subtypes.variants {
                let string_rep = &sub.original_name;
                let subtype_ident = format_ident!("{}", sub.variant_name);
                let string_rep = match string_rep.as_str() {
                    "{" => "{{".to_string(),
                    "}" => "}}".to_string(),
                    _ => string_rep.to_string(),
                };
                let arm: syn::Arm = syn::parse_quote! {
                    Self::#variant_ident(#variant_ident::#subtype_ident) => { write!(f, #string_rep) },

                };

                the_match.arms.push(arm);
            }
        } else {
            let string_rep = &variant.original_name;
            let string_rep = match string_rep.as_str() {
                "{" => "{{".to_string(),
                "}" => "}}".to_string(),
                _ => string_rep.to_string(),
            };
            let arm: syn::Arm = syn::parse_quote! {
                Self::#variant_ident => { write!(f, #string_rep) },

            };

            the_match.arms.push(arm);
        }
    }
    quote! {
        impl std::fmt::Display for #enum_name {
           fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
               #the_match
           }
        }
    }
}
