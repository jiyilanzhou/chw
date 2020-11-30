
// 序列化
use serde::{Serialize,Deserialize};
// 标准库 io 读写 buffer
use std::io::Cursor;
// 请求对象
use rocket::request::Request;
// 响应
use rocket::response::{self,Response,Responder};
use rocket::http::ContentType;

#[derive(Serialize)]
pub struct UserModel {
    pub user_id: i32,
    pub user_name: String,
}

// 实现 Responder
impl<'a>Responder<'a> for UserModel{
    fn respond_to(self, _:&Request) -> response::Result<'a> {
        let json = serde_json::to_string(&self).unwrap();
        // " Response::build() -> ResponseBuilder<'r> " 用于构建 body 及 header
        Response::build()
            .sized_body(Cursor::new(json))
            .header(ContentType::new("application","json"))
            .ok()
    }
}

#[derive(Serialize,Deserialize,Debug)]
pub struct User<T> {
    // serialize 为输出字段名 / deserialize 为输入时反序列化对应的字段名
    #[serde(rename(serialize = "ser_name", deserialize = "de_name"))]
    pub user_id: T,
    pub user_name: String,
}