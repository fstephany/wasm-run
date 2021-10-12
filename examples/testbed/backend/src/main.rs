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
    let asset_prefix = "".to_string();

    rocket::build()
        .attach(Template::custom(move |engines| {
            engines.tera.register_filter("assetify", TeraFilters::AssetsFilter { 
                assets_prefix: asset_prefix.clone()
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

    /// This filter will convert the relative URL of the assets to the full 
    /// URL including the content hash if necessary
    pub struct AssetsFilter {
        pub assets_prefix: String,
    }

    impl rocket_dyn_templates::tera::Filter for AssetsFilter {
        fn filter(&self, value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
            let full_url = format!("/assets{}/{}", self.assets_prefix, value.to_string());

            Ok(full_url.into())
        }
    }
} 



#[get("/")]
pub async fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();


    Template::render("index", context)
}