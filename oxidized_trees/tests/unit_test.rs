extern crate oxidized_trees;
use oxidized_trees::Tree;
use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_ancestors() {
        let mut tree = Tree::new();

        tree.add_node(1, None);
        tree.add_node(2, Some(1));
        tree.add_node(3, Some(1));
        tree.add_node(4, Some(2));
        tree.add_node(5, Some(2));
        tree.add_node(6, Some(3));

        let node1 = tree.get_node(1).unwrap();
        let node2 = tree.get_node(2).unwrap();
        let node6 = tree.get_node(6).unwrap();

        assert_eq!(node1.children, HashSet::from([2, 3]));
        assert_eq!(node2.children, HashSet::from([4, 5]));
        assert_eq!(node6.children, HashSet::new());

        assert_eq!(
            tree.common_ancestors(4, 5),
            (HashSet::from([1, 2]), Some(2), Some(1), Some(2))
        );
        assert_eq!(
            tree.common_ancestors(4, 6),
            (HashSet::from([1]), Some(1), Some(1), Some(1))
        );
        assert_eq!(
            tree.common_ancestors(1, 6),
            (HashSet::new(), None, None, Some(0))
        );
        assert_eq!(
            tree.common_ancestors(1, 99),
            (HashSet::new(), None, None, None)
        );
    }

    #[test]
    fn test_non_existent_node() {
        let mut tree = Tree::new();
        tree.add_node(1, None);
        assert_eq!(
            tree.common_ancestors(1, 2),
            (HashSet::new(), None, None, None)
        );
    }

    #[test]
    fn test_get_descendants() {
        let mut tree = Tree::new();
        tree.add_node(1, None);
        tree.add_node(2, Some(1));
        tree.add_node(3, Some(1));
        tree.add_node(4, Some(2));
        tree.add_node(5, Some(2));
        tree.add_node(6, Some(3));
        tree.add_node(7, Some(3));

        assert_eq!(tree.get_descendants(1), HashSet::from([2, 3, 4, 5, 6, 7]));
        assert_eq!(tree.get_descendants(2), HashSet::from([4, 5]));
        assert_eq!(tree.get_descendants(3), HashSet::from([6, 7]));
    }
}
