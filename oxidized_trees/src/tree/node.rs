#[derive(Debug, Clone)]
pub struct Node {
    pub id: u32,
    pub parent_id: Option<u32>,
    pub path: String,
    pub ancestors: Vec<u32>,
    pub height: u32,
}

impl Node {
    pub fn new(id: u32, parent_id: Option<u32>, path: String, ancestors: Vec<u32>) -> Self {
        Node {
            id,
            parent_id,
            path,
            ancestors,
            height: 0,
        }
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn get_ancestors(&self) -> Vec<u32> {
        self.ancestors.clone()
    }
}
