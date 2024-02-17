#[macro_use] extern crate rocket;

use rocket::serde::{Deserialize, json::Json};

#[put("/challenge/update", data="<request>")]
fn change_value_by(request: Json<ChangeValueByRequest>) -> Result<(), String> {
    println!("{} - {}", request.user_id, request.change_value);
    Ok(())
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct ChangeValueByRequest {
    user_id: String,
    change_value: i32
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![change_value_by])
}
