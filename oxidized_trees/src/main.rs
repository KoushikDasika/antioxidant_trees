#[macro_use]
extern crate rocket;
extern crate rocket_dyn_templates;

pub mod generate;
pub mod tree;

use generate::generate_random_nodes;
use rocket::State;
use rocket_dyn_templates::Template;
pub use tree::Node;
pub use tree::Tree;

#[get("/hello")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/descendants/<id>")]
fn descendants(tree: &State<Tree>, id: u32) -> Template {
    let mut descendants = tree
        .get_descendants(id)
        .iter()
        .map(|n| *n)
        .collect::<Vec<u32>>();

    descendants.sort();

    let context = rocket_dyn_templates::serde::json::json!({
        "id": id,
        "descendants": descendants
    });

    Template::render("descendants", &context)
}

#[get("/nodes/<id1>/common_ancestors/<id2>")]
fn common_ancestors(tree: &State<Tree>, id1: u32, id2: u32) -> Template {
    let (common_ancestors, common_ancestor, root_node, height) = tree.common_ancestors(id1, id2);

    let context = rocket_dyn_templates::serde::json::json!({
        "common_ancestors": common_ancestors,
        "common_ancestor": common_ancestor,
        "root_node": root_node,
        "height": height
    });

    Template::render("common_ancestors", &context)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let mut tree = Tree::new();
    generate_random_nodes(&mut tree, 100_000, 10);

    let _rocket = rocket::build()
        .mount("/", routes![index, descendants, common_ancestors])
        .manage(tree)
        .attach(rocket_dyn_templates::Template::fairing())
        .launch()
        .await?;

    Ok(())
}
