use rocket::{
    get,
    fs::FileServer,
    Build, Rocket,
};
use rocket_dyn_templates::Template;
use std::collections::HashMap;

#[rocket::main]
pub async fn main() {
    let assets_path = "assets";
    rocket::build()
        .attach(Template::fairing())
        .mount(
            "/",
            rocket::routes![index],
        )
        .mount("/assets/", FileServer::from(&assets_path))
        .launch()
        .await
        .expect("Could not start backend");
}

#[get("/")]
pub async fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();


    Template::render("index", context)
}