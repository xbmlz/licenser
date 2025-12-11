#[macro_use]
extern crate rocket;

mod db;

use db::init_db;

#[get("/")]
async fn index() -> String {
    "Hello, world!".to_string()
}

#[launch]
async fn rocket() -> _ {
    let db = init_db().await;
    rocket::build().manage(db).mount("/", routes![index])
}
