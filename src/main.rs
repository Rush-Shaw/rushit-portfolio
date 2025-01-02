#[macro_use]
extern crate rocket;

use rocket_dyn_templates::{Template, context};
use rocket::fs::{FileServer, NamedFile, relative};

#[get("/")]
fn index() -> Template {
    // Declare the description variable outside the context! macro
    let description = vec![
        "dive into my digital oasis! 🏝️".to_string(),
        "i'm a third year student studying computer science at McMaster University 🤓".to_string(),
        "i aim to develop expertise in data science 📊 and analysis 🔍, driven by my passion for uncovering the stories data tells. my goal is to leverage these skills to curate meaningful insights, particularly in areas like risk analysis ⛔️, where informed decisions can make a significant impact 🤑".to_string(),
        "when i'm not cooking it up in school 👨🏽‍🍳, you can catch me following my passion for finance 📈 or playing basketball outdoors 🏀".to_string(),
        "for a snapshot of my skills and experiences, check out my <a href='/resume' class='resume-link' style='text-decoration:underline;'>resume</a> 📄".to_string(),
    ];

    // Use the description variable inside context!
    Template::render("index", context! {
        title: "hey there, I’m Rushit! 👋",
        socials: "reach out to me 📲",
        projects: "cool stuff i've made 👨🏽‍💻",
        description: description, // Pass the preprocessed description
        footer: "made in rust by rushit",
        description_socials: vec![ 
            ("email --> rushshaw9@gmail.com", "mailto:rushshaw9@gmail.com"),
            ("github --> /rush-shaw", "https://github.com/Rush-Shaw"),
            ("linkedin --> /rushit-shah", "https://www.linkedin.com/in/rushit-shah-03b37319a/"),
            ("instagram --> /rush.photography9", "https://www.instagram.com/rush.photography9/"),
        ],
        description_projects: vec![ 
            ("vinculum --> bridging the hybrid and in-person workforce", "https://github.com/scythemenace/Vinculum"),
            ("grades automator --> aiding teachers dish out grades faster", "https://github.com/Rush-Shaw/GradesAutomater"),
            ("instagram-followers --> catching people that don't follow back", "https://github.com/Rush-Shaw/Instagram-Followers"),
        ],
    })
}

#[get("/resume")]
async fn render_resume() -> Option<NamedFile> {
    // Serve the precompiled resume.pdf directly
    NamedFile::open("static/resume.pdf").await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, render_resume]) // Mount both routes
        .mount("/static", FileServer::from(relative!("static"))) // Serve static files
        .attach(Template::fairing()) // Attach Tera templates
}
