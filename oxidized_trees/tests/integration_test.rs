extern crate oxidized_trees;
use oxidized_trees::Tree;

#[test]
fn test_tree() {
    let mut tree = Tree::new();
    tree.add_node(1, None);
    tree.add_node(2, Some(1));

    assert_eq!(tree.nodes.len(), 2);
    assert_eq!(tree.nodes.get(&1).unwrap().id, 1);
    assert_eq!(tree.nodes.get(&1).unwrap().path, "1");
    assert_eq!(tree.nodes.get(&1).unwrap().parent_id, None);

    assert_eq!(tree.nodes.get(&2).unwrap().id, 2);
    assert_eq!(tree.nodes.get(&2).unwrap().path, "1.2");
    assert_eq!(tree.nodes.get(&2).unwrap().parent_id, Some(1));
}
