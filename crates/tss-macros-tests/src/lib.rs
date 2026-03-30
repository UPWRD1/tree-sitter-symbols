use tss_macros::generate_nodes;

#[generate_nodes(tree_sitter_rust)]
enum RustNodes {}

// #[generate_nodes(tree_sitter_python)]
enum PyNodes {}

#[cfg(test)]
mod tests {
    use crate::PyNodes;

    #[test]
    fn match_nodes() {
        let t: PyNodes;
        match t {
            _ => todo!(),
        }
    }
}
