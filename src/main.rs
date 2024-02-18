mod db_init;
mod db_migrate;

#[macro_use] extern crate rocket;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rocket::serde::{Deserialize, json::Json};
use rocket::State;

#[put("/challenge/update", data="<request>")]
fn change_value_by(request: Json<ChangeValueByRequest>) -> Result<(), String> {
    println!("{} - {}", request.user_id, request.change_value);
    Ok(())
}

#[get("/")]
fn hello(p: &State<Pool<SqliteConnectionManager>>) -> String {
    let pool = p.clone();
    let result = pool.get().unwrap();
    let mut stmt = result.prepare("SELECT name FROM challenge").unwrap();
    let rows = stmt.query_map([], |row| {
        Ok(row.get::<usize, String>(0))
    }).unwrap();

    let x = rows.last().unwrap().unwrap().unwrap();

    x
}

// fn create_new_user()

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct ChangeValueByRequest {
    user_id: String,
    change_value: i32
}

#[launch]
fn rocket() -> _ {
    rocket::build().manage(db_init::init("./db.sqlite")).mount("/", routes![change_value_by, hello])
}

