
// generic[dʒəˈnerɪk]adj.一般的，泛化(通用型)   // generics[dʒəˈnerɪks]n.泛型
// camel[ˈkæml]adj&n.驼色的,骆驼     // CamelCase 驼峰命名法
// blanket[ˈblæŋkɪt]v&n.覆盖,综合(的),毯状物
/*
0. 泛型、trait 与生命周期
   泛型是具体类型或其它属性的抽象替代(用于" 避免 "代码重复)

1. 泛型数据类型(p230)
   a. Rust 选择驼峰命名法(CamelCase)作为类型的命名规范。" T "作为" type "缩写
   b. " fn largest<T>(list: &[T]){ //... } "中参数" list:&[T] "为"T"值的切片

2. 在结构体定义中
   可在结构体定义中使用任意多个泛型，但过多的泛型会降低代码的阅读性。所以当代码
   中使用过多泛型时，可能就意味着需要重构为更小的片段

3. 在枚举定义中
   a. 标准库提供的 Option<T> 枚举是用来表示一个值可能存在的抽象概念，其源码如丅:
           enum Option<T>{
               Some(T),     // 持有 T 类型值的 Some(T) 变体
               None,        // 不持有任何值的 None 变体
           }
   b. 标准库提供的 Result<T,E> 枚举是用于操作可能成功亦可能失败的场景,其源码如下:
           enum Rusult<T,E>{
               Ok(T),           // 操作成功返回某个 T 类型的值
               Err(E),          // 操作失败返回某个 E 类型的值
           }

4. 在方法定义中
   a. 必须紧随着 impl 关键字声明 T 以便能够在实现方法时指定类型 StructName<T> ，
      通过在 impl 关键字之后的泛型 T 声明， Rust 才能识别出后续结构体中尖括号内的
      类型是泛型而不是具体类型
   b. 结构体定义中的泛型参数并不总是与方法签名上使用的类型参数一致。即某些情况下会
      有一部分泛型参数声明于 impl 关键字后，而另一部分泛型参数则声明于方法定义中。
      声明于 impl 关键字后的泛型(如" T, U ")用于识别结构体中尖括号内的类型是泛型
      而非具体类型；而定义于方法中的泛型(如" V, W ")仅与方法本身有关。(如下示例)

*/
struct Point<T, U> {
    x: T,
    y: U,
}

