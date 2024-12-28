#[macro_use]
extern crate rocket;

use rocket_dyn_templates::{Template, context};
use rocket::fs::{FileServer, relative};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        title: "hey there, I’m Rushit! 👋",
        projects: "cool stuff i've made 👨🏽‍💻",
        description: vec![ "dive into my digital oasis! 🏝️",
                           "i'm a third year student studying computer science at McMaster University 🤓",
                           "i aim to develop expertise in data science 📊 and analysis 🔍, driven by my passion for uncovering the stories data tells. my goal is to leverage these skills to curate meaningful insights, particularly in areas like risk analysis ⛔️, where informed decisions can make a significant impact 🤑", 
                           "when i'm not cooking it up in school 👨🏽‍🍳, you can catch me following my passion for finance 📈 or playing basketball outdoors 🏀", 
                           "for a snapshot of my skills and experiences, check out my resume 📄"],
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index]) // Mount homepage
        .mount("/static", FileServer::from(relative!("static"))) // Serve static files
        .attach(Template::fairing()) // Attach Tera templates
}
