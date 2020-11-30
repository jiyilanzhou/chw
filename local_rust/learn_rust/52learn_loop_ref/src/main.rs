
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    // 获取末尾元素(因可能为空故使用" Option<T> ")
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons, Nil};

fn main() {
    // 定义链表 a (使用" Rc<T> "只读共享)
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("1, a rc count = {}", Rc::strong_count(&a));   // 引用计数
    println!("1, a tail = {:?}", a.tail());     // 末尾元素
    println!("------------------------------");

    {
        // 定义链表 b
        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
        println!("2, a rc count = {}", Rc::strong_count(&a));
        println!("2, b rc count = {}", Rc::strong_count(&b));
        println!("2, b tail = {:?}", b.tail());
        println!("------------------------------");

        // 循环链表(参阅" 0_循环链表.png ")
        if let Some(link) = a.tail() {
            // 按需修改(故使用" RefCell<T> ")使得"a,b"形成循环链表
            *link.borrow_mut() = Rc::clone(&b); // " a.tail() "指向" b "
        }

        println!("3, a rc count = {}", Rc::strong_count(&a));
        // " a "指向" b "后" b "的计数亦增加
        println!("3, b rc count = {}", Rc::strong_count(&b));
        //println!("3, a tail = {:?}", a.tail()); // 死循环
        println!("------------------------------");
    }
    /* 内存泄露 :
        此处 b 的引用计数应为 1 (因 b 离开作用域其自身引用减为 0 但" a 仍指向 b
        [即 a 持有 b 的引用] "故 b 仍然存在并未真正释放从而导致" 内存泄露 ")
    */

    println!("4, a rc count = {}", Rc::strong_count(&a));

}