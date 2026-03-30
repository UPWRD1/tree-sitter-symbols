use tss_macros::generate_nodes;

#[generate_nodes(tree_sitter_rust)]
enum RustNodes {}

mod tests{
    use crate::RustNodes;

    #[test]
    fn match_nodes {
        let t: RustNodes;
        match t {
                    }
    }
}
