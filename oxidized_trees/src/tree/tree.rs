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
                let new_node = Node::new(id, Some(parent_id.unwrap()));
                self.nodes.insert(id, new_node);
            }
            false => {
                let new_node = Node::new(id, None);
                self.nodes.insert(id, new_node);
            }
        };

        self.count += 1;
    }
}
