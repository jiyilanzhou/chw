
/*
0. Mysql 驱动库
    文档" https://github.com/blackbeam/rust-mysql-simple "

1. 获取参数

2. 预处理

*/
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod models;
mod util;

use models::user_model::User;
use util::*;
// 内含相应 sql 操作的 trait 声明
use mysql::prelude::*;
use mysql::PooledConn;

// 查询
fn find(conn: &mut PooledConn) {
    // 获取一行数据
    let rs: Option<String> = conn
        .query_first("select user_name from user order by user_id desc limit 1")
        .unwrap();
    println!("{:?}", rs.unwrap());
    // 获取一行第 0 列字段数据
    let rs: Option<(i32, String)> = conn
        .query_first("select user_id,user_name from user order by user_id desc limit 1")
        .unwrap();
    println!("{:?}", rs.unwrap().0);
    // 查询集合
    let sql = "select * from user";
    /*  // 方式 1
        let users = conn
            .query_map(sql, |row: (i32, String)| User {
                user_id: row.0,
                user_name: row.1,
            }).unwrap();
     */

    /*  // 方式 2   // 飘红报错：待解决[?]
        let users = conn.query_map(sql, |(uid:i32, uname:String)| User {
            user_id: uid,
            user_name: uname,
        }).unwrap();
     */

    // 方式 3 ：使用 TextQuery 字符串方式
    let users: Vec<User<String>> = sql
        .map(conn, |(uid, uname)| {
            User {
                user_id: uid,
                user_name: uname,
            }
        })
        .unwrap();
    println!("-----------\n{:#?}", users);
}

// 预处理
fn preprocess(mut conn: PooledConn) {
    // 方式 1 ：使用占位符
    let stmt = conn.prep("select * from user where user_id = ?").unwrap();
    // let rs:Option<(i32,String)> = conn.exec_first(stmt,(3,)).unwrap();
    // 同理使用" |(uid:i32,uname:String)|User{user_id:uid,user_name:uname} "飘红(待解决[?])
    let rs = conn.exec_map(&stmt, (3, ), |row: (i32, String)| User {
        user_id: row.0,
        user_name: row.1,
    }).unwrap();
    //println!("-----------\n{:#?}", rs);

    // 方式 2 ：使用命名参数
    let stmt = conn.prep("select * from user where user_id = :uid").unwrap();
    // 使用 params! 需导入" use mysql::*; "
    use mysql::*;
    let rs: Option<(i32, String)> = conn.exec_first(stmt, params! {"uid"=>3}).unwrap();
    println!("-----------\n{:#?}", rs);
}

fn main() {
    // 初始化
    //init_db(5,10);
    let mut conn = db().unwrap();
    // 查询
    find(&mut conn);

    // 预处理
    preprocess(conn);
}