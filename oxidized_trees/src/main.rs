mod tree;
pub use tree::Node;
pub use tree::Tree;

fn main() {
    let mut tree = Tree::new();
    println!("tree: {:?}", tree);
    tree.add_node(1, None);
    println!("Adding Node 1");
    tree.add_node(2, Some(1));
    println!("Adding Node 2");

    dbg!(&tree);
    dbg!(&tree.nodes);
}
