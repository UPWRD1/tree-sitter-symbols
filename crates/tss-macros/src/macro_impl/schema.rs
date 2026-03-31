use std::collections::HashMap;

use proc_macro2::Span;
/// Schema for node-types.json, which is deserialized and transformed into enums
use serde::{Deserialize, Serialize};
use syn::Ident;

use crate::macro_impl::parse::mangle_node_name;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeType {
    pub children: Option<Children>,
    pub extra: Option<bool>,
    pub fields: Option<HashMap<String, Children>>,
    pub named: bool,
    pub root: Option<bool>,
    pub subtypes: Option<Vec<Subtype>>,
    #[allow(clippy::struct_field_names)]
    #[serde(rename = "type")]
    pub node_type_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChildrenClass {
    // multiple == false; required == true; len == 1
    Single(NamedSubtype),

    // multiple == false; required == false; len == 1
    MaybeSingle(NamedSubtype),

    // multiple == false; required = true; len > 1
    Choice(Vec<NamedSubtype>),

    // multiple == false; required = false; len > 1
    MaybeChoice(Vec<NamedSubtype>),

    // multiple == true; required == true; len == 1
    Repeated(NamedSubtype),

    // multiple == true; required == false; len == 1
    MaybeRepeated(NamedSubtype),

    // multiple == true; required = true; len > 1
    List(Vec<NamedSubtype>),

    // multiple == true; required = false; len > 1
    MaybeList(Vec<NamedSubtype>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeClass {
    Terminal, // No children, fields, or subtypes, likely named == false. Unit struct
    FieldsOnly {
        // Has fields. Becomes a struct
        fields: Vec<(String, ChildrenClass)>,
    },
    FieldsAndChildren {
        // Has fields and children. Will become two types:
        // 1. A wrapper struct that conatains fields and
        // 2. An enum containing the children
        fields: Vec<(String, ChildrenClass)>,
        children: ChildrenClass,
    },
    ChildrenOnly {
        children: ChildrenClass,
    },
    SuperType {
        subtypes: Vec<NamedSubtype>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NamedNodeType {
    pub named: bool,
    pub node_type_name: String,
    pub rustified_name: Ident,
    pub class: NodeClass,
}

impl NodeType {
    pub fn class(self) -> NodeClass {
        match (self.children, self.fields, self.subtypes) {
            (None, None, None) => NodeClass::Terminal,
            (None, None, Some(subtypes)) => NodeClass::SuperType {
                subtypes: subtypes.into_iter().map(|c| c.name()).collect(),
            },
            (None, Some(fields), None) if fields.is_empty() => NodeClass::Terminal,
            (None, Some(fields), None) => NodeClass::FieldsOnly {
                fields: fields.into_iter().map(|(k, f)| (k, f.class())).collect(),
            },
            (Some(children), None, None) => NodeClass::ChildrenOnly {
                children: children.class(),
            },

            (Some(children), Some(fields), None) if fields.is_empty() => NodeClass::ChildrenOnly {
                children: children.class(),
            },
            (Some(children), Some(fields), None) => NodeClass::FieldsAndChildren {
                fields: fields.into_iter().map(|(k, f)| (k, f.class())).collect(),
                children: children.class(),
            },

            n => panic!("Invalid node: {n:?}"),
        }
    }
    pub fn name(mut self, name: String) -> NamedNodeType {
        let node_type_name = std::mem::take(&mut self.node_type_name);
        let named = self.named;
        let class = self.class();
        NamedNodeType {
            named,
            node_type_name,
            rustified_name: syn::Ident::new(&name, Span::mixed_site()),
            class,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Children {
    pub multiple: bool,
    pub required: bool,
    pub types: Vec<Subtype>,
}

impl Children {
    pub fn class(self) -> ChildrenClass {
        match (self.multiple, self.required, self.types.len()) {
            // multiple == false; required == true; len == 1
            (false, true, 1) => ChildrenClass::Single(self.types.first().unwrap().clone().name()),

            // multiple == false; required == false; len == 1
            (false, false, 1) => {
                ChildrenClass::MaybeSingle(self.types.first().unwrap().clone().name())
            }

            // multiple == false; required = true; len > 1
            (false, true, _) => {
                ChildrenClass::Choice(self.types.into_iter().map(|t| t.name()).collect())
            }

            // multiple == false; required = false; len > 1
            (false, false, _) => {
                ChildrenClass::MaybeChoice(self.types.into_iter().map(|t| t.name()).collect())
            }

            // multiple == true; required == true; len == 1
            (true, true, 1) => ChildrenClass::Repeated(self.types.first().unwrap().clone().name()),

            // multiple == true; required == false; len == 1
            (true, false, 1) => {
                ChildrenClass::MaybeRepeated(self.types.first().unwrap().clone().name())
            }

            // multiple == true; required = true; len > 1
            (true, true, _) => {
                ChildrenClass::List(self.types.into_iter().map(|t| t.name()).collect())
            }

            // multiple == true; required = false; len > 1
            (true, false, _) => {
                ChildrenClass::MaybeList(self.types.into_iter().map(|t| t.name()).collect())
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Subtype {
    pub named: bool,
    #[serde(rename = "type")]
    pub subchild_type_name: String,
}

impl Subtype {
    pub fn name(self) -> NamedSubtype {
        let name = mangle_node_name(self.named, &self.subchild_type_name);
        NamedSubtype {
            named: self.named,
            subchild_type_name: self.subchild_type_name,
            ident: syn::Ident::new(&name, Span::mixed_site()),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NamedSubtype {
    pub named: bool,
    pub subchild_type_name: String,
    pub ident: Ident,
}