// impl 后声明 <T, U> 以致 Rust 才能识别出 Point<T, U> 中的 T、U 为泛型
impl<T, U> Point<T, U> {
    // 方法使用了与结构体定义不同的参数
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

/*
5. trait：定义共享行为(p241[*])

6. 为类型实现 trait (p242[*])
   孤儿规则(orphan rule)：避免破坏封装

7. 默认实现
    a. 特定类型实现 trait 时可选择性地重写默认行为
    b. 默认实现允许调用相同 trait 中其它(即使无默认实现的)方法
       (如此 trait 可提供很多有用功能而只需实现指定的部分内容)
    c. 无法在重载方法实现的过程中调用其默认实现

*/
// 定义 Summary trait
pub trait Summary {
    fn summarize_author(&self) -> String;
    // "有默认实现的方法"调用"未携带默认实现的 summarize_author 方法"
    fn summarize(&self) -> String {
        format!("(Read more from {})", self.summarize_author())
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
/* 只需实现 Summary 部分方法签名 ：
   实现" summarize_author "方法签名后其结构体实例即可调用 summarize
   (因 Summary 默认实现会调用已实现的" summarize_author "方法功能)
   // 注 : 无法在方法重载实现中调用默认方法
*/
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    /*  // 重载
        fn summarize(&self) -> String {
           // 无法在重载方法实现的过程中调用其默认实现
        }
    */
}

fn main_0() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}

/*
8. 使用 trait 作为参数

9. trait 约束(p248[*])
     a. " impl trait "常被用于较短的示例中，其本质是 trait bound 的语法糖如：
          pub fn notify(item: impl Summary) {
              println!("Breaking news! {}", item.summarize());
          }
         // 其本质(trait 约束)为
         pub fn notify<T: Summary>(item: T) {
             println!("Breaking news! {}", item.summarize());
         }
     b. " impl trait "及" trait bound "适用场景：
        (0). " impl trait "适用于短小示例(如参数列表中仅有单参数实现某个 trait)
        (1). " trait bound "适用于更复杂场景如" 获取两个实现了 Summary 的参数 "
     c. " impl trait "及" trait bound "实现对比 (item1,item2 皆实现 Summary)
        (0) 使用 impl Trait 语法 ：
            适用于单参数限定为不同类型(或实现同一 trait 的多参数为不同的具体类型)
            pub fn notify(item1: impl Summary, item2: impl Summary) { //... }
            // item1,item2 无需强制为同一类型(只需均实现 Summary trait 即可)
        (1) 使用 trait bound 语法 ：适用于存在多个参数限定为相同的具体类型
            pub fn notify<T: Summary>(item1: T, item2: T) { //... }
            // item1,item2 强制为同一类型(因为泛型 T 同时被指定为 item1,item2 的
               参数限制)

10. 通过" + "指定多个" trait bound "
     a. 若 notify 调用 summarize 方法的同时显示格式化后的 item，则 item 就必须实现
        两个不同 trait("Display、Summary")，可通过" + "语法实现:
            pub fn notify(item: impl Summary + Display) { //... }
     b. " + "语法也适用于泛型约束(trait bound):
            pub fn notify<T: Summary + Display>(item: T) { //... }
        // 通过指定两个 trait 约束可在 notify 函数体中调用 summarize 并使用 {} 来
           格式化 item

11. 使用 where 从句来简化 trait 约束
    在定义有多个泛型参数的列表间可能有较长 trait bound 信息，可能使得函数签名难以阅读)，
    可在函数签名后使用 where 从句来简化

12. 返回实现了 trait 的类型(p249[*])
    a. 可在返回值中使用 impl trait 语法，用于返回某种实现了 trait 的类型(但不确定
       返回的具体类型)
    b. 闭包和迭代器是重度依赖于" trait "的功能，这些功能会创建出"只有编译器知道或
       签名非常长"的类型。impl Trait 可精练地声明函数会返回实现了 Iterator trait
       的类型，而无需写出具体的类型，但 impl Summary 仅适用于返回单一具体类型情况。
      若尝试返回实现 Summary 的不同类型(如 NewsArticle 类型或 Tweet 类型则行不通
      [碍于 impl trait 工作方式的限制]：可用"面向对象的特性"解决)如：
          // 指定返回实现 Summary 的类型但尝试返回不同类型则报错：
          // (尝试返回 NewsArticle 或 Tweet 类型)
          fn returns_summarizable(switch: bool) -> impl Summary {
              if switch {
                  NewsArticle {
                      headline: String::from("Penguins win the Stanley Cup Championship!"),
                      location: String::from("Pittsburgh, PA, USA"),
                      author: String::from("Iceburgh"),
                      content: String::from("The Pittsburgh Penguins once again are the best
                      hockey team in the NHL."),
                  }
              } else {
                  Tweet {
                      username: String::from("horse_ebooks"),
                      content: String::from("of course, as you probably already know, people"),
                      reply: false,
                      retweet: false,
                  }
              }
          }
          // 编译错误信息：" error[E0308]: if and else have incompatible types "
                           (分析：if .. else .. 亦不允许返回不同类型)

13. 使用 trait 约束来修复 largest 函数(p251)
    (对应的原 largest 函数[p233])
    a. 在 largest 函数体中想要使用大于运算符( > )比较两个 T 类型的值需要
       在 T 的 trait bound 中指定 PartialOrd，如此 largest 函数才可用
       (因 PartialOrd 位于 prelude 中[无需手动引入作用域])
    b. 针对 " fn largest<T>(list: &[T]) -> T { // ... }
            fn largest<T>(list: &[T]) -> T {
                let mut largest = list[0];  // 飘红 ： Cannot move
                for &item in list.iter() {
                    if item > largest {
                        largest = item; // 飘红 ： Use of moved value
                    }
                }
                largest
            }
        // 编译错误 ：" error[E0369]: binary operation `>` cannot be applied to type `T` "
    c. (指定 PartialOrd )针对 " fn largest<T: PartialOrd>(list: &[T]) -> T { // ... }
            fn largest<T: PartialOrd>(list: &[T]) -> T {
                // 未实现 Copy 无法将 list[0] 中的值移出并绑定到 largest 变量上
                let mut largest = list[0];  // 飘红 ： Cannot move
                for &item in list.iter() {
                    if item > largest {
                        largest = item; // 飘红 ： Use of moved value
                    }
                }
                largest
            }
        // 编译错误 : " cannot move out of type `[T]`, a non-copy slice "
        //           " error[E0507]: cannot move out of a shared reference "
        // 分析:已知大小的类型(如 i32、char 等)可储存在栈上(其已实现 Copy trait)而 largest
                函数泛型参数可能未实现 Copy trait 即不能将 list[0] 值移至 largest 变量
    d. 为仅对实现 Copy 的类型调用(可在 T 的 trait bounds 中增加 Copy )
       参数实现 " PartialOrd 、 Copy 两个 trait " ：可调用

*/

/* 函数实现 PartialOrd 及 Copy trait 泛型 ：
    a. 若不希望限制 largest 函数仅用于实现 Copy trait 类型则可在 T 的 trait bounds 中指定
        Clone 而不是 Copy,克隆 slice 的每一个值使得 largest 函数拥有其所有权, clone 函数对于
       拥有堆上数据的类型(如String)会潜在分配更多堆上空间(堆分配涉及大量数据时其可能会相当缓慢)
    b. 另一种实现方式 无需任何 Clone 或 Copy 的 trait bounds 且不会有任何的堆分配
       仅返回在 slice 中 T 值的引用(如将函数返回值改为 &T 并改变使其函数体返回引用）
*/
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    /* a. 若未实现 Copy 则此处为将 list[0] 中的值移出并绑定到 largest 变量上
       b. 非引用(如" list[0] ")不可使用引用(如" &mut largest 或 &largest ")匹配
          如" let &mut largest = list[0]; 或 let &largest = list[0] "编译报错：
          " error[E0308]: mismatched types "
    */
    let mut largest = list[0];
    /* 模式匹配:" list.iter() "迭代出来的是类型 T 的引用即 &T ：
           a. 若直接使用 item 进行匹配则 item 类型为 &T，后续使用可能需要解引用
           b. 若使用 &item 匹配则 item 类型为 T，后续使用无需解引用
       // 引用(如"list.iter()"即" &T ")可用引用(如" &item "[模式匹配后 item 的类型即是 T ])
          或非引用(如 item [模式匹配后 item 的类型仍是引用即 &T])匹配
    */
    /* // 直接使用 item 匹配
        for item in list.iter() {
            if *item > largest {    // 解引用：使得前后匹配
                largest = *item;
            }
        }
        // 或者
        for item in list.iter() {
            if item > &largest {    // 取引用：使得前后匹配
                largest = *item;
            }
        }
    */
    // 使用 &item 匹配( [自]"&item"为模式解构)
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

/*
14. 使用 trait 约束来有条件地实现方法(p254[*])
    a. 为所有 T 类型实现方法:" impl<T> StructName<T> { //... } "
    b. 单独为实现了指定 trait 的类型编写方法
    c. 为满足 trait 约束的所有类型实现另一 trait 称为覆盖实现 blanket implementations
       (覆盖实现机制广泛用于 Rust 标准库)
        (1)如" 标准库为任何实现了 Display trait 的类型实现 ToString trait "类似：
            impl<T: Display> ToString for T {
                // --snip--
            }
        (2)因标准库存在 Display 的" blanket implementation (覆盖实现)"故可对任何实现
           Display trait 的类型调用由 ToString 定义的 to_string 方法如" 可将整型转换
           为对应 String 值(因整型实现了 Display )"
                let s = 3.to_string();
        (3) 有关 blanket implementation 描述会出现在 trait 文档的"Implementers"部分,
            trait 和 trait bound 可用泛型类型参数来减少重复并仍然能够向编译器明确指定
            泛型类型需要拥有哪些行为(因向编译器提供了 trait bound 信息[可检查代码中所
            用到的具体类型是否提供了正确的行为])
        (4) 在动态类型语言中若尝试调用一个类型并未实现的方法会在运行时出现错误.而Rust将
            这些错误移动至编译时甚至在代码能够运行前就强迫修复错误,且无需编写运行时检查
            行为的代码(因在编译时已检查过[相比其它不愿放弃泛型灵活性的语言有更好的性能])
    d. 另一种泛型即是生命周期( lifetimes ):
       (不同于其它泛型[确保类型拥有期望的行为])生命周期有助于确保引用在使用过程中一直有效

*/
// 为所有 T 类型实现方法
struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}
/* 单独为实现了指定 trait 的类型编写方法：
   只有 T 类型实现 PartialOrd trait(允许比较)及 Display trait(启用打印)
   的 Pair<T> 才会实现 cmp_display 方法
*/
// 根据泛型的 trait 约束有条件地实现方法
impl<T: std::fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

