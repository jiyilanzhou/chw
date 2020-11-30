
// ignite[ɪɡˈnaɪt]v. 点燃,燃烧
/*
问题：file is not included module tree,analysis is not available
     文件不包括模块树，分析不可用，待解决[?]

 */

/*
0.  Web 开发
    a. 几大劲爆的 Web 开发语言：Java、Go、Node等
    b. Java 为首的技术栈 : 适合开发懂重型、业务复杂等应用
    c. Go 为首的技术栈 : 高并发、高性能混合应用
    d. RUST web : 更适合对安全（如内存泄露）要求特别高的场景

1. Rocker
    a. Github 地址：" https://github.com/SergioBenitez/Rocket.git "
    b. 文档地址：" https://rocket.rs/v0.4/guide/getting-started/ "
    c. 官网：" https://rocket.rs/ "
    d. (使用依赖)在 Cargo.toml 中配置
        [depedencies]
        rocket="0.4.5"

 */

#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

#[get("/")]
fn my_index()->&'static str{
    "this is my index"
}

#[get("/user")]
fn user()->&'static str{
    "this is user "
}

#[get("/")]
fn info()->&'static str{
    "this is user "
}

fn main() {
    let server = rocket::ignite();
    /*" mount (挂载)"：首参数" / "为前缀路径(常用于"分组/分模块")
       未配置则默认" 8000 "端口
     */
    server.mount("/", routes![my_index,user]).launch();

    /* 编译报错:
        error[E0382]: use of moved value: `server`
            let server = rocket::ignite();
                ------ move occurs because `server` has type `rocket::Rocket`,
                                    which does not implement the `Copy` trait
            server.mount("/", routes![my_index,user]).launch();
            ------ value moved here
            server.mount("/info", routes![info]).launch();
            ^^^^^^ value used here after move
     */
    //server.mount("/info", routes![info]).launch();
    // 问题：如何挂载多个不同前缀的模块[?]

}