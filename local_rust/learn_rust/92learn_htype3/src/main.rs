
/*
3、动态大小类型和 Sized trait   // unsized : 未确定大小
   动态大小类型:"dynamically sized types(DST)"或"unsized types",
   允许处理只有在运行时才知道大小的类型(典型代表: str )

*/

fn main() {
    /* 飘红报错:
          mismatched types [E0308] expected `str`, found `&str`
       编译报错：
          error[E0308]: mismatched types
            let s1: str = "hello";
                          ^^^^^^^ expected str, found reference
             = note: expected type `str`
                        found type `&'static str`

          error[E0277]: the size for values of type `str` cannot be known at
                                                           compilation time
             let s1: str = "hello";
                 ^^ doesn't have a size known at compile-time
             = help:the trait `std::marker::Sized` is not implemented for `str`
             = note: all local variables must have a statically known size
             = help: unsized locals are gated as an unstable feature

    */
    // let s1: str = "hello";   // 暂且注释：以通过编译

    let s2: &str = "world!";

}
/*
a. &str 有两个值即 str 的地址和长度:
   &str "大小、长度"皆占用"usize"大小空间(编译时已知),故其长度为" 2*usize "

b. 动态大小类型使用黄金规则：
   必须将动态大小类型值置于某种指针之后(如" &str、Box<str>、Rc<str> ")

c. trait (另一个动态大小类型)
   每一个 trait 都是可通过 trait 名称来引用的动态大小类型。为将 trait 用于
   " trait "对象须将其放入指针之后(如" &Trait、Box<Trait>、Rc<Trait> ")

*/

/*
Sized trait :
   为处理 DST，Rust 用 Sized trait 来决定一个类型的大小是否在编译时可知
   即若实现" Sized trait (编译器自动为已知大小的类型在编译时实现) "则在
   编译时可知否则不可知。如:
        fn generic<T>(t: T) {// T 为编译时已知大小的类型
            // --snip--
        }
   等价于：
       fn generic<T: Sized>(t: T) {// T 为编译时就知道大小的类型
            // --snip--
       }
   亦等价于"放宽限制(即编译时未知泛型 T 类型大小是否确定)"如下方式:
      fn generic<T: ?Sized>(t: &T) { // T 是或不是 Sized 皆可
           // --snip--
      }

*/