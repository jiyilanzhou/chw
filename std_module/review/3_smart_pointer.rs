// 1. Box<T>
/* 编译报错：
error: expected mut or const in raw pointer type
     cusRef:*SN
            ^ expected mut or const in raw pointer type
   = help: use `*mut T` or `*const T` as appropriate
*/
struct SN {
    name: String,
    // cusRef: *SN      // 区别于 golang (Go 可用此表示自身引用)
    /* 即使将其更改为" cusRef: *mut SN "或" cusRef: *const SN "
       那也是"原生指针范畴"(而不再是简单"自身引用"的问题)
       // [自] Rust 中的 Box<T> 可能类比 Go 中类" *SN "的自身引用
    */
    // cusRef: *mut SN   // cusRef: *const SN
}

// 2. Deref trait 内" type Target: ?Sized; "
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    // 问题:此处为何非要显式标注"  type Target = T; "?
    /*解析此处" type Target "源于 Deref trait 其源码：
           pub trait Deref {
               /// The resulting type after dereferencing.
               #[stable(feature = "rust1", since = "1.0.0")]
               type Target: ?Sized;

               /// Dereferences the value.
               #[must_use]
               #[stable(feature = "rust1", since = "1.0.0")]
               fn deref(&self) -> &Self::Target;
           }
   */
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn main_3_0() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
}

/*
3. 选择 Box<T>、Rc<T> 或 RefCell<T> 场景：
   Rc<T>允许相同数据有多个所有者；而"Box<T>、RefCell<T>"仅有单一所有者
   Box<T>允许在编译时执行不可变或可变借用检查；Rc<T>仅允许在编译时执行不可变借用检查；
   RefCell<T>允许在运行时执行可变(即使自身不可变情况下亦可修改其内部值)或不可变借用检查
   // 注: 在不可变值内部改变值即是"内部可变性"模式

4. Rc<RefCell<Type> 与 RefCell<Rc<Type>> 区别与应用场景
    // 存放"RefCell"的"cons list"定义:可修改 Cons 成员所引用的数据
    enum List {
        /*  不同于"Cons(Rc<RefCell<i32>>, Rc<List>):可修改i32类型值",Cons 成员第二个元素
            是"RefCell<Rc<List>>:预期能够修改 Cons 成员所指向的 List"
        */
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }
    a. Rc<RefCell<Type> 适用于自身不可变的情况下修改其内的数据([自]如可用于实现不可变 trait
       但又需要修改内部数据的实现)。
       示例：用于修改内部数据
            let value = Rc::new(RefCell::new(5));  // 按需可修改故将 5 置于" RefCell "
            // 按需共享所有权故使用" Rc "
            let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
            println!("a before: {:?}", a);
            // " *value "解引用
            *value.borrow_mut() += 10;
    b. RefCell<Rc<Type>> 可用于修改链表循环如下；

 */
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
    // 定义链表 b
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    // 循环链表(参阅" 0_循环链表.png ")
    if let Some(link) = a.tail() {
        // 按需修改(故使用" RefCell<T> ")使得"a,b"形成循环链表
        *link.borrow_mut() = Rc::clone(&b); // " a.tail() "指向" b "
    }
    // 循环打印直至栈溢出
    println!("{:?}",a.tail())
}

/*
5. 泛型是否标注生命周期的区别
    // pub struct LimitTracker<'a, T: Messenger> {    // T 是否添加 'a 约束的区别[?]
    pub struct LimitTracker<'a, T: 'a + Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }

6. 为何 Rc<T> 仅适用于单线程场景
   Send 标记 trait 表明类型的所有权可以在线程间传递。几乎所有的 Rust 类型都是 Send 的,但也有
   一些例外(包括 Rc<T>:无法发送，因为如果克隆Rc<T>值并尝试将克隆的所有权转移到另一个线程，则
   两个线程可能会同时更新引用计数，为此 Rc<T> 被实现用于单线程场景(无需付出线程安全性能惩罚)

*/