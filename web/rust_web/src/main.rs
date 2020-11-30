
/*
0. 读取配置文件
   # 引入依赖

1. config 依赖(" https://crates.io/crates/config ")
    [dependencies]
    config = "0.10.1"   # 可用于加载环境配置


 */
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use tokio_postgres::NoTis;
use crate::handler::{get_lists, query_lists, query_items, create_list, check_item};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

mod model;
mod config;
mod handler;
mod db;

async fn status() -> impl Responder {
    // "{\"status\":\"up\"}"
    web::HttpResponse::Ok()
        .json(model::Status{ status: "OK".to_string() })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //println!("Starting server at http://127.0.0.1:8080/");
    // 使用 config (未实现[?])
    // let config = config::Config::from_env().unwrap();
    // println!("Starting server at http://{}:{}/",config.server.host,config.server.port);

    // 使用线程池
    let pool = config.pg.create_pool(NoTis).unwrap();

    HttpServer::new(move || {
        App::new()
            //.data(pool.clone())
            // 路由配置方式 1 : 使用注解路由(推荐使用)
            //.service(hello)
            .service(echo)
            // 路由配置方式 2 ：使用 router
            .route("/hey", web::get().to(manual_hello))
            /*仅执行首个匹配的路径:
                " hello 及 status "均配置访问路径为" / "故永远无法执行到后配置的" status "
                若调换 hello  及 status 顺序则执行 status function
            */
            .route("/",web::get().to(status))

            // 调用 handler 下的 get_list
            .route("/todos{_:/?}",web::get().to(query_lists))
            .route("/todos/{list_id}/items",web::get().to(query_items))
            // 插入数据
            .route("/todos{_:/?}",web::post().to(create_list))
            .route("/todos/{list_id}/items/{item_id}{_:/?}",web::post().to(check_item))

    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}