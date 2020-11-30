
// 提前清理 : Rust 提供" std::mem::drop() "

struct Dog {
    name: String,
}

impl Drop for Dog {
    fn drop(&mut self) {
        println!("{} leave", self.name);
    }
}

// rust 提供" std::mem::drop() "
fn main() {
    let a = Dog{name: String::from("wangcai")};
    let mut b = Dog{name: String::from("dahuang")};

    /* 提前释放: 禁止显式调用" drop "
       飘红报错:" Explicit calls to `drop` are forbidden.
                 Use `std::mem::drop` instead [E0040] "
       编译报错:
       error[E0040]: explicit use of destructor method
           b.drop();
             ^^^^ explicit destructor calls not allowed
    */
    // b.drop();

    drop(b);
    drop(a);

}