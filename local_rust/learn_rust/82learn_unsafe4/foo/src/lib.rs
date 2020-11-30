
#![crate_type = "staticlib"]
#[no_mangle]
// 使用 extern 导出
pub extern fn foo() {
    println!("use rust");
}
