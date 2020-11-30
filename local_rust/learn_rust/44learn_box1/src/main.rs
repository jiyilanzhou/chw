
/* // detect[dɪˈtekt]v.检测,探测
box 适用场景：
   (1)当有一个在编译时未知大小的类型，而又需要再确切大小的上下文中使用这个类型值的时候；
      （举例子：在一个 list 环境下，存放数据，但是每个元素的大小在编译时又不确定）
   (2)当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候；
   (3)当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型时。

*/
use List::Cons;
use List::Nil;

/* //循环嵌套:
    enum List {
        Cons(i32, List),    // 编译时未知 List 大小类型
        Nil,
    }
    fn main() {
        / * 编译报错: 递归类型拥有无穷大小
            error[E0072]: recursive type `List` has infinite size
                 enum List {
                 ^^^^^^^^^ recursive type has infinite size
                     Cons(i32, List),
                               ---- recursive without indirection
                  = help: insert indirection (e.g., a `Box`, `Rc`, or `&`)
                          at some point to make `List` representable

            error[E0391]: cycle detected when processing `List`
                 enum List {
                 ^^^^^^^^^
                   = note: ...which again requires processing `List`,completing the cycle
                   = note: cycle used when computing dropck types for `Canonical {
                           max_universe: U0, variables: [], value: ParamEnvAnd { param_env:
                           ParamEnv { caller_bounds: [], reveal: UserFacing, def_id: None }
                           , value: List } }`
            // 处理时检测到循环 List        // detect[dɪˈtekt]v.检测,探测
        * /
        let list = Cons(1, Cons(2, Cons(3, Nil)));
    }

    // 类比 C 语言中的定义如下:
        struct List {
            int value;
            struct List l;          // 结构体无限循环嵌套
            // struct List *next;
        };

*/

/* 使用 Box<T> 智能指针解决无限循环问题:
   // 类比 C 语言中的如下定义：
    struct List {
        int value;
        // C 语言使用指针解决"结构体递归嵌套"问题
        struct List *next;  // 使用指针(占用空间大小一定)
        //struct List l;    // 避免使用结构体无限循环嵌套
    };

*/
// Rust 使用 Box<T> 智能指针解决 List 无限循环问题:
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    // 使用 Box<T> 创建链表
    let list = Cons(1,
                    Box::new(Cons(2,
                                  Box::new(Cons(3,
                                                Box::new(Nil))))));
    println!("Hello World!");
}