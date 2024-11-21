#[derive(Debug, Clone)]
pub struct Node {
    pub id: u32,
    pub parent_id: Option<u32>,
    pub path: String
}

impl Node {
    pub fn new(id: u32, parent_id: Option<u32>, path: String) -> Self {
        Node {
            id,
            parent_id,
            path
        }
    }

    pub fn get_parent_id(&self) -> Option<u32> {
        self.parent_id
    }
}
