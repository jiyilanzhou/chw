
// 调用 C 语言函数
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("abs(-3) : {}", abs(-3));
    }
}
