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

    pub fn common_ancestors(&self, id1: u32, id2: u32) -> Vec<u32> {
        let node1 = self.nodes.get(&id1).unwrap();
        let node2 = self.nodes.get(&id2).unwrap();

        dbg!(&node1.get_ancestors());
        dbg!(&node2.get_ancestors());

        let output = node1
            .get_ancestors()
            .iter()
            .filter(|a| node2.get_ancestors().contains(a))
            .copied()
            .collect();

        dbg!(&output);

        output
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

        assert_eq!(tree.common_ancestors(4, 5), vec![1, 2]);
        assert_eq!(tree.common_ancestors(4, 6), vec![1]);
        assert_eq!(tree.common_ancestors(1, 6), vec![]);
    }
}
