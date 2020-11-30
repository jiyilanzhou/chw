
// actix[æktɪks]
/*
0. 引入依赖并编译运行
    [dependencies]
    actix-web = "3"

1. 测试
    Win + R -> curl http://localhost:8080/
        C:\Users\Administrator>curl http://localhost:8080/
        Hello world!

 */
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

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
async fn status() -> impl Responder {
    // "{\"status\":\"up\"}"
    web::HttpResponse::Ok()
        .json(model::Status{ status: "OK".to_string() })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080/");
    HttpServer::new(|| {
        App::new()
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
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}