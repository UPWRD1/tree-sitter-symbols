use super::schema::NodeType;
use heck::{ToKebabCase, ToUpperCamelCase};
use std::collections::{HashMap, HashSet};
use std::io::{self};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Variant {
    pub variant_name: String,
    pub original_name: String,
    pub subtypes: Option<DerivedType>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct DerivedType {
    pub variants: Vec<Variant>,
}

pub fn analyze(crate_name: &str) -> io::Result<DerivedType> {
    // Use cargo metadata to find the crate's source directory
    let metadata = cargo_metadata::MetadataCommand::new().exec().unwrap();
    let package = metadata
        .packages
        .iter()
        .find(|p| p.name == crate_name.to_kebab_case())
        .expect("Crate not found in dependencies");

    let json_path = package
        .manifest_path
        .parent()
        .unwrap()
        .join("src/node-types.json");
    // println!("{json_path}");
    let node_json_string = std::fs::read_to_string(json_path)?;

    // Parse the nodes from the provided json
    let node_types: Vec<NodeType> = serde_json::from_str(&node_json_string)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let variant_map = build_variants(&node_types);
    Ok(variant_map)
}

fn build_variants(node_types: &[NodeType]) -> DerivedType {
    let mut seen_variants = HashSet::new();
    let mut seen_subtypes: HashSet<String> = HashSet::new();
    let mut potential_variants = HashSet::new();
    for node_type in node_types {
        let original = node_type.clone();
        let original_name = original.node_type_name;
        let variant_name = mangle_node_name(&mut seen_variants, node_type.named, &original_name);
        if variant_name.as_ref().is_none_or(|v| v.is_empty()) {
            continue;
        }
        let Some(variant_name) = variant_name else {
            unreachable!("should have continued")
        };

        let variant = if let Some(subtypes) = &node_type.subtypes {
            for sub in subtypes {
                if sub.named {
                    seen_subtypes.insert(sub.subchild_type_name.clone());
                }
            }
            let subtypes = DerivedType {
                variants: subtypes
                    .iter()
                    .filter_map(|s| {
                        let subtype_name =
                            mangle_node_name(&mut seen_variants, s.named, &s.subchild_type_name);
                        match subtype_name {
                            Some(subtype_name) => Some(Variant {
                                variant_name: subtype_name,
                                original_name: s.subchild_type_name.clone(),
                                subtypes: None,
                            }),
                            None => None,
                        }
                    })
                    .collect(),
            };

            Variant {
                variant_name,
                original_name,
                subtypes: Some(subtypes),
            }
        } else {
            Variant {
                variant_name,
                original_name,
                subtypes: None,
            }
        };
        potential_variants.insert(variant);
    }

    let variants = potential_variants
        .into_iter()
        .filter(|k| !seen_subtypes.contains(&k.original_name))
        .collect();
    DerivedType { variants }
}

fn mangle_node_name(
    seen: &mut HashSet<String>,

    is_named: bool,
    original_name: &str,
) -> Option<String> {
    let mut name = original_name.to_owned();
    let chars = name.chars();
    name = chars
        .enumerate()
        .map(|(i, c)| {
            if i == 0 {
                if !(c.is_alphabetic() || c == '_') {
                    unicode_names2::name(c).unwrap().to_string()
                } else {
                    c.to_string()
                }
            } else {
                if !(c.is_alphanumeric() || c == '_') {
                    unicode_names2::name(c).unwrap().to_string()
                } else {
                    c.to_string()
                }
            }
        })
        .collect();
    name = name.to_upper_camel_case();
    name = match syn::parse_str::<syn::Ident>(&name) {
        Ok(_) => {
            if !is_named {
                name.push_str("Token");
                name
            } else {
                name
            }
        }
        Err(_) => format!("{}Token", name),
    };
    // Add suffix for unnamed nodes to distinguish them

    if original_name.starts_with('_') {
        name = format!("_{name}")
    }

    if seen.contains(original_name) {
        // *count += 1;
        None
        // format!("{name}{count}")
    } else {
        seen.insert(original_name.to_string());
        Some(name.clone())
    }
}
