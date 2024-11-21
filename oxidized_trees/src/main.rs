pub mod tree;
pub use tree::Node;
pub use tree::Tree;

fn main() {
    let mut tree = Tree::new();

    tree.add_node(1, None);
    tree.add_node(2, Some(1));

    dbg!(&tree);
    // dbg!(&tree.nodes);
}
