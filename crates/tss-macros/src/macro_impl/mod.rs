// Modified from https://medium.com/@alfred.weirich/the-rust-macro-system-part-1-an-introduction-to-attribute-macros-73c963fd63ea
// extern crate proc_macro;

use crate::macro_impl::schema::NamedNodeType;
use quote::{quote, quote_spanned};
use std::collections::HashSet;

mod generate;
mod parse;
mod schema;

pub fn macro_impl(
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

    let mut analysis: HashSet<NamedNodeType> = HashSet::default();
    // Check if the enum has ANY variants
    let accum = if the_enum.variants.is_empty() {
        generate_types(crate_name, the_enum, &mut analysis)
    } else {
        quote! {
            compile_error!("This macro only operates on enums with no variants");
        }
    };

    // Generate conversion functions
    // let from_string = generate_from_string(&analysis, &the_enum);
    // let display = generate_display(&analysis, &the_enum);

    // Generate the final enum and conversion functions
    quote! {
        #accum



        // #from_string

        // #display
    }
}

fn generate_types(
    crate_name: syn::Ident,
    base_enum: syn::ItemEnum,
    analysis: &mut HashSet<NamedNodeType>,
) -> proc_macro2::TokenStream {
    let mut root_enum = base_enum.clone();
    match parse::analyze(&crate_name.to_string()) {
        Ok(vm) => {
            *analysis = vm;
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

    let mut seen: HashSet<syn::Ident> = HashSet::default();

    let items: Vec<proc_macro2::TokenStream> = analysis
        .iter()
        .map(|element| {
            dbg!(&element);
            seen.insert(element.rustified_name.clone());
            let newtype = match &element.class {
                schema::NodeClass::Terminal => generate::terminal(&mut root_enum, element),
                schema::NodeClass::FieldsOnly { fields } => {
                    generate::fields_only(&mut root_enum, element, fields)
                }
                schema::NodeClass::FieldsAndChildren { fields, children } => {
                    generate::fields_and_children(&mut root_enum, element, fields, children)
                }
                schema::NodeClass::ChildrenOnly { children } => {
                    generate::children_only(&mut root_enum, element, children)
                }
                schema::NodeClass::SuperType { subtypes } => {
                    generate::supertype_enum(&mut root_enum, element, subtypes)
                }
            };
            quote! {
                #newtype
            }
        })
        .collect();
    quote! {
        #(#items)*
    }
}

// fn generate_from_string(
//     analysis: &,
//     new_enum: &syn::ItemEnum,
// ) -> proc_macro2::TokenStream {
//     let variants = &analysis.variants;
//     let enum_name = &new_enum.ident;
//     let mut the_match: syn::ExprMatch = syn::parse_quote! {
//         match s {

//         }
//     };
//     for variant in variants {
//         if let Some(subtypes) = &variant.subtypes {
//             let variant_ident = format_ident!("{}", variant.variant_name);
//             for sub in &subtypes.variants {
//                 let string_rep = &sub.original_name;
//                 let subtype_ident = format_ident!("{}", sub.variant_name);
//                 let arm: syn::Arm = syn::parse_quote! {
//                     #string_rep => {return std::result::Result::Ok(#enum_name::#variant_ident(#variant_ident::#subtype_ident))},

//                 };
//                 the_match.arms.push(arm);
//             }
//         } else {
//             let variant_ident = format_ident!("{}", variant.variant_name);
//             let string_rep = &variant.original_name;
//             let arm: syn::Arm = syn::parse_quote! {
//                 #string_rep => {return std::result::Result::Ok(#enum_name::#variant_ident)},
//             };
//             the_match.arms.push(arm);
//         }
//     }
//     // Wildcard case _ => return err
//     {
//         let wildcard_arm: syn::Arm = syn::parse_quote! {
//             err => {panic!("Unknown token name: '{err}'")},
//         };
//         the_match.arms.push(wildcard_arm);
//     }

//     quote! {
//         impl std::str::FromStr for #enum_name {
//             type Err = String;

//             fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
//                 #the_match
//             }
//         }
//     }
// }

// fn generate_display(analysis: &DerivedType, new_enum: &syn::ItemEnum) -> proc_macro2::TokenStream {
//     let variants = &analysis.variants;
//     let enum_name = &new_enum.ident;
//     let mut the_match: syn::ExprMatch = syn::parse_quote! {
//         match self {

//         }
//     };
//     for variant in variants {
//         let variant_ident = format_ident!("{}", variant.variant_name);
//         if let Some(subtypes) = &variant.subtypes {
//             for sub in &subtypes.variants {
//                 let string_rep = &sub.original_name;
//                 let subtype_ident = format_ident!("{}", sub.variant_name);
//                 let string_rep = match string_rep.as_str() {
//                     "{" => "{{".to_string(),
//                     "}" => "}}".to_string(),
//                     _ => string_rep.to_string(),
//                 };
//                 let arm: syn::Arm = syn::parse_quote! {
//                     Self::#variant_ident(#variant_ident::#subtype_ident) => { write!(f, #string_rep) },

//                 };

//                 the_match.arms.push(arm);
//             }
//         } else {
//             let string_rep = &variant.original_name;
//             let string_rep = match string_rep.as_str() {
//                 "{" => "{{".to_string(),
//                 "}" => "}}".to_string(),
//                 _ => string_rep.to_string(),
//             };
//             let arm: syn::Arm = syn::parse_quote! {
//                 Self::#variant_ident => { write!(f, #string_rep) },

//             };

//             the_match.arms.push(arm);
//         }
//     }
//     quote! {
//         impl std::fmt::Display for #enum_name {
//            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//                #the_match
//            }
//         }
//     }
// }
