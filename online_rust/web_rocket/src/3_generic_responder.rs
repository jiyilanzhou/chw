
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod models;
// 通用 Responder 处理
mod lib;

use models::UserModel;
use lib::Json;

use rocket::response::content;

#[get("/info")]
fn user(uid: i32) -> Json<UserModel> {
    Json(UserModel {user_id: uid, user_name: String::from("chw")})
}
fn main() {     // 编译报错:待解决[?]
    let server = rocket::ignite();
    server.mount("/", routes![user]).launch();
}
