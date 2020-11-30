
// downgrade[ˌdaʊnˈɡreɪd]n.降级,退步        // upgrade[ˈʌpɡreɪd]n.升级,增加
/* 弱引用 Weak<T> 特点：
    （1）弱引用通过 Rc::downgrade 传递 Rc 实例的引用，调用 Rc::downgrade 会得到 Weak<T> 类型
         的智能指针，同时将 weak_count 加 1 (而非 strong_count 加 1 )。其区别在于 weak_count
         无需计数为 0 就能使 Rc 实例被清理,只要 strong_count 为 0 即可
    （3）可通过 Rc::upgrade 方法返回 Option<Rc<T>> 对象
*/
use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons, Nil};
use std::rc::Weak;

#[derive(Debug)]
enum List {
    //Cons(i32, RefCell<Rc<List>>),
    Cons(i32, RefCell<Weak<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Weak<List>>> {
        // match 仅匹配" strong_count "(即不会匹配" weak_count ")
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Weak::new())));
    println!("1, a strong count = {}, weak count = {}",
             Rc::strong_count(&a), Rc::weak_count(&a));
    println!("1, a tail = {:?}", a.tail());
    println!("------------------------------------------------");

    let b = Rc::new(Cons(10, RefCell::new(Weak::new())));
    if let Some(link) = b.tail() {
        // 修改使得" b 指向 a "
        *link.borrow_mut() = Rc::downgrade(&a);
    }
    println!("2, a strong count = {}, weak count = {}",
             Rc::strong_count(&a), Rc::weak_count(&a));
    println!("2, b strong count = {}, weak count = {}",
             Rc::strong_count(&b), Rc::weak_count(&b));
    println!("2, b tail = {:?}", b.tail());
    println!("-----------------------------------------------");

    if let Some(link) = a.tail() {
        // 修改使得" a 指向 b "
        *link.borrow_mut() = Rc::downgrade(&b);
    }
    println!("3, a strong count = {}, weak count = {}",
             Rc::strong_count(&a), Rc::weak_count(&a));
    println!("3, b strong count = {}, weak count = {}",
             Rc::strong_count(&b), Rc::weak_count(&b));
    println!("3, a tail = {:?}", a.tail());

    /* Console :
         1, a strong count = 1, weak count = 0
         1, a tail = Some(RefCell { value: (Weak) })
         ------------------------------------------------
         2, a strong count = 1, weak count = 1
         2, b strong count = 1, weak count = 0
         2, b tail = Some(RefCell { value: (Weak) })
         -----------------------------------------------
         3, a strong count = 1, weak count = 1
         3, b strong count = 1, weak count = 1
         3, a tail = Some(RefCell { value: (Weak) })
  */

}
