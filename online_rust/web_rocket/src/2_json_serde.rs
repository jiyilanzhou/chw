
// extern[ˈekstɜːn]adj.外部的
/*
4. 对象 JSON 输出， Serde 第三方库使用
   [dependencies]
    rocket = "0.4.5"
    serde = {version="1.0",features=["derive"]}
    serde_json = "1.0"
 */
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod models;
use models::UserModel;

use rocket::response::content;

#[get("/user")]
fn user()->&'static str{
    "用户列表"
}

#[get("/user/<uid>/info")]
/*
    //方式1: 官方推荐的" 返回 json 的格式 content::Json<String> "但可直接返回对象
    fn user_detail(uid:i32)->content::Json<String>{
    //fn user_detail(uid:i32)->String{         // 返回未携带格式的 String
        let user = UserModel{
            user_id:uid,
            user_name:String::from("chw"),
        };
        // serde_json::to_string(&user).unwrap() // 返回未携带格式的 String
        // 返回的格式是否为" json "应从"响应头"查看(而非观察数据表象)
        content::Json(serde_json::to_string(&user).unwrap())
    }
*/
/* 方式2：直接返回对象(结构体需实现" rocket::response::Responder<'_> ")
    // Responder<'r> Trait 源码：
    pub trait Responder<'r> {
        fn respond_to(self, request: &Request) -> response::Result<'r>;
    }
 */
fn user_detail(uid:i32)->UserModel{
    UserModel{
        user_id:uid,
        user_name:String::from("chw"),
    }
}

/*
5. 响应可返回 &str、String 及 对象等多种皆可：
    其原因是源于实现" rocket::response::Responder "trait。如对于" &str 及
    String "类型 Responder 皆" set Content-Type text/plain "(即设置为文本)

6. 自定义实现 Responder
    use std::io::Cursor;
    use rocket::request::Request;
    use rocket::response::{self,Response,Responder};
    use rocket::http::ContentType;

 */

fn main() {
    let server=rocket::ignite();
    server.mount("/", routes![user,user_detail]).launch();
}
