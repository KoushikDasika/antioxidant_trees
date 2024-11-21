#[macro_use]
extern crate rocket;
extern crate rocket_dyn_templates;

pub mod tree;
use rocket::State;
use rocket_dyn_templates::{context, Template};
pub use tree::Node;
pub use tree::Tree;

#[get("/hello")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/descendants/<id>")]
fn descendants(tree: &State<Tree>, id: u32) -> Template {
    let descendants = tree
        .get_descendants(id)
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>();
    Template::render(
        "descendants",
        context! {
            title: "Descendants",
            descendants: descendants
        },
    )
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let tree = setup();
    let _rocket = rocket::build()
        .mount("/", routes![index, descendants])
        .manage(tree)
        .attach(rocket_dyn_templates::Template::fairing())
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
    tree
}
