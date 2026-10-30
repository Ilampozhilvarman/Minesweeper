#[macro_use]
extern crate rocket;

use rocket::fs::{FileServer, NamedFile, relative};
use std::path::Path;

struct File<'a> {
    folder: &'a str,
    name: &'a str,
}

async fn open_file(file: File<'_>) -> Option<NamedFile> {
    NamedFile::open(Path::new(relative!(".")).join(file.folder).join(file.name))
        .await
        .ok()
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    open_file(File {
        folder: "templates",
        name: "index.html",
    })
    .await
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[get("/game")]
async fn game() -> Option<NamedFile> {
    open_file(File {
        folder: "templates",
        name: "game.html",
    })
    .await
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, hello, game])
        .mount("/static", FileServer::from(relative!("static")))
}
