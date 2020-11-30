/*

 */
// 引入作用域
mod d0_model;
use d0_model::d0f0_user;
//use d0_model::d0f0_user::User; // 一般提取至父模块(降低代码冗余)

// 外部函数 // 可将" model::user::User "提取
fn set_user(u: &mut d0f0_user::User) {
    u.username = "静心道".to_string();
}

fn main() {
    let mut user = d0f0_user::User::new();
    println!("{:?}", user);
    // Console:" User { id: 0, username: "", age: 0, tags: ["", "", "", "", ""] } "

    // 修改属性方式 1 ：(条件：结构体相应字段须公开)
    user.username = String::from("chw");
    println!("{}", user.username);   // Console:" chw "
    // 修改属性方式 2 ：使用提供的函数
    set_user(&mut user);
    println!("{}", user.username);   // Console:" 静心道 "
}
