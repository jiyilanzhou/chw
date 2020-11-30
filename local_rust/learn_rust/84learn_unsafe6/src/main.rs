
/*
实现不安全的 trait ： 在 trait 前添加 unsafe 关键字声明其不安全
    a. trait 内有方法包含编译器不能验证的不变量时为 unsafe trait
    b. unsafe trait 实现也必须用 unsafe 标记。

*/

// unsafe trait 及其具体实现结构体
unsafe trait Foo {
    fn foo(&self);
}
struct Bar();
unsafe impl Foo for Bar {
    fn foo(&self) {
        println!("foo method ... ");
    }
}

fn main() {
    let a = Bar();
    a.foo();

}