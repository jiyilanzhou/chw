
// 引用计数智能指针
/*
// 本地定义还需使用"use"?(待理解[?])
// 解析： 虽为本地定义可无需导入但每次使用变体均需添加前缀(如" List::Cons... ")
//        较为繁锁，故一般采用 use 导入直接使用变体(如" Cons... ")即可.
use crate::List_0::{Cons, Nil};

enum List_0 {   // 定义枚举体
    Cons(i32, Box<List_0>),
    Nil,
}

fn main() {
    / * 定义链表(结构参见" 0_引用计数.png "):编译报错
       error[E0382]: use of moved value: `a`
           let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
               - move occurs because `a` has type `List_0`, which does
                                        not implement the `Copy` trait
           let b = Cons(3, Box::new(a));
                                    - value moved here
           let c = Cons(4, Box::new(a));
                                    ^ value used here after move
       // " a "移至" b "后不可再使用于" c "
       // 解决方案:使用"Rc<T>"智能指针(可解决"多个引用即共享所有权的问题")
    * /
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a)); // 飘红报错:" Use of moved value "
}
*/

/* 使用 Rc<T> 智能指针:
    " Rc<T> "源码： // RC : "  reference-counting "
    /// A single-threaded reference-counting pointer. 'Rc' stands for
    /// 'Reference Counted'.
    pub struct Rc<T: ?Sized> {
        ptr: NonNull<RcBox<T>>,
        phantom: PhantomData<T>,
    }

*/
use std::rc::Rc;
use crate::List::{Cons, Nil};
// 定义枚举体
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    /* 定义链表(结构参见" 0_引用计数.png ")
       "Rc::new(prm)"源码：" pub fn new(value: T) -> Rc<T> {//... } "
    */
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    //let b = Cons(3, Rc::clone(&a));  // 写法1：" Rc::clone(&a) "
    let b = Cons(3, a.clone());        // 写法2: " a.clone() "
    let c = Cons(4, Rc::clone(&a));
}
