use rocket::fairing::{self, AdHoc};
use rocket::futures::TryFutureExt;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{Build, Rocket};
use rocket_db_pools::{Connection, Database};

#[derive(Database)]
#[database("simple_msg")]
struct Db(sqlx::MySqlPool);

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct User {
    id: Option<i64>,
    username: String,
    email: String,
}

#[get("/<id>")]
async fn read(mut db: Connection<Db>, id: i64) -> Option<Json<User>> {
    sqlx::query!("SELECT id, username, email FROM users WHERE id = ?", id)
        .fetch_one(&mut **db)
        .map_ok(|r| {
            Json(User {
                id: Some(r.id.into()),
                username: r.username.expect("No user"),
                email: r.email.expect("No email"),
            })
        })
        .await
        .ok()
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match Db::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("db/migrations").run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        rocket
            .attach(Db::init())
            .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
            .mount("/sqlx", routes![read])
    })
}
