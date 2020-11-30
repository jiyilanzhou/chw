
/*
0. 插入

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

// 插入
fn insert(mut conn: PooledConn) {
    /*  // 方式 1 : stmt 方式执行
        let stmt = conn
            .prep("insert into user(user_name) values(?)").unwrap();
        // 针对非重要的数据且不关注执行结果则可用 exec_drop
        // conn.exec_drop(&stmt,("chw"));
        let rs = conn
            .exec_iter(stmt, ("chw", ))
            .unwrap();
        println!("{:?}", rs.last_insert_id().unwrap());
        println!("{:?}", rs.affected_rows());
    */

    // 方式 2 ：TextQuery 字符串方式执行
    let rs = "insert into user(user_name) values(?)"
        .with(("chw", ))
        .run(&mut conn)
        .unwrap();

    println!("{:?}", rs.last_insert_id().unwrap());
    println!("{:?}", rs.affected_rows());
}

fn main() {
    // 初始化
    let mut conn = db().unwrap();
    //  插入
    insert(conn);
}
