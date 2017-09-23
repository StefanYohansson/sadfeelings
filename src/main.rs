#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;
extern crate github_rs;
extern crate rss;

use github_rs::client::Github;
use github_rs::Json;

use std::collections::HashMap;
use rocket_contrib::Template;
use rocket::Request;

use std::path::{Path, PathBuf};

use rocket::response::NamedFile;

use rss::{Channel, Item};

fn mock_context<'a>() -> HashMap<&'a str, String> {
    let mut context: HashMap<&str, String> = HashMap::new();
    context.insert("email", "snotr@sadfeelings.me".to_string());
    context.insert("url", "https://github.com/StefanYohansson".to_string());
    context.insert("avatar_url", "".to_string());
    context.insert("followers", "41".to_string());
    context.insert("followers_url", "https://github.com/StefanYohansson/followers".to_string());
    context
}

fn extract_json(json: &Json, key: &str) -> String {
    json.get(key).unwrap().to_string().trim_matches('"').to_string()
}

#[get("/")]
fn index() -> Template {
    let mut context: HashMap<&str, String> = HashMap::new();
    // @TODO: move token to config
    // it's a read only token, so it won't be a problem
    let client = Github::new("2538a3824751f390882c5c1edd06fa7f30953c9f");
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
