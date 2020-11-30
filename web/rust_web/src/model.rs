/* 使用需先引入" serde "依赖即：
    [dependencies]
    serde = { version = "1.0", features = ["derive"] }

 */
use serde::Serialize;
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize)]
pub struct Status {
    pub status: String
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "list")]
pub struct List {
    pub id: i32,
    pub title: String,
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "item")]
pub struct Item {
    pub id: i32,
    pub title: String,
    pub checked: bool,
    pub list_id: i32,
}

#[derive(Deserialize)]
pub struct CreateList {
    pub(crate) title: String
}

#[derive(Serialize)]
pub struct ResultResponse {
    pub success: bool
}