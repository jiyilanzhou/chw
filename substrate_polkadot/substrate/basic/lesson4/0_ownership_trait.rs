/*
0. Ownership、Lifetime
   RALL : 一种可精确计算资源和变量生命的分析机制
          (Resource Acquisition Is Initialization)

1. Trait、Generic
    a. Trait 为具体类型的抽象(duck 模型)
    b. Trait 设计：Trait 用于建模，可用来给事物行为分类，分为多个 Trait
    c. Trait Bound 与泛型配合，对泛型的可能类型集进行限定(缩小集合取值空间)
    d. 泛型 ：泛指的类型(具体可为实现某个 Trait 的具体类型)。当泛型取值固定
              时使用 enum 是更好的替代

2. Iterator
    a. 迭代器用于对一个元素序列进行迭代(遍历)
    b. for 循环能实现的功能迭代器都能实现
       (忘掉传统的 for 循环，使用迭代器更加安全和高效)

3. macro
    a. 声明宏(declaretive)
    b. 过程宏(procedural)：分为三类
      （0）自定义[# derive]宏：可用在 struct 和 enum 上
      （1）类属性(Attribute-like)宏：可作用在几乎所有条目上
      （2）类函数(Function-like)宏：接收一串 token 作为其参数即是可直接操作
           代码 token (把人为升级为编译器,强大到令人发指)

*/
// Trait、Generic
trait Summary {
    fn summarize(&self) -> String;
}

fn notify_0<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// item1、item2 为实现 Summary 的同一类型
fn notify_1<T: Summary>(item1: &T, item2: &T) {}

// item1、item2 可为实现 Summary 的不同类型
fn notify_2(item1: &impl Summary, item2: &impl Summary) {}


/* // Iterator
// 标准库定义的 Iterator trait
pub trait Iterator {
    type Item;
    // 关联类型
    fn next(&mut self) -> Option<Self::Item>;
}
 */

struct Counter {
    count: u32
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// [自]" std::iter::Iterator "位于 prelude* 模块(自动导入)
//impl std::iter::Iterator for Counter {
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let c = Counter::new();
    // for i in c.into_iter() {
    for i in c {
        print!("{}\t", i);  // Console:" 1 2 3 4 5 "
    }
}