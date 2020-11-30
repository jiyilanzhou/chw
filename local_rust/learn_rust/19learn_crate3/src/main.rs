
mod modA {
    #[derive(Debug)]
    pub struct A {
        pub number: i32,
        name: String,
    }

    impl A {
        pub fn new_a() -> A {
            A {
                number: 1,
                name: String::from("A"),
            }
        }

        pub fn print_a(&self) {
            println!("number: {}, name: {}", self.number, self.name);
        }
    }

    pub mod modB {
        pub fn print_B() {
            println!("B");
        }

        pub mod modC {
            pub fn print_C() {
                println!("C");
                // 可用" super "访问父模块
                super::print_B();
            }
        }
    }
}

use modA::A;
use modA::A as A1;
fn main() {
    // 使用绝对路径
    let a = modA::A::new_a();
    a.print_a();

    //  use 引入使用相对路径
    let a = A::new_a();
    a.print_a();

    // use 引用且使用别名
    let a = A1::new_a();
    a.print_a();

    /* 结构体：
            pub struct A {
                pub number: i32,    // 字段公有
                name: String,       // 字段私有
            }
    */
    let number = a.number;  // 字段公有(可使用圆点符号"."直接访问)
    // let name = a.name;    // 字段私有(不可使用圆点符号"."直接访问)

    println!("+++++++++++++");
    modA::modB::modC::print_C();        // 绝对路径

}