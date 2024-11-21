use crate::tree::Node;
use std::collections::HashMap;
use std::collections::HashSet;

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

    pub fn get_node(&self, id: u32) -> Option<&Node> {
        self.nodes.get(&id)
    }

    pub fn add_node(&mut self, id: u32, parent_id: Option<u32>) {
        let has_parent = self.nodes.contains_key(&parent_id.unwrap_or_default());
        match has_parent {
            true => {
                let parent_node = self.nodes.get_mut(&parent_id.unwrap()).unwrap();
                let new_path = format!("{}.{}", parent_node.path, id);
                let mut ancestors = parent_node.get_ancestors();
                ancestors.insert(parent_node.id);

                let new_node = Node::new(id, Some(parent_id.unwrap()), new_path, ancestors);
                parent_node.children.insert(id);
                self.nodes.insert(id, new_node);
            }
            false => {
                let new_node = Node::new(id, None, String::from(id.to_string()), HashSet::new());
                self.nodes.insert(id, new_node);
            }
        };

        self.count += 1;
    }

    pub fn common_ancestors(
        &self,
        id1: u32,
        id2: u32,
    ) -> (HashSet<u32>, Option<u32>, Option<u32>, Option<u32>) {
        let node1 = self.nodes.get(&id1);
        let node2 = self.nodes.get(&id2);

        match (node1, node2) {
            (Some(node1), Some(node2)) => {
                let ancestors1 = node1.get_ancestors();
                let ancestors2 = node2.get_ancestors();

                let ancestor_list: HashSet<u32> =
                    ancestors1.intersection(&ancestors2).cloned().collect();

                let common_ancestor = ancestor_list.iter().max().copied();

                let root_node = ancestor_list.iter().min().copied();
                let height = Some(ancestor_list.len() as u32);

                (ancestor_list, common_ancestor, root_node, height)
            }
            _ => (HashSet::new(), None, None, None),
        }
    }

    pub fn get_descendants(&self, id: u32) -> HashSet<u32> {
        let node = self.nodes.get(&id).unwrap();
        let children = node.children.clone();

        let mut descendants = HashSet::new();

        for child in children.iter() {
            descendants.insert(*child);
            descendants.extend(self.get_descendants(*child));
        }

        descendants
    }
}
