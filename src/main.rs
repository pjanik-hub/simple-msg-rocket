use dotenvy::dotenv;
use rocket::{get, routes};
use rocket_db_pools::sqlx::Row;
use rocket_db_pools::{Connection, Database};

#[derive(Database)]
#[database("simple_msg")]
struct DB(sqlx::MySqlPool);

#[get("/<id>")]
async fn read(mut db: Connection<DB>, id: i64) -> Option<String> {
    sqlx::query("SELECT * FROM users WHERE id = ?")
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
        .mount("/", routes![read])
        .launch()
        .await;
}
