use rocket::{
    get,
    fs::FileServer,
    Build, Rocket,
};
use rocket_dyn_templates::Template;
use std::collections::HashMap;
use std::sync::Arc;

    use rocket::serde::json::Value;
    use rocket_dyn_templates::tera::{Context, Result, Tera};
    use rocket_dyn_templates::tera::Filter;


#[rocket::main]
pub async fn main() {
    let assets_path = "assets";
    let brol: HashMap<String, String> = HashMap::new();
    let assets = Arc::new(brol);

    rocket::build()
        .attach(Template::custom(move |engines| {
            engines.tera.register_filter("assetify", TeraFilters::AssetsFilter { 
                asset_store: assets.clone()
            });
        }))
        .mount(
            "/",
            rocket::routes![index],
        )
        .mount("/assets/", FileServer::from(&assets_path))
        .launch()
        .await
        .expect("Could not start backend");
}


pub mod TeraFilters {    
    use std::collections::HashMap;
    use rocket::serde::json::Value;
    use rocket_dyn_templates::tera::{Context, Result, Tera};
    use std::sync::Arc;

    pub struct AssetsFilter {
        pub asset_store: Arc<HashMap<String, String>>,
    }

    impl rocket_dyn_templates::tera::Filter for AssetsFilter {
        fn filter(&self, value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
            Ok("This is an asset".into())
        }
    }
} 



#[get("/")]
pub async fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();


    Template::render("index", context)
}