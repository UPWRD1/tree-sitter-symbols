use serde::{Deserialize, Serialize};
use tss_macros::generate_nodes;

#[generate_nodes(tree_sitter_python)]
enum PyNodes {}

#[cfg(test)]
mod tests {
    use super::PyNodes;
    #[test]
    fn match_nodes() {
        let t: PyNodes = PyNodes::_SimpleStatement(_SimpleStatement::PassStatement);
        match t {
            PyNodes::_SimpleStatement(_SimpleStatement::PassStatement) => {
                println!("pass statement")
            }

            _ => todo!(),
        }
    }
}
