/*
0. 获取 query 参数方法

*/
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

// 网页请求" http://localhost:8080/user?page=6&size=8 "
#[get("/user?<page>&<size>")]
/* // 无法避免由于用户误输入而导致崩溃
    fn user(page: i32, size: i32) -> String {
        format!("用户列表  page:{}, size:{}", page, size)
    }
*/
// 优化(用于处理用户误输入如" http://localhost:8080/user?page=3&size=a ")
fn user(page: Option<i32>, size: Option<i32>) -> String {
    let page = page.unwrap_or(1);
    let size = size.unwrap_or(6);
    format!("用户列表  page:{}, size:{}", page, size)
}

/*
1. 相同路由 path 的优先级设置
   使用" rank "如 ： #[get("/detail/<uid>", rank = 1)]

*/
mod lib;
mod models;

use lib::Json;
// 使用" 重导出 "
use models::UserModel;
// 未使用" 重导出 "
use models::user_model::User;

// 路由相同时使用 rank 设置优先级
// " http://localhost:8080/detail/003 "
#[get("/detail/<uid>", rank = 1)]
fn detail(uid: i32) -> Json<User<i32>> {
    Json(User { user_id: uid, user_name: String::from("chw") })
}
/* 因访问皆为字符串形式故若将字符串形式的访问级别设置为最高
   则永远无法访问其余设置的级别。
   // " http://localhost:8080/detail/a003 "
 */
#[get("/detail/<uid>", rank = 2)]
fn info(uid: String) ->Json<User<String>> {
    Json(User { user_id: uid, user_name: String::from("静心道") })
}


/*
2. 获取 JSON 请求参数的方法、自定义 json 字段输出
   a. 项目 Cargo.toml 文件内添加配置 :
       [dependencies.rocket_contrib]
       version = "0.4.5"
       default-feature = false
       features = ["json"]
    b. (使用提供的 Json )其实现 Responder 源码：
          /// Serializes the wrapped value into JSON. Returns a response with Content-Type
          /// JSON and a fixed-size body with the serialized value. If serialization
          /// fails, an `Err` of `Status::InternalServerError` is returned.
          impl<'a, T: Serialize> Responder<'a> for Json<T> {
              fn respond_to(self, req: &Request) -> response::Result<'a> {
                  serde_json::to_string(&self.0).map(|string| {
                      content::Json(string).respond_to(req).unwrap()
                  }).map_err(|e| {
                      error_!("JSON failed to serialize: {:?}", e);
                      Status::InternalServerError
                  })
              }
          }

3. 自定义 Json 字段
    a. 文档" https://serde.rs/variant-attrs.html "
    b. 如" #[serde(rename(serialize = "ser_name", deserialize = "de_name"))]  "
       则"ser_name"为输出的名称


*/
use rocket_contrib::json::Json as RC_Json;

/* // 使用 postman 请求：
   RC_Json<User<i32>> 反序列化为实体 user
   (前提：在 User 结构体上派生 Deserialize trait)
 */
#[post("/login", format="json",data="<u>")]
fn login(u: RC_Json<User<String>>) -> RC_Json<User<String>> {
    u
}

fn main() {
    let server = rocket::ignite();
    server.mount("/", routes![user,detail,info,login]).launch();
}
