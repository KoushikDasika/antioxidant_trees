pub mod tree;
pub use tree::Node;
pub use tree::Tree;

use std::collections::HashSet;

pub fn common_ancestors(
    id1: u32,
    id2: u32,
) -> (HashSet<u32>, Option<u32>, Option<u32>, Option<u32>) {
    crate::tree::Tree::new().common_ancestors(id1, id2)
}

pub fn get_descendants(id: u32) -> HashSet<u32> {
    crate::tree::Tree::new().get_descendants(id)
}
