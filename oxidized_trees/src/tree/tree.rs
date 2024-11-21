use crate::tree::Node;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Tree {
    pub nodes: HashMap<u32, Node>,
    pub count: u32,
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            nodes: HashMap::new(),
            count: 0,
        }
    }

    pub fn add_node(&mut self, id: u32, parent_id: Option<u32>) {
        let has_parent = self.nodes.contains_key(&parent_id.unwrap_or_default());
        match has_parent {
            true => {
                let parent_node = self.nodes.get(&parent_id.unwrap()).unwrap();
                let new_path = format!("{}.{}", parent_node.path, id);
                let mut ancestors = parent_node.ancestors.clone();
                ancestors.push(parent_node.id);

                let new_node = Node::new(id, Some(parent_id.unwrap()), new_path, ancestors);
                self.nodes.insert(id, new_node);
            }
            false => {
                let new_node = Node::new(id, None, String::from(id.to_string()), Vec::new());
                self.nodes.insert(id, new_node);
            }
        };

        self.count += 1;
    }

    pub fn common_ancestors(&self, id1: u32, id2: u32) -> (Vec<u32>, Option<u32>, Option<u32>, Option<u32>) {
        let node1 = self.nodes.get(&id1);
        let node2 = self.nodes.get(&id2);

        match (node1, node2) {
            (Some(node1), Some(node2)) => {
                let ancestor_list: Vec<u32> = node1
                    .get_ancestors()
                    .iter()
                    .filter(|a| node2.get_ancestors().contains(a))
                    .copied()
                    .collect();

                let common_ancestor = ancestor_list.last().copied();
                let root_node = ancestor_list.first().copied();
                let height = Some(ancestor_list.len() as u32);

                (ancestor_list, common_ancestor, root_node, height)
            }
            _ => (vec![], None, None, None),
        }
    }
}

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

        assert_eq!(tree.common_ancestors(4, 5), (vec![1, 2], Some(2), Some(1), Some(2)));
        assert_eq!(tree.common_ancestors(4, 6), (vec![1], Some(1), Some(1), Some(2)));
        assert_eq!(tree.common_ancestors(1, 6), (vec![], None, Some(0), Some(2)));
        assert_eq!(tree.common_ancestors(1, 99), (vec![], None, None, None));
    }

    #[test]
    fn test_non_existent_node() {
        let mut tree = Tree::new();
        tree.add_node(1, None);
        assert_eq!(tree.common_ancestors(1, 2), (vec![], None, None));
    }
}
