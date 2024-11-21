#[macro_use]
extern crate rocket;

pub mod tree;
pub use tree::Node;
pub use tree::Tree;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let tree = setup();
    dbg!(&tree);
    // dbg!(tree.common_ancestors(4, 5));
    // dbg!(tree.common_ancestors(4, 7));

    let _rocket = rocket::build()
        .mount("/hello", routes![index])
        .launch()
        .await?;

    Ok(())
}

fn setup() -> Tree {
    let mut tree = Tree::new();

    tree.add_node(1, None);
    tree.add_node(2, Some(1));
    tree.add_node(3, Some(1));
    tree.add_node(4, Some(2));
    tree.add_node(5, Some(2));
    tree.add_node(6, Some(3));
    tree.add_node(7, Some(3));

    tree
}
