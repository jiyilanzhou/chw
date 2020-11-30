
/* 访问或者修改可变静态变量:
   (其它语言中的)"全局变量" 在 Rust 中称为" 静态变量 "

*/
// 声明静态变量
static HELLO_WORLD: &str = "Hello World!";
fn main_0() {
    // 访问静态变量
    println!("{}", HELLO_WORLD);
}

/* 静态变量(可变)和常量(不可变)的区别：
    a. 静态变量有固定的内存地址(使用变量值总会访问相同地址),
       常量则允许在任何被用到的时候复制其数据。
    b. 静态变量可以是可变的(虽然可能不安全[用 unsafe 包含])
*/
static mut COUNTER: u32 = 0;
fn add_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    // 修改可变静态变量
    add_counter(3);
    add_counter(3);

    unsafe {
        // 读写可变静态变量须置于 unsafe 块
        println!("counter: {}", COUNTER);
    }
}
