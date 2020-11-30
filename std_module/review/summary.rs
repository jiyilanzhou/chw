
/*
0. 字符串与字节相互转换
    a. 字符串转为字节" &str.char() "如
         // 字符串转为字节
         for b in "静心道".bytes() {
             print!("{} \t", b)
             // console : " 233  157  153  229  191  131  233  129  147 "
         }
     b. 字节转为字符串" string::from_utf8_lossy(&str) "
            // 字节到字符串
            let str = vec![233, 157, 153, 229, 191, 131, 233, 129, 147];
            let str = String::from_utf8_lossy(&str);
            println!("\n {} ",str)  // console : " 静心道  "

1. 为何 Rc<T> 仅适用于单线程场景
   Send 标记 trait 表明类型的所有权可以在线程间传递。几乎所有的 Rust 类型都是 Send 的,但也有
   一些例外(包括 Rc<T>:无法发送，因为如果克隆Rc<T>值并尝试将克隆的所有权转移到另一个线程，则
   两个线程可能会同时更新引用计数，为此 Rc<T> 被实现用于单线程场景(无需付出线程安全性能惩罚)

2. 使用泛型与 trait 对象的区别及各自适用场景

 */
// Draw trait
pub trait Draw {
    fn draw(&self);
}
// (屏幕)结构体及其实现
pub struct Screen {
    // 屏幕组件
    pub components: Vec<Box<dyn Draw>>,  // trait 对象(使用 dyn 关键字)
}
impl Screen {
    pub fn run(&self) {
        for comp in self.components.iter() {
            comp.draw();
        }
    }
}

// (使用泛型)屏幕结构体及其实现
pub struct Screen2<T: Draw> {
    pub components: Vec<T>,
}
impl<T> Screen2 <T>
    where T: Draw {
    pub fn run(&self) {
        for comp in self.components.iter() {
            comp.draw();
        }
    }
}

// (屏幕组件)结构体及其实现
pub struct Button {
    pub width: u32,     // 宽
    pub height: u32,    // 高
    pub label: String,  // 标签
}
pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub option: Vec<String>,
}
impl Draw for Button {
    fn draw(&self) {
        println!("draw button, width = {}, height = {}, label = {}",
                 self.width, self.height, self.label);
    }
}
impl Draw for SelectBox {
    fn draw(&self) {
        println!("draw selectBox, width = {}, height = {}, option = {:?}",
                 self.width, self.height, self.option);
    }
}

fn main() {
    // 创建屏幕(内附组件)
    let s1 = Screen {
        // 组件:内置组件具体实例
        components: vec![
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("ok"),
            }),
            Box::new(SelectBox {
                width: 60,
                height: 40,
                option: vec![
                    String::from("Yes"),
                    String::from("No"),
                    String::from("MayBe"),
                ],
            }),
        ],
    };
    s1.run();

    /* 使用泛型 : (编译错误)类型匹配错误
       error[E0308]: mismatched types
          /   Button {
          |       width: 50,
          |       height: 10,
          |       label: String::from("ok"),
          |   },
          |___^ expected struct `std::boxed::Box`,found struct `gui::Button`
          = note: expected type `std::boxed::Box<dyn gui::Draw>`
                     found type `gui::Button`
        // 原因分析：使用泛型为静态分发(单态化[编译时已确定具体类型])[?]
        // 解决方案：可用动态分发" dyn "(类似多态[通过 dyn trait 指针指向具体对象])
    */
    /* 暂且注释：以通过编译
        let s2 = Screen {
            components: vec![
                Button {
                    width: 50,
                    height: 10,
                    label: String::from("ok"),
                },
            ],
        };
        s2.run();
    */

}