use crate::tree::Node;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Tree {
    pub nodes: HashMap<u32, Node>,
    pub count: u32
}

impl Tree {
    pub fn new() -> Self {
        Tree { nodes: HashMap::new(), count: 0 }
    }
    
    pub fn add_node(&mut self, id: u32, parent_id: Option<u32>) {
        let parent = self.nodes.get(&parent_id.unwrap_or_default());
        match parent {
            Some(_parent) => {
                let new_node = Node::new(id, parent_id);
                self.nodes.insert(self.count, new_node);
            }
            None => {
                let new_node = Node::new(id, None);
                self.nodes.insert(self.count, new_node);
            }
        };

        self.count += 1;
    }
}