/*
15. 使用生命周期保证的有效性(p256[*])
    a. Rust 的每个引用都有自己的生命周期(lifetime),大多数候生命周期是隐式
       并可以推断的，正如大部分时候类型也是可以推断的一样。
    b. 当引用的生命周期可能以不同的方式相互关联时必须手动标注生命周期。
       (如同" 出现多种可能的类型时，须手动声明类型 "一样)
    c. Rust 需要注明泛型生命周期参数之间的关系来确保运行时实际引用一定有效

16. 生命周期避免悬垂引用
   生命周期的主要目标是避免悬垂引用，进而避免程序引用到非预期的数据

17. 借用检查器 :
    Rust 编译器拥有一个借用检查器(borrow checker)，其被用于比较不同的
    作用域并确定所有借用的合法性

18. 生命周期标注语法
    生命周期参数名称必须以撇号(')开头，其名称通常全是小写，类似于泛型其名称非常短(大多数
    默认使用 'a [生命周期参数标注位于引用 & 后，且通过空格来将标注名称与引用类型区分开]）
    (1). 生命周期注解并不改变任何引用生命周期的长短
    (2). 指定泛型生命周期后函数可接受生命周期的引用(类比函数签名指定泛型)
    (3). 生命周期注解描述多个引用生命周期相互的关系而不影响其生命周期
         &i32        // 引用
         &'a i32     // 带有显式生命周期的引用
         &'a mut i32 // 带有显式生命周期的可变引用
         // 单个生命周期的标注本身并没有多大意义(因生命周期注解主用于告知 Rust 多个引用
            的泛型生命周期参数之间的关系)

19. 函数签名中生命周期标注 ：
    (类比泛型参数)函数生命周期的标注同样需要在函数名与参数列表间的尖括号内声明(用以告知
    Rust 参数引用及返回值间生命周期的关联)

20. 深入理解生命周期
    a. 与返回值生命周期无关的参数无需指定其生命周期(因指定无意义)
    b. 返回值生命周期至少与一个参数生命周期相匹配,若返回的引用未指向任何参数则唯一可能
       即是指向函数内部创建值(将会是一个悬垂引用[因其在函数结束时离开作用域])
    c. 生命周期语法用于将函数参数与其返回值生命周期关联(有足够的信息来允许内存安全操作
       并阻止会产生悬垂指针亦或是违反内存安全的行为)

21. 结构体定义中的生命周期标注：
    a. 为结构体定义中的每个引用添加生命周期标注(因存放引用故其定义需添加生命周期注解)
    b. (类似泛型参数类型)须在结构体名称后尖括号内声明泛型生命周期参数以便在结构体定义
       中使用生命周期参数(标注意味着结构体实例不能比其相应字段中的引用存活更久)

22. 生命周期省略(p267[*])
    a. 每一个引用都有一个生命周期且需要为使用引用的函数或结构体指定生命周期参数
    b. 函数参数或方法参数的生命周期被称为输入生命周期(input lifetimes)，而返回值的
       生命周期被称为输出生命周期(output lifetimes)

23. 方法定义中的生命周期标注(p270[*])
    a. 当需要为某个拥有生命周期的结构体实现方法时，声明和使用生命周期参数的位置取决于
       是与结构体字段相关还是与方法参数、返回值相关(类比"泛型参数"语法)
    b. 结构体字段中的生命周期名称总是被声明在 impl 关键字之后，并被用于结构体名称之后，
       因为这些生命周期是结构体类型的一部分
    c. 在 impl 代码块的方法签名中，引用可能是独立的，也可能会与结构体字段中引用的生命
       周期相关联。另外生命周期省略规则在大部分情况下都可免去方法签名中的生命周期标注

24. 静态生命周期
   a. Rust 还存在一种特殊的生命周期 'static，其表示整个程序的执行期，
   b. 所有字符串字面量都拥有 'static 生命周期，可选择标注如下:
       let s: &'static str = "I have a static lifetime.";
       // 字符串的文本被直接储存在二进制程序中并总是可用的，故所有字符串字面值的生命
          周期都是 'static
   c. 将引用指定为 'static 前应考虑引用是否需要一直有效，大多情况下代码错误原因在于
      尝试创建悬垂引用或可用的生命周期不匹配，具体问题应从分析入手(而非一概将其指定
      为 'static 生命周期来解决)

25. 同时使用泛型参数、trait 约束 及 生命周期 :
    同一函数中指定泛型类型参数、trait bounds 和生命周期
    生命周期亦是泛型故生命周期参数 'a 和泛型类型参数 T 都位于函数名后同一尖括号列表中

*/
use std::fmt::Display;
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    /* ann 的类型是泛型 T 其可被放入实现 where 从句中指定 Display trait 的类型
      （在函数比较字符串 slice 的长度前被打印出来[ Display trait bound 原因])
    */
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}