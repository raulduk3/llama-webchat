#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_dyn_templates;

use rocket::response::content::RawHtml;
use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::Template;

mod routes;

#[launch]
fn rocket() -> _ {
    let llama = llm::load::<llm::models::Llama>(
        std::path::Path::new(relative!("/open_llama_7b-f16.bin")),
        Default::default(),
        llm::load_progress_callback_stdout
    )
    .unwrap_or_else(|err| panic!("Failed to load model: {err}"));

    rocket::build()
        .manage(llama)
        .mount("/", routes![routes::index, routes::ask])
        .mount("/static", FileServer::from(relative!("static")))
        .register("/", catchers![not_found])
        .attach(Template::fairing())
}

#[catch(404)]
fn not_found() -> RawHtml<String> {
    RawHtml("<html><body><h1>404: Page not found</h1></body></html>".to_string())
}

