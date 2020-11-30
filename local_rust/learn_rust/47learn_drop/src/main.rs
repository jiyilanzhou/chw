
/* Drop trait : 当值离开作用域时执行此函数的代码
   (类似于其它语言中的析构函数[清理对象或值时编译器自动调用的函数])

*/
struct Dog {
    name: String,
    //count: i32,
}
// 实现 Drop trait
impl Drop for Dog{
    // 释放即是对其修改故需" &mut "(如释放减少引用计数)
    fn drop(&mut self) {
        println!("Dog {} leave", self.name);
        //self.count -= 1;
    }
}

fn main() {
    let a = Dog{name: String::from("wangcai")};
    { 
        let b = Dog{name: String::from("dahuang")};
        println!(" --------------------- ");
    }
    println!(" --------------------- ");

}
