
// stage[steɪdʒ]n. 阶段,舞台
/*
2. Rocket 支持的几个配置
   development(short:dev)   # 开发使用配置
   staging(short:stage)     # 阶段性(分段)配置
   production(short:prod)   # 生产环境使用配置

3. Rocket.toml 配置(项目根目录下配置" Rocket.toml "[自动加载])
    a. 配置示例
        [development]
        address = "0.0.0.0"
        port = 8080
        # 临时设置并运行" set ROCKET_ENV=development && cargo run "
        # Linux 直接" ROCKET_ENV=development cargo run "
    b. 检测
       (Windows 终端)执行" set ROCKET_ENV=development "命令
       (Windows 终端)检测" echo %ROCKET_ENV% "
       (Windows 终端)启动" cargo run "

 */
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

#[get("/")]
fn my_index()->&'static str{
    "this is my index"
}
#[get("/user")]
fn users()->&'static str{
    "用户列表"
}

#[get("/user/<uid>/info")]
fn users_detail(uid:i32)->String{
    format!("user detail id:{}",uid)
}
fn main() {
    let server=rocket::ignite();
    server.mount("/", routes![my_index,users,users_detail]).launch();
}
