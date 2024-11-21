pub mod tree;
pub use tree::Node;
pub use tree::Tree;

fn main() {
    let tree = setup();
    dbg!(&tree);

    // dbg!(tree.common_ancestors(4, 5));
    // dbg!(tree.common_ancestors(4, 7));
    dbg!(tree.get_descendants(1));
}

fn setup() -> Tree {
    let mut tree = Tree::new();

    tree.add_node(1, None);
    tree.add_node(2, Some(1));
    tree.add_node(3, Some(1));
    tree.add_node(4, Some(2));
    tree.add_node(5, Some(2));
    tree.add_node(6, Some(3));
    tree.add_node(7, Some(3));

    tree
}
