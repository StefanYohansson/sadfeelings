#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;

use std::collections::HashMap;
use rocket_contrib::Template;
use rocket::Request;

#[get("/")]
fn index() -> Template {
    let mut context: HashMap<&str, &str> = HashMap::new();
    Template::render("index", &context)
}

#[error(404)]
fn not_found(req: &Request) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().as_str());
    Template::render("error/404", &map)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .catch(errors![not_found])
        .launch();
}
