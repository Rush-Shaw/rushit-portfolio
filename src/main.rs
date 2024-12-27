#[macro_use]
extern crate rocket;

use rocket_dyn_templates::{Template, context};
use rocket::fs::{FileServer, relative};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        title: "Hey there, I’m Rushit! 👋",
        description: "Welcome to my digital oasis!",
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index]) // Mount homepage
        .mount("/static", FileServer::from(relative!("static"))) // Serve static files
        .attach(Template::fairing()) // Attach Tera templates
}
