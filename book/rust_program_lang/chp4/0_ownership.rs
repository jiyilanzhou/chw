
// shallow[ˈʃæləʊ]n.(肤)浅的
/*
0. 所有权

1. 栈与堆
    a. 向栈上推入数据比在堆上分配更有效率一些，因为操作系统省去了搜索新数据存储位置
       的工作(这个位置永远处于栈的顶端)。由于多了指针跳转的环节所以访问堆上的数据要
       慢于访问栈上的数据
    b. 一般而言现代处理器在进行计算的过程中，由于缓存的缘故，指令在内存中跳转的次数
       越多，性能就越差

2. 所有权规则
    a. Rust 中的每一个值都有一个对应的变量作为它的所有者
    b. 在同一时间内，值有且仅有一个所有者
    c. 当所有者离开自己作用域时，它持有的值就会被释放

3. String 类型(p83[*])
    a. 字符串字面量是那些被硬编码进程序的字符串值，字符串字面量的确很方便但其并不能
       满足所有需要使用文本的场景。原因之一在于字符串字面量是不可变的，而另一个原因
       则在于并不是所有字符串的值都能够在编写代码时确定。
    b. 为应对获取用户输入并保存这种情况，Rust 提供了第二种字符串类型 String，这个
       类型会在堆上分配到自己需要的空间，所以它能够处理在编译时未知大小的文本。

4. 内存与分配(p84[*])
    a. 对于字符串字面量而言，由于在编译时就知道其内容，所以这部分硬编码的文本被直接
       嵌入到最终的可执行文件中。这就是访问客串字面量异常高效的原因，而这些性质完全
       得益于字符串字面量的不可变性。不幸的是没有办法将那些未知大小的文本在编译期统
       放入二进制文件中，更何况这些文本的大小还可能随着程序的运行而发生改变
    b. 对于 String 类型而言为了支持一个可变的、可增长的文本类型需要在堆上分配一块
       编译时未知大小的内存来存放数据，这同时也意味着：
       (0). 使用的内存是由操作系统在运行时动态分配出来的
       (1). 当使用完 String 时需要通过某种方式来将这些内存归还给操作系统。
       // [自]如"运行阶段"调用" String::from "时此函数才会按需请求所需的内存空间
    c. 浅度拷贝(shallow copy)与深度拷贝(deep copy)
       (0). Rust 中使用 move 语义(没有浅度拷贝)
       (1). 设计原则：Rust 永远不会自动地创建数据的深度拷贝，因此 Rust 中任何自动
            的赋值操作都可以被视为高效的

5. 变量和数据交互的方式：克隆
    当其实需要去深度拷贝 String 堆上的数据，而不仅仅是栈数据时可用 clone 方法如：
        let s1 = String::from("hello");
        let s2 = s1.clone();    // 使用 clone 复制堆上数据
        println!("s1 = {}, s2 = {}",s1,s2); // Console:"s1 = hello,s2 = hello"
    // 当某处调用了 clone 时应该知道某些特定的代码将会被执行且可能会消耗相当资源。

6. 栈上数据的复制(p90[*])
    a. (示例):
            let x = 5;
            //let y = x.clone();
            let y = x; // 即便没有调用 clone，x 赋值于 y 后亦仍然有效且未发生移动
            println!("x = {}, y = {}", x, y);
       // 类似标量类型(如"整型")在编译时可确定大小且能将数据完整存储于栈中，对于这些值
          的复制操作永远都是非常快速的。这也意味着在创建变量 y 后没有任何理由去阻止变量
           x 继续保持有效。换言之对于这些类型而言，"深度拷贝"与"浅度拷贝"没有任何区别即
          是否调用 clone 并无区别(因此无需考虑)
    b. Copy trait 与 Drop trait (p90)
       一旦某种类型本身或其任意成员实现了 Drop trait 则 Rust 不允许其实现 Copy trait。
       尝试给某个需要在离开作用域时执行特殊指令的类型([自]即实现了" Drop trait "的类型)
       实现 Copy trait 则会导致编译错误。
    c. 可通过查看特定类型文档来确定类型是否是 Copy 的。但一般来说" 任何简单标量的组合 "
       都可以是 Copy 的，任何需要分配内存或某种资源的类型都不会是 Copy 的
    d. 一些拥有 Copy 这种 trait 的类型：
        (0). 标量类型：整型、布尔类型、浮点型、字符类型
        (1). 若元组包含的所有字段类型都是 Copy 的那么此元组也是 Copy 的如" (i32,i32) "
             是可 Copy 的而" (i32,String) "则不是

7. 引用与借用(p94[*])
    a. & 代表的就是"引用"语义，允许在不获取所有权的前提下使用值。同理函数签名中 & 用来
       表明参数类型是一个引用([自]类比 Golang 中的指针" * ")
    b. 通过引用传递参数的方式也被称为"借用(borrowing)"
    c. 原书图例" 图 4-5 "(p95[*])

8. 可变引用
    a. 对于特定作用域中的特定数据来说，一次只能声明一个可变引用
    b. 数据竞争(p98)
       (0). 两个或两个以上的指针同时访问同一空间
       (1). 其中至少有一个指针会向空间中写入数据
       (2). 没有同步数据访问的机制
    c. 不能同时拥有可变引用和不可变引用。但同时存在多个不可变引用是合理的(因其只读)
    d. 引用规则：在任何给定一段时间里只有拥有单个可变引用或任意数量的不可变引用

9. 切片 slice (p101[*])
    a. 除了引用，Rust 还有另外一种不持有所有权的数据类型：切片(slice)
    b. 切片允许引用集合中 某一段连续的"元素序列"而不是 整个集合
    c. "  for (i, &item) in bytes.iter().enumerate() { //... } "既然 enumerate
       方法返回的是一个元组则可使用模式匹配来解构(其中 i 是元素中的索引部分，而 &item
       则是元组中指向集合元素的引用)。由于从" .iter().enumerate() "中获取的是产生引用
       元素的迭代器，所以在模式中使用了" &item "(参阅"原书示例 4-7/8 ")

10. 字符串切片(p104[*])
    a. 字符串切片可为栈空间指向 String 对象中某个连续部分的引用如
           let s = String::from("hello world");
           let hello = &s[0..5];
           let world = &s[6..11];
        // 其中" [start .. end] "的声明是告诉编译器创建一个 String  切片
           引用(而非" 整个字符串 "的引用)
        // 可在一对方括号中指定切片的范围区间" [starting_index .. ending_index] ",
           切片数据结构在内部存储了"指向起始位置的引用和一个描述切片长度的字段(切片
           长度字段等于" ending_index - starting_index ")"
    b. 字符串字面量就是切片,其变量类型即是"&str(不可变引用)"故字符串字面量亦是不可变的
    c. 字符串切片的边界必须位于有效的 UTF-8 字符边界内，尝试从一个多字节字符的中间创建
       字符串切片会导致运行时错误
    d. 字符串切片的类型写作 &str (一个指向二进制程序特定位置的切片)

11. 将字符串切片作为参数
    有经验的开发者会将" fn first_word(s: &String) -> &str{ // ... } "函数的声明优化
    为" fn first_word(s: &str) -> &str{ // ... } "，使其可同时处理 String 与 &str。
    (因当持有字符串切片时可直接调用此函数，若持有 String 时可创建完整 String 切片作为
    参数。在定义函数时使用字符串切片来代替字符串引用会使 API 更加通用且不损失任何性能)

*/
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let my_string = String::from("hello world");
    /* 飘红报错：类型不匹配
            Cannot move
            mismatched types [E0308] expected `&str`, found `str
       // 解析： " my_string[..] "是" str "，" &my_string[..] "才是" &str "
                 (具体结构可参见原书图例" 图 4-5 ")
    */
    // let word = first_word(my_string[..]);
    // first_word 可以接收 String 对象的切片作为参数
    let word = first_word(&my_string[..]);
    // [自]对切片的引用还是切片，故可用任意个" & "与" my_string_literal "拼接
    let word = first_word(&&my_string[..]);
    let word = first_word(&&&my_string[..]);

    let my_string_literal = "hello world";
    // first_word 可以接收字符串字面量的切片作为参数
    let word = first_word(&my_string_literal[..]);
    // 由于字符串字面量本身就是切片，故可直接将其作为参数而无需使用额外的切片语法!
    let word = first_word(my_string_literal);
    // [自]对切片的引用还是切片，故可用任意个" & "与" my_string_literal "拼接
    let word = first_word(&my_string_literal);
    let word = first_word(&&my_string_literal);

}