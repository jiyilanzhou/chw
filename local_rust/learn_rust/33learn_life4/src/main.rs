
// 方法中的生命周期
// 结构体
struct StuA<'a> {
    name: &'a str,
}

// 类比"泛型"
impl<'a> StuA<'a> {
    // 输出非引用(与" self "生命周期无关)
    fn do_something(&self) -> i32 {
        3
    }
    // 输出引用(其生命周期源于" self "生命周期)
    fn do_something2(&self, s: &str) -> &str{
    //fn do_something2<'b>(&'b self, s: &str) -> &'b str{
        self.name
    }
    /*
       1. 生命周期标识符不能冲突：
             即 " impl<'a> StuA<'a> ([自]因 'a 已绑定于"&self") "
       2. 编译器默认推导输出引用的生命周期为" self "生命周期：
             但不能推导明显与"非 self 参数"相关的输出引用的生命周期
             (故需显式标明生命周期)
    */
    fn do_something3<'c>(&self, s: &'c str) -> &'c str{
        s
    }
}

fn main() {
    let s = String::from("hello");
    let a = StuA{name: &s};
    println!("{}", a.do_something());

    let s2 = String::from("hello");
    println!("{}", a.do_something2(&s2));

    println!("{}", a.do_something3(&s2));
}
