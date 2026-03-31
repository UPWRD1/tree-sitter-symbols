use std::collections::HashSet;

use proc_macro2::Span;
use syn::{punctuated::Punctuated, token, Token};

use crate::macro_impl::schema::{ChildrenClass, NamedNodeType, NamedSubtype};
use quote::{format_ident, quote};

pub fn terminal(
    item_accumulator: &mut HashSet<syn::Item>,
    root_enum: &syn::ItemEnum,
    element: &NamedNodeType,
) {
    let terminal_struct = syn::ItemStruct {
        attrs: root_enum.attrs.clone(),
        vis: root_enum.vis.clone(),
        struct_token: syn::Token![struct](Span::mixed_site()),
        ident: element.rustified_name.clone(),
        generics: syn::Generics {
            lt_token: None,
            params: syn::punctuated::Punctuated::new(),
            gt_token: None,
            where_clause: None,
        },
        fields: syn::Fields::Unit,
        semi_token: Some(Token![;](Span::mixed_site())),
    };
    item_accumulator.insert(syn::Item::Struct(terminal_struct));
}

pub fn supertype_enum(
    item_accumulator: &mut HashSet<syn::Item>,
    root_enum: &syn::ItemEnum,
    element: &NamedNodeType,
    subtypes: &[NamedSubtype],
) {
    let supertype_ident = &element.rustified_name;
    let mut supertype_enum = syn::ItemEnum {
        attrs: root_enum.attrs.clone(),
        vis: root_enum.vis.clone(),
        enum_token: syn::Token![enum](Span::mixed_site()),
        ident: supertype_ident.clone(),
        generics: syn::Generics {
            lt_token: None,
            params: syn::punctuated::Punctuated::new(),
            gt_token: None,
            where_clause: None,
        },
        brace_token: token::Brace(Span::mixed_site()),
        variants: syn::punctuated::Punctuated::new(),
    };
    for subtype in subtypes {
        let subtype_ident = subtype.ident.clone();

        supertype_enum.variants.push(syn::Variant {
            ident: subtype_ident.clone(),
            attrs: vec![],
            fields: syn::Fields::Unnamed(syn::parse_quote! {(#subtype_ident)}),
            discriminant: None, //TODO Consider investigating if this can be the tree-sitter node ID
        });
    }
    item_accumulator.insert(syn::Item::Enum(supertype_enum));
}

pub fn fields_and_children(
    item_accumulator: &mut HashSet<syn::Item>,
    root_enum: &mut syn::ItemEnum,
    element: &NamedNodeType,
    fields: &[(String, ChildrenClass)],
    children_class: &ChildrenClass,
) {
    let element_ident = &element.rustified_name;
    let children_enum_ident = format_ident!("{}Child", element_ident);

    // let mut potential_children_item: Option<syn::Item> = None;

    let mut punct_fields: Punctuated<syn::Field, Token![,]> = Punctuated::new();

    for (field_name, field_children) in fields {
        let field_child_ident = format_ident!("{}{}", element_ident, field_name);
        let the_field = syn::Field {
            attrs: vec![],
            vis: syn::Visibility::Inherited,
            mutability: syn::FieldMutability::None,
            ident: Some(syn::Ident::new(field_name, Span::mixed_site())),
            colon_token: Some(Token![:](Span::mixed_site())),
            ty: syn::Type::Verbatim(field_children.to_type(
                root_enum,
                &field_child_ident,
                item_accumulator,
            )),
        };
        punct_fields.push(the_field);
    }

    {
        let field_name = match children_class {
            ChildrenClass::Single(child) => {
                syn::Ident::new(&child.subchild_type_name, Span::mixed_site())
            }
            ChildrenClass::MaybeSingle(named_subtype) => {
                format_ident!("maybe_{}", named_subtype.subchild_type_name)
            }
            _ => format_ident!("children"),
        };
        let child_field = syn::Field {
            attrs: vec![],
            vis: syn::Visibility::Inherited,
            mutability: syn::FieldMutability::None,
            ident: Some(field_name),
            colon_token: Some(Token![:](Span::mixed_site())),
            ty: syn::Type::Verbatim(children_class.to_type(
                root_enum,
                &children_enum_ident,
                item_accumulator,
            )),
        };
        punct_fields.push(child_field);
    }

    let element_struct = syn::ItemStruct {
        attrs: root_enum.attrs.clone(),
        vis: root_enum.vis.clone(),
        struct_token: syn::Token![struct](Span::mixed_site()),
        ident: element_ident.clone(),
        generics: syn::Generics {
            lt_token: None,
            params: syn::punctuated::Punctuated::new(),
            gt_token: None,
            where_clause: None,
        },
        fields: syn::Fields::Named(syn::FieldsNamed {
            brace_token: token::Brace(Span::mixed_site()),
            named: punct_fields,
        }),
        semi_token: Some(Token![;](Span::mixed_site())),
    };

    item_accumulator.insert(syn::Item::Struct(element_struct));
}

pub fn fields_only(
    item_accumulator: &mut HashSet<syn::Item>,
    root_enum: &mut syn::ItemEnum,
    element: &NamedNodeType,
    fields: &[(String, ChildrenClass)],
) {
    let element_ident = &element.rustified_name;
    let children_enum_ident = format_ident!("{}Child", element_ident);

    // let mut potential_children_item: Option<syn::Item> = None;

    let mut punct_fields: Punctuated<syn::Field, Token![,]> = Punctuated::new();

    for (field_name, field_children) in fields {
        let the_field = syn::Field {
            attrs: vec![],
            vis: syn::Visibility::Inherited,
            mutability: syn::FieldMutability::None,
            ident: Some(syn::Ident::new(field_name, Span::mixed_site())),
            colon_token: Some(Token![:](Span::mixed_site())),
            ty: syn::Type::Verbatim(field_children.to_type(
                root_enum,
                &children_enum_ident,
                item_accumulator,
            )),
        };
        punct_fields.push(the_field);
    }

    let element_struct = syn::ItemStruct {
        attrs: root_enum.attrs.clone(),
        vis: root_enum.vis.clone(),
        struct_token: syn::Token![struct](Span::mixed_site()),
        ident: element_ident.clone(),
        generics: syn::Generics {
            lt_token: None,
            params: syn::punctuated::Punctuated::new(),
            gt_token: None,
            where_clause: None,
        },
        fields: syn::Fields::Named(syn::FieldsNamed {
            brace_token: token::Brace(Span::mixed_site()),
            named: punct_fields,
        }),
        semi_token: Some(Token![;](Span::mixed_site())),
    };

    item_accumulator.insert(syn::Item::Struct(element_struct));
}

pub fn children_only(
    item_accumulator: &mut HashSet<syn::Item>,
    root_enum: &mut syn::ItemEnum,
    element: &NamedNodeType,
    children_class: &ChildrenClass,
) {
    let element_ident = &element.rustified_name;
    let children_enum_ident = format_ident!("{}Child", element_ident);

    let mut punct_fields: Punctuated<syn::Field, Token![,]> = Punctuated::new();

    {
        let field_name = match children_class {
            ChildrenClass::Single(child) => {
                syn::Ident::new(&child.subchild_type_name, Span::mixed_site())
            }
            ChildrenClass::MaybeSingle(named_subtype) => {
                format_ident!("maybe_{}", named_subtype.subchild_type_name)
            }
            _ => format_ident!("children"),
        };
        let child_field = syn::Field {
            attrs: vec![],
            vis: syn::Visibility::Inherited,
            mutability: syn::FieldMutability::None,
            ident: Some(field_name),
            colon_token: Some(Token![:](Span::mixed_site())),
            ty: syn::Type::Verbatim(children_class.to_type(
                root_enum,
                &children_enum_ident,
                item_accumulator,
            )),
        };
        punct_fields.push(child_field);
    }

    let element_struct = syn::ItemStruct {
        attrs: root_enum.attrs.clone(),
        vis: root_enum.vis.clone(),
        struct_token: syn::Token![struct](Span::mixed_site()),
        ident: element_ident.clone(),
        generics: syn::Generics {
            lt_token: None,
            params: syn::punctuated::Punctuated::new(),
            gt_token: None,
            where_clause: None,
        },
        fields: syn::Fields::Named(syn::FieldsNamed {
            brace_token: token::Brace(Span::mixed_site()),
            named: punct_fields,
        }),
        semi_token: Some(Token![;](Span::mixed_site())),
    };
    item_accumulator.insert(syn::Item::Struct(element_struct));
}

impl ChildrenClass {
    fn to_type(
        &self,
        root_enum: &mut syn::ItemEnum,
        children_enum_ident: &syn::Ident,
        item_accumulator: &mut HashSet<syn::Item>,
    ) -> proc_macro2::TokenStream {
        match self {
            ChildrenClass::Single(child) => {
                let ident = child.ident.clone();
                quote! {#ident}
            }
            ChildrenClass::MaybeSingle(child) => {
                let ident = child.ident.clone();
                quote! {std::option::Option<#ident>}
            }
            ChildrenClass::Choice(children) => {
                children_choice(item_accumulator, root_enum, children_enum_ident, children);
                quote! {#children_enum_ident}
            }
            ChildrenClass::MaybeChoice(children) => {
                children_choice(item_accumulator, root_enum, children_enum_ident, children);
                quote! {std::option::Option<#children_enum_ident>}
            }
            ChildrenClass::Repeated(child) => {
                let ident = child.ident.clone();
                quote! {std::vec::Vec<#ident>}
            }
            ChildrenClass::MaybeRepeated(child) => {
                let ident = child.ident.clone();
                quote! {std::option::Option<std::vec::Vec<#ident>>}
            }
            ChildrenClass::List(children) => {
                children_choice(item_accumulator, root_enum, children_enum_ident, children);
                quote! {std::vec::Vec<#children_enum_ident>}
            }
            ChildrenClass::MaybeList(children) => {
                children_choice(item_accumulator, root_enum, children_enum_ident, children);
                quote! {std::option::Option<std::vec::Vec<#children_enum_ident>>}
            }
        }
    }
}

fn children_choice(
    item_accumulator: &mut HashSet<syn::Item>,
    root_enum: &syn::ItemEnum,
    children_enum_ident: &syn::Ident,
    children: &Vec<NamedSubtype>,
) {
    let mut children_enum = syn::ItemEnum {
        attrs: root_enum.attrs.clone(),
        vis: root_enum.vis.clone(),
        enum_token: Token![enum](Span::mixed_site()),
        ident: children_enum_ident.clone(),
        generics: syn::Generics {
            lt_token: None,
            params: syn::punctuated::Punctuated::new(),
            gt_token: None,
            where_clause: None,
        },
        brace_token: token::Brace(Span::mixed_site()),
        variants: Punctuated::new(),
    };
    for child in children {
        let ident = child.ident.clone();
        let child_variant = syn::Variant {
            attrs: vec![],
            ident: ident.clone(),
            fields: syn::Fields::Unnamed(syn::parse_quote! {(#ident)}),
            discriminant: None,
        };
        children_enum.variants.push(child_variant);
    }
    item_accumulator.insert(syn::Item::Enum(children_enum));
}
