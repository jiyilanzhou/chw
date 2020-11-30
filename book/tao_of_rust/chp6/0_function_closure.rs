
// hint[hɪnt]v&n.提示,暗示,示意             // flat[flæt]adj&n.平坦(的)
// reserve[rɪˈzɜːv]v&n.预约(预定),保留      // reverse[rɪˈvɜːs]v&n.逆向,反转
/*
0. 函数、闭包、迭代器
   Rust 是一门混合范式的编程语言(有机融合了面向对象、函数式及泛型)

6.1 函数
    a. 可利用 Raw Identifier (原生标识符)"r#"为前缀将语言的关键字或保留字
       用作函数名(如函数名" r#match ").该语法一般用于 FFI 中避免 C 或其它
       语言的函数名和 Rust 关键字或保留字重名而引用的冲突
    b. Rust 中函数参数亦默认为不可变，同理可变操作需使用 mut 关键字声明。
       调用函数时"参数相当于重新声明的另一个变量绑定"如:
            fn modify(mut v:Vec<u32>){  // 参数声明为 mut 可变绑定
                v.push(3);
                println!("{:?}",v); // Console:"[0 3]
            }
            fn main(){
                let v = vec![0];        // 声明默认不可变
                // 值传递(若为引用传递则原变量 v 亦需声明为 mut
                //        即" let mut v = vec![0] ")
                modify(v);              // 传参即重新声明为 mut 可变绑定
            }

6.1.1 函数屏蔽([自]就近原则)
       fn f() { print!("1"); }
       fn main() {
           f(); // 2
           {
               f(); // 3
               fn f() { print!("3"); }     // 仅在当前块作用域有效
           }
           f(); // 2
           fn f() { print!("2"); }  // 屏蔽作用域外的同名函数
       }

6.1.2 函数参数模式匹配
    a. 函数中的参数等价于一个隐式的 let 绑定而 let 绑定本身是一个模式匹配的行为
       故函数参数支持模式匹配
    b. 函数参数可用" ref、mut ref、_ "来修饰，分别表示使用模式匹配来获取参数的
       "不可变引用、可变引用、忽略参数"。

6.1.3 函数返回值
    Rust 中的函数只能有唯一返回值，未显式声明返回值的函数相当于返回一个单元值()；
    若需返回多个值则可使用元组

6.1.6 高阶函数(p168[*])
    a. 在计算机科学里高阶函数是指以函数作为参数或返回值的函数
    b. 函数类型(Fn-Item Type)及函数指针类型(Fn-Pointer Type) :
       (1) let 声明必须显式指定函数指针类型如" let fn_ptr:fn() = funcName; "
       (2) let 声明并未指定函数指针类型则为函数类型如" let other_fn = funcName; "
       // 注：函数指针类型的性能开销高于函数类型故非必要则昼避免使用函数指针类型[*]
    c. 函数指针类型可用 type 关键字为其定义别名以便提高代码可读性

6.2 闭包：将自由变量(函数式编程中的称呼[即"外部环境变量"])和自身绑定的函数即是闭包
    a. 闭包(closure)通常指词法闭包
    b. 闭包类型" Fn()->i32 " ：以大写字母 F 开头的 Fn 是一个" trait "(而非函数指针
                              类型" fn()->i32 ")
    c. 闭包特性：延迟执行及捕获环境变量

6.2.1 闭包的基本语法
    a. 闭包由管道符("||")和花括号(或圆括号)组合而成
    b. 当闭包函数无参仅捕获自由变量时管道符内参数可省略如
        let (a,b) = (1,2);
        let add = || a + b
    c. 两个相同定义的闭包却不属于同一种类型如
           let c1 = || {};
           let c2 = || {};
           let v = [c1,c2];
           / * // 编译报错：
              error[E0308]: mismatched types
                 let v = [c1,c2];
                             ^^ expected closure, found a different closure
               = note: expected type `[closure@src\main.rs:8:14: 8:19]`
                          found type `[closure@src\main.rs:9:14: 9:19]`
               = note: no two closures, even if identical, have the same type
               = help: consider boxing your closure and/or using it as a trait object
          * /
    d. 装箱(Boxed)闭包 与 非装箱(Unboxed)闭包：(p174[?])

6.2.3 闭包与所有权(p178~p184[?])
    a. 闭包表达式会由编译器自动翻译为结构体实例并为其实现" Fn、FnMut、FnOnce "三个 trait
    b. "Fn、FnMut、FnOnce" trait (分别对应"&self、&mut self、self")与所有权关系(p178[*])
       "Fn"继承"FnMut","FnMut"又继承"FnOnce": 若要实现 Fn 则必须实现 FnMut 及 FnOnce;若
       要实现" FnMut "则必须实现" FnOnce "
    c. 复制语义类型自动实现 Fn
    d. 移动语义类型自动实现 FnOnce
    e. 使用 Move 关键字自动实现 Fn
       FnMut 的闭包在使用 move 关键字时，如果捕获变量是复制语义类型的则闭包会自动为其实现
       " Copy/Clone ";若捕获的变量是移动语义类型的则闭包不会自动为其实现" Copy/Clone "(
       出于保证内存安全的考虑)
    f. 修改环境变量以自动实现 FnMut
    g. 未捕获任何环境变量的闭包会自动实现 Fn
    h. 规则总结(p184[*])

6.2.4 闭包作为函数参数和返回值
    a. Rust 使用 trait 和匿名结构体提供的闭包机制非常强大
    b. 闭包作为 trait 对象 ：
       逃逸闭包(escape closure)、非逃逸闭包(non-escape closure)
       只有逃逸闭包才能装箱(p188[?])
    c. 闭包作为函数参数
    d. Rust 中的函数都默认实现" Fn、FnMut、FnOnce "这三个 trait
    e. 闭包作为函数返回值
       因闭包是 trait 的语法糖故无法直接作为函数返回值(欲将闭包作为返回值则须用 trait 对象)

6.2.5 高阶生命周期(p191[?])       // rank[ræŋk]v&n.排名,等级
    a. 高阶生命周期(Higher-Ranked Lifetime)
       亦称高阶 trait 绑定(Higher-Ranked Trait Bound,HRTB)
    b. " for<> "语法只能用于标注生命周期(不能用于其它泛型类型)

*/