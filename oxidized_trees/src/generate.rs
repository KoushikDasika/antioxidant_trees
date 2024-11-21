use crate::tree::Tree;

pub fn generate_random_nodes(tree: &mut Tree, number_of_nodes: u32, batch_size: u32) {
    let mut counter = 0;
    let mut stack: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    while counter < number_of_nodes {
        let parent_id = stack.pop().unwrap();

        for batch_counter in 0..batch_size {
            let new_id = parent_id + 10 * batch_size + batch_counter;
            tree.add_node(new_id, Some(parent_id));
            stack.push(new_id);
            counter += 1;
        }

        println!("Number of nodes: {}", tree.number_of_nodes());
    }

    println!("Finished");
}
