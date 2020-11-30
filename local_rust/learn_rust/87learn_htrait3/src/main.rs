
/*
3、完全限定语法
   定义： <Type as Trait>::function(.....)

*/

// trait 及其实现
trait A {
    fn print(&self);
}
trait B {
    fn print(&self);
}
struct MyType;
impl A for MyType {
    fn print(&self) {
        println!("A trait for MyType");
    }
}
impl B for MyType {
    fn print(&self) {
        println!("B trait for MyType");
    }
}
impl MyType {
    fn print(&self) {
        println!("MyType");
    }
}

fn main_0() {
    // 声明
    let my_type = MyType;

    // 等价于" MyType::print(&my_type); "
    my_type.print();

    // 限定：调用方法(类比"关联函数"调用[传入" self "])
    A::print(&my_type);
    B::print(&my_type);

}

// trait 及其实现
trait Animal {
    fn baby_name() -> String;
}
struct Dog;
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    // 结构体本身关联函数
    println!("baby_name: {}", Dog::baby_name());

    /* 编译报错：
       error[E0283]: type annotations required: cannot
                                    resolve `_: Animal`
         println!("baby_name: {}", Animal::baby_name());
                                   ^^^^^^^^^^^^^^^^^
       // [自]原因分析:因 trait 可有多个实现故需限定具体实现类型
    */
    // println!("{}",Animal::baby_name());//暂且注释:以通过编译

    // 完全限定语法(限定 trait 具体实现类型):调用关联函数
    println!("baby_name: {}", <Dog as Animal>::baby_name());

}