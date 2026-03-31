use crate::macro_impl::schema::NamedNodeType;

use super::schema::NodeType;
use heck::{ToKebabCase, ToUpperCamelCase};
use std::collections::HashSet;
use std::io::{self};

pub fn analyze(crate_name: &str) -> io::Result<HashSet<NamedNodeType>> {
    // Use cargo metadata to find the crate's source directory
    let metadata = cargo_metadata::MetadataCommand::new().exec().unwrap();
    let package = metadata
        .packages
        .iter()
        .find(|p| p.name == crate_name.to_kebab_case())
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Crate not found in dependencies",
            )
        })?;

    let json_path = package
        .manifest_path
        .parent()
        .unwrap()
        .join("src/node-types.json");

    let node_json_string = std::fs::read_to_string(json_path)?;

    // Parse the nodes from the provided json
    let node_types: Vec<NodeType> = serde_json::from_str(&node_json_string)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Invalid json: {e}")))?;

    let variant_map = build_variants(&node_types);
    Ok(variant_map)
}

fn build_variants(node_types: &[NodeType]) -> HashSet<NamedNodeType> {
    node_types
        .iter()
        .filter_map(|node_type| {
            let original = node_type.clone();
            let variant_name = mangle_node_name(node_type.named, &original.node_type_name);
            if variant_name.is_empty() {
                return None;
            }

            let named_node = original.name(&variant_name);
            Some(named_node)
        })
        .collect()
}

pub fn mangle_node_name(is_named: bool, original_name: &str) -> String {
    let mut name = original_name.to_owned();
    let chars = name.chars();
    name = chars
        .enumerate()
        .map(|(i, c)| {
            if i == 0 {
                if c.is_alphabetic() || c == '_' {
                    c.to_string()
                } else {
                    unicode_names2::name(c).unwrap().to_string()
                }
            } else {
                if c.is_alphanumeric() || c == '_' {
                    c.to_string()
                } else {
                    unicode_names2::name(c).unwrap().to_string()
                }
            }
        })
        .collect();
    name = name.to_upper_camel_case();
    name = match syn::parse_str::<syn::Ident>(&name) {
        Ok(_) => {
            if !is_named {
                name.push_str("Token");
            }
            name
        }
        Err(_) => format!("{name}Token"),
    };
    // Add suffix for unnamed nodes to distinguish them

    if original_name.starts_with('_') {
        name = format!("_{name}");
    }

    name.clone()
}
