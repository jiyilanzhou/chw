/*
0. 容器类型:Rust 中容器类型可分为两大类(图"14_容器类型.png")
    a. 第一类是"可共享的可变容器"，其支持一种内部可变性(即在不可变基础上实现可变)
    b. 第二类是"数据集合容器"，Rust 标准库支持动态数组及 HashMap 等集合类型

1. (容器类型)内部可变性(interior mutability):
    a. 与继承式可变相对应
       因为 Rust 默认不可变，继承式可变是 Rust 默认的强制行为，在实际开发中使用继承式可变不是很灵活，所以
       标准库增加了可变性功能
    b. 由可变性核心原语 UnsafeCell <T> 泛型类型提供支持
       内部可变性本质简单，就是把容器内的 T 原始可变指针 *mut T 传出
    c. 基于 UnsafeCell <T> 提供了 Cell<T> 和 RefCell<T>
        UnsafeCell 是 Safe Rust 中唯一合法的把不可变引用转变为可变指针的方法，标准库又基于 UnsafeCell<T>
        实现了两个内部可变性容器( Cell 及 RefCell )

*/
use std::cell::Cell;

struct Foo {
    x: u32,
    /*  Cell 类型：
            a. 可提供的 get 方法可用于获取容器内的值，亦可通过 set 方法来修改容器内的值
            b. Cell 的 get 方法只能用于实现了 Copy 的类型，其相应源码如下:
                   impl<T: Copy> Cell<T> {
                       #[inline]
                       #[stable(feature = "rust1", since = "1.0.0")]
                       pub fn get(&self) -> T {
                           // SAFETY: This can cause data races if called from a separate thread,
                           // but `Cell` is `!Sync` so this won't happen.
                           unsafe { *self.value.get() }
                       }
                       #[inline]
                       #[unstable(feature = "cell_update", issue = "50186")]
                       pub fn update<F>(&self, f: F) -> T
                       where
                           F: FnOnce(T) -> T,
                       {
                           let old = self.get();
                           let new = f(old);
                           self.set(new);
                           new
                       }
                   }
            c.
     */
    y: Cell<u32>,
}

fn main_0() {
    let foo = Foo { x: 1, y: Cell::new(3) };
    assert_eq!(1, foo.x);
    /* 可用 Cell 类型提供的 get 方法获取容器内的值，亦可通过 set 方法来修改容器内的值。但对于字段 y 而言其
       本身是不可变的(因" let foo "默认为不可变绑定)，它只是通过 Cell 来获得内部可变性。
     */
    assert_eq!(3, foo.y.get());
    // 通过 set 方法来修改容器内的值
    foo.y.set(5);
    assert_eq!(5, foo.y.get());

    let s = "hello".to_string();
    let bar = Cell::new(s);
    /* into_inner 方法可将容器内的值转换出来([自]获取所有权): 其源码如下
          impl<T> Cell<T> {
               pub const fn new(value: T) -> Cell<T> {
                   Cell { value: UnsafeCell::new(value) }
               }
               #[inline]
               #[stable(feature = "rust1", since = "1.0.0")]
               pub fn set(&self, val: T) {
                   let old = self.replace(val);
                   drop(old);
               }
               #[inline]
               #[stable(feature = "move_cell", since = "1.17.0")]
               pub fn swap(&self, other: &Self) {
                   if ptr::eq(self, other) {
                       return;
                   }
                   unsafe {
                       ptr::swap(self.value.get(), other.value.get());
                   }
               }
               #[stable(feature = "move_cell", since = "1.17.0")]
               pub fn replace(&self, val: T) -> T {
                   mem::replace(unsafe { &mut *self.value.get() }, val)
               }
               #[stable(feature = "move_cell", since = "1.17.0")]
               pub fn into_inner(self) -> T {
                   self.value.into_inner()
               }
          }
        // 注：可看得出来 Cell 容器是通过移进移出值来达到内部可变的目的。若想要使用引用则需使用 RefCell
               容器类型
     */
    let x = bar.into_inner();
    /* 编译报错：
        error[E0382]: use of moved value: `bar`
             let bar = Cell::new(s);
                 --- move occurs because `bar` has type `Cell<String>`, which does not implement
                                                                                the `Copy` trait
             let x = bar.into_inner();
                         ------------ `bar` moved due to this method call
             bar;
             ^^^ value used here after move
     */
    // bar;     // 暂且注释以通过编译
}

use std::cell::RefCell;
fn main() {
    /* 在 RefCell 容器置入动态可增长数组(动态数组不能实现 Copy ):
           通过 borrow 及 borrow_mut 来操作内部的值，分别对应 Rust 不可变引用及可变引用。 RefCell 适于
           没有实现 Copy 的类型，有运行时开销，一个在运行时而不是在编译时执行借用规则的类型。
     */
    let x = RefCell::new(vec![1, 2, 3, 4]);
    println!("{:?}",x.borrow());
    x.borrow_mut().push(5);
    println!("{:?}",x.borrow());

    // 运行时借用检查
    let x = RefCell::new(vec![1, 2, 3, 4]);
    let mut mut_v = x.borrow_mut();
    mut_v.push(5);
    /* 违反借用检查：第二次借用违反了"可变借用独占"的规则
       运行时错误：thread 'main' panicked at 'already borrowed: BorrowMutError'
     */
    // let mut mut_v2 = x.borrow_mut();     // (可正常编译)暂且注释以通过运行

}
