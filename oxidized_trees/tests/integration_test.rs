extern crate oxidized_trees;
use oxidized_trees::Tree;
use std::collections::HashSet;

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

#[test]
fn test_common_ancestors() {
    let mut tree = Tree::new();

    tree.add_node(1, None);
    tree.add_node(2, Some(1));
    tree.add_node(3, Some(1));
    tree.add_node(4, Some(2));
    tree.add_node(5, Some(2));
    tree.add_node(6, Some(3));
    tree.add_node(7, Some(3));

    assert_eq!(
        tree.common_ancestors(2, 3),
        (HashSet::from([1]), Some(1), Some(1), Some(1))
    );
    assert_eq!(
        tree.common_ancestors(4, 5),
        (HashSet::from([1, 2]), Some(2), Some(1), Some(2))
    );
    assert_eq!(
        tree.common_ancestors(4, 7),
        (HashSet::from([1]), Some(1), Some(1), Some(1))
    );
}
