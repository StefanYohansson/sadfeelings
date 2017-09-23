#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate rocket_contrib;
extern crate rocket;
extern crate github_rs;
extern crate rss;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use github_rs::Json;
use github_rs::client::Github;
use rss::{Channel, Item};

use rocket::Request;
use rocket_contrib::Template;
use rocket::response::NamedFile;


#[derive(Deserialize)]
struct Config {
    api: String,
}

fn extract_json(json: &Json, key: &str) -> String {
    json.get(key).unwrap().to_string().trim_matches('"').to_string()
}

#[get("/")]
fn index() -> Template {
    let mut context: HashMap<&str, String> = HashMap::new();

    let mut file = File::open("config.toml").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents);
    let config: Config = toml::from_str(&contents).unwrap();

    let client = Github::new(config.api);
    let me = client.unwrap().get()
                   .user()
                   .execute();

    match me {
        Ok((headers, status, json)) => {
            if let Some(json) = json{
                context.insert("email", extract_json(&json, "email"));
                context.insert("url", extract_json(&json, "html_url"));
                context.insert("avatar_url", extract_json(&json, "avatar_url"));
                context.insert("followers", extract_json(&json, "followers"));
                context.insert("followers_url", extract_json(&json, "followers_url"));
            }
        },
        Err(e) => println!("{}", e)
    }

    let channel = Channel::from_url("http://snotr.sadfeelings.me/feed.xml").unwrap();
    let item = channel.items().first();
    match item {
        Some(i) => {
           context.insert("latest_title", i.title().unwrap().to_string()); 
           context.insert("latest_link", i.link().unwrap().to_string()); 
        },
        None => println!("no feed available")
    }
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
