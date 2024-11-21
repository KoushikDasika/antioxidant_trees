mod common;

#[test]
fn test_setup() {
    common::setup();
    assert_eq!(1, 1);
}

#[test]
fn test_tree() {
    let mut tree = Tree::new();
    let node = Node::new(1, 0);
    tree.add_node(node);
    
    dbg!(&tree);
    dbg!(&tree.nodes);

    assert_eq!(tree.count, 1);
}