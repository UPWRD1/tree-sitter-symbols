use super::schema::NodeType;
use heck::{ToKebabCase, ToUpperCamelCase};
use std::collections::HashMap;
use std::io::{self};

pub fn generate(crate_name: &str) -> io::Result<Vec<(String, String)>> {
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
    println!("{json_path}");
    let node_json_string = std::fs::read_to_string(json_path)?;

    // Parse the nodes from the provided json
    let node_types: Vec<NodeType> = serde_json::from_str(&node_json_string)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let variant_map = build_variant_map(&node_types);
    Ok(variant_map)
}

fn build_variant_map(node_types: &[NodeType]) -> Vec<(String, String)> {
    let mut seen = HashMap::new();
    let mut result = Vec::new();

    for node_type in node_types {
        let mut name = node_type.node_type_name.clone();
        let chars = name.chars();
        name = chars
            .map(|c| {
                if !(c.is_alphabetic() || c == '_') {
                    unicode_names2::name(c).unwrap().to_string()
                } else {
                    c.to_string()
                }
            })
            .collect();
        name = name.to_upper_camel_case();
        name = match syn::parse_str::<syn::Ident>(&name) {
            Ok(_) => name,
            Err(_) => format!("{}Token", name),
        };
        // Add suffix for unnamed nodes to distinguish them
        if !node_type.named {
            name.push_str("Token");
            // continue;
        }

        let variant_name = if let Some(count) = seen.get_mut(&name) {
            *count += 1;
            format!("{name}{count}")
        } else {
            seen.insert(name.clone(), 1);
            name.clone()
        };
        if name.is_empty() {
            continue;
        }
        dbg!(&name);

        result.push((name, variant_name));
    }
    result
}
