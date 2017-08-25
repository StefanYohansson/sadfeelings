#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;
extern crate github_rs;

use github_rs::client::Github;

use std::collections::HashMap;
use rocket_contrib::Template;
use rocket::Request;

use std::path::{Path, PathBuf};

use rocket::response::NamedFile;

#[get("/")]
fn index() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
/*
    let client = Github::new("API TOKEN");
    let me = client.get()
                   .user()
                   .execute();
                   */
    Template::render("index", &context)
}

// @TODO(snotr): receive as many folders and join by /
#[get("/assets/<folder>/<file..>")]
fn assets(folder: String, file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new(&format!("assets/{}", folder)).join(file)).ok()
}

#[error(404)]
fn not_found(req: &Request) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().as_str());
    Template::render("error/404", &map)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, assets])
        .attach(Template::fairing())
        .catch(errors![not_found])
        .launch();
}
