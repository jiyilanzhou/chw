
// 不安全的函数/方法
unsafe fn dangerous() {
    println!("do something dangerous");
}
// 完全函数/方法(可内置" unsafe "片段)
fn foo() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // 解引用: 须置于 unsafe 块
    unsafe {
        *r2 = 3;// 操作(读写)数据须置于 unsafe 块
        println!("*r1 = {}", *r1);   // Console:" 3 "
        println!("*r2 = {}", *r2);   // Console:" 3 "
    }
}

fn main() {
    // 调用普通安全函数/方法(尽管其内置" unsafe "片段)
    foo();

    // 调用不安全的函数/方法(同理须置于" unsafe "块)
    unsafe {
        dangerous();
    }
    /* 直接调用 unsafe 函数/方法 ：飘红报错
      "Call to unsafe function requires unsafe function or block [E0133]"
    */
   // dangerous(); // 暂且注释：以通过编译

}