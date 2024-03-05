use dotenvy::dotenv;
use rocket::get;
use rocket_db_pools::sqlx::Row;
use rocket_db_pools::{Connection, Database};
use rocket_okapi::settings::UrlObject;
use rocket_okapi::{rapidoc::*, swagger_ui::*};

#[derive(Database)]
#[database("simple_msg")]
struct DB(sqlx::MySqlPool);

#[get("/<id>")]
async fn read(mut db: Connection<DB>, id: i64) -> Option<String> {
    sqlx::query("SELECT content FROM user WHERE id = ?")
        .bind(id)
        .fetch_one(&mut **db)
        .await
        .and_then(|r| Ok(r.try_get(0)?))
        .ok()
}

#[rocket::main]
async fn main() {
    dotenv().ok();

    let _ = rocket::build()
        .attach(DB::init())
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
        .mount("/", rocket::routes![read])
        .launch()
        .await;
}
