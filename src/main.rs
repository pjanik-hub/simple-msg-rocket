#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_db_pools;

mod db;

#[launch]
fn rocket() -> _ {
    rocket::build().attach(db::stage())
}
