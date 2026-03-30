// Modified from https://medium.com/@alfred.weirich/the-rust-macro-system-part-1-an-introduction-to-attribute-macros-73c963fd63ea
extern crate proc_macro;
// use darling::{ast::NestedMeta, FromMeta};
use proc_macro2::Span;
use quote::{format_ident, quote};

mod generate;
mod schema;

pub fn generate_nodes(
    crate_name: String,
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

    let variant_map: Vec<(String, String)>;
    // Check if the enum has ANY variants
    if new_enum.variants.is_empty() {
        match generate::generate(&crate_name) {
            Ok(vm) => {
                variant_map = vm;
            }
            Err(e) => {
                let e = e.to_string();
                let msg = format!("Error generating variants: {e}");
                return quote! {
                    compile_error!(#msg);
                };
            }
        }
        for (_, variant_name_string) in &variant_map {
            new_enum.variants.push(syn::Variant {
                ident: syn::Ident::new(variant_name_string, Span::call_site()),
                attrs: vec![],
                fields: syn::Fields::Unit,
                discriminant: None, //TODO Consider investigating if this can be the tree-sitter node ID
            });
        }
    } else {
        return quote! {
            compile_error!("This macro only operates on enums with no variants");
        };
    }

    // Generate conversion functions
    let (from_string) = generate_from_string(&variant_map, &new_enum);
    let (display) = generate_display(&variant_map, &new_enum);

    // Collect and process any errors encountered during field processing
    // let all_errors: Vec<proc_macro2::TokenStream> = [getter_errors, setter_errors].concat();
    // if !all_errors.is_empty() {
    //     return quote! { #(#all_errors)* };
    // }

    // Step 6: Generate the modified struct and any additional trait implementations
    quote! {
        #new_enum

        #from_string

        #display
    }
}

fn generate_from_string(
    variant_map: &[(String, String)],
    new_enum: &syn::ItemEnum,
) -> proc_macro2::TokenStream {
    let enum_name = &new_enum.ident;
    let mut the_match: syn::ExprMatch = syn::parse_quote! {
        match s {

        }
    };
    for (string_rep, variant) in variant_map {
        let variant_ident = format_ident!("{}", variant);
        let arm: syn::Arm = syn::parse_quote! {
            #string_rep => {return std::result::Result::Ok(#enum_name::#variant_ident)},

        };

        the_match.arms.push(arm);
    }
    // Wildcard case _ => return err
    {
        let wildcard_arm: syn::Arm = syn::parse_quote! {
            err => {return std::result::Result::Err(format!("Unknown token name: '{err}'"))},
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

fn generate_display(
    variant_map: &[(String, String)],
    new_enum: &syn::ItemEnum,
) -> proc_macro2::TokenStream {
    let enum_name = &new_enum.ident;
    let mut the_match: syn::ExprMatch = syn::parse_quote! {
        match self {

        }
    };
    for (string_rep, variant) in variant_map {
        let variant_ident = format_ident!("{}", variant);
        let arm: syn::Arm = syn::parse_quote! {
            #variant_ident => {write!(f, #string_rep)},

        };

        the_match.arms.push(arm);
    }
    // Wildcard case _ => return err
    {
        let wildcard_arm: syn::Arm = syn::parse_quote! {
        err => {return panic!("Unknown token name: '{err}'")},        };
        the_match.arms.push(wildcard_arm);
    }

    quote! {
        impl std::fmt::Display for #enum_name {
           fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
               #the_match
           }
        }
    }
}
