
// scalar[ˈskeɪlə(r)]n.标量
// lossy[ˈlɔːsi]adj&n.有损(耗的),失真
/*
0. &str VS String :
    a. Rust 中的字符串主要涉及两种类型( &str 及 String )
    b. String：堆分配、可增长
    c. &str：指向相同内存区域，只在不同偏移量处开始和结束

1. 字面量
    a. 字面量(literal)：源代码中一个固定值的表示法（notation）诸如"整数、
                       浮点数及字符串"等都是字面量
    b. 字符串字面量(stringliteral)是指双引号引住的一系列字符

2. 字符串切片(p104[*])
    a. 字符串切片可为栈空间指向 String 对象中某个连续部分的引用如
           let s = String::from("hello world");
           let hello = &s[0..5];
           let world = &s[6..11];
        // 其中" [start .. end] "的声明是告诉编译器创建一个 String  切片
          引用(而非" 整个字符串 "的引用)
    b. 字符串字面量就是切片

3. 字符串是什么(p183[*])
    a. Rust 在语言核心部分只有一种字符串类型，那就是字符串切片 str, 它通常
       以借用的形式(&str)出现。字符串切片是一些指向存储在别处的 UTF-8 编码
       字符串的引用。例如"字符串字面量"的数据被存储在程序的二进制文件中，而
       它们本身也是字符串切片的一种
    b. String 类型被定义于 Rust 标准库中而没有被内置在语言的核心部分。一般
       当 Rust 开发者提到" 字符串 "时其通常指的是 String 与字符串切片 &str
       两种类型而非其中一种
    c. Rust 标准库亦提供 OsString、OsStr、CsString 及 CStr 等一系列字符串
       类型。其以" String 或 Str "结尾表明类型提供的是所有者还是借用版本。

4. 创建一个新的字符串
   String::from("string_literal") 等同于 "string_literal".to_string()

5. 字符串索引(p188[*])
    在许多编程语言中，往往可合法地通过索引来引用字符串中每一个单独的字符但
    在 Rust 语言中禁止通过索引来获得 String 中的字符(编译直接 panic! [因
    未必正好能索引至字符有效边界])

6. 内存布局
    String 实际上是一个基于 Vec<u8> 的封装类型故对于 Vec<T> 的诸多操作亦
    适用于 String

7. 字节、标量值及字形簇(p190[*])
   a. 使用 UTF-8 编码还会引发另外一个问题。在 Rust 中实际上可通过 3 种不同
      方式来看待字符串数据：字节、标量值和字形簇(最接近人眼中的"字母"概念)
   b. Rust 中的字符串并不支持索引
        let s1 = String::from("hello");
        let h = s1[0];      // 编译报错
      // 因为字符串中的字节索引并不总是对应到一个有效的 Unicode 标量值
      // Rust 不允许通过索引来获得 String 中的字符另一原因：索引操作的
         复杂度往往会被预期为常数时间 O(1)，但在 Rust 中的 String 中却
         无法保障此性能，因为 Rust 必须要遍历从头至索引位置的整个内容来
         确定究竟有多个合法的字符串存在。

8. 搜索" 字形簇 "
   a. 引用 base
      内部实现 :
       1. String 是一个 Vec<u8> 的封装
          a. let len = String::from("Hola").len();
             "let=4"意味着储存字符串"Hola"的Vec长度是4个字节(每个字母的UTF-8编码都占1个字节)
          b. let len = String::from("Здравствуйте").len();
             "let=24"意味着储存字符串"Здравствуйте"的Vec长度是24个字节(每个 Unicode 标量值需
             占用2个字节),因此一个字符串字节值的索引并不总是对应一个有效的 Unicode 标量值
          c. let hello = "Здравствуйте";
             let answer = &hello[0];
             //  З 的第一个字节是208第二个是151故 answer 实际上应该是 208，但208自身并非有效
                字母,返回208并非请求者所期望(但确是 Rust 在字节索引 0 位置所能提供的唯一数据)。
                比如即便 &"hello"[0] 是返回字节值的有效代码，也应当返回 104 而不是 h。
                为避免返回意外值并造成不能立刻发现的bug其Rust根本不会编译(编辑报错[及早杜绝])
       2. 字节、标量值及字形簇
            从 Rust 角度而言事实上有三种相关方式可以理解字符串：字节、标量值和字形簇
          a. 梵文书写的印度语单词 “नमस्ते”其最终储存在 vector 中的 u8 值类似：
              [224,164,168,224,164,174,224,164,184,224,165,141,224,164,164,224,165,135]
          b. 共有 18 个字节(即计算机最终储存数据),若从Unicode标量值角度理解则类似Rust的 char 类型：
              ['न', 'म', 'स', '्', 'त', 'े'] //有6个char但第4、6个是发音符号[本身无意义]而非字母
          c. 若以字形簇的角度理解则会得到所期望的构成这个单词的4个字母
              ["न", "म", "स्", "ते"]
              // Rust提供多种不同式来解释计算机储存的原始字符串数据(如此程序可选择其需要的表现方式)
        3. Rust 不允许使用索引获取 String 字符的另外原因
             索引操作预期总是需要常数时间 (O(1)),但对于String不可能保证这样的性能(因 Rust须遍历从
             起始到索引位置来确定有多少有效字符）

9. 字符串切片(p191[*])
   尝试通过索引引用字符串通常是一个坏主意，因为字符串索引操作返回的类型不确定：究竟应该是字节、字符
   还是字形簇、甚至是字符串切片呢，因此如果真想要使用索引来创建字符串切片则 Rust 会要求做出更加明确
   的标记。即为明确表明需要一个字符串切片则需要在索引的" [] "中填写范围即" [start_idx..end_idx] "
   (而非单数字索引即" [ idx ] ")来指定所需的字节内容。
   切忌要小心谨慎地使用范围语法创建字符串切片，因错误指令会导致程序崩溃(比如索引非有效字符边界)。
   错误示例： let hello = "Здравствуйте";
             let s = &hello[0..1];        // 索引范围非有效字符边界则" panic! "

10. 搜索"字符串切片"
    a. 引用 dive
        6.2.1 &str (p74~75[*])
            a. str 是 Rust 的内置类型， &str 是对 str 的借用。Rust 的字符串内部默认是使用
               utf-8 编码格式，而内置的 char 类型是 4 个字节长度(存储的是 Unicode Scalar
               Value )故 Rust 中的字符串不能视为 char 类型的数组而更接近 u8 类型的数组
            b. str 类型有一种方法" fn as_ptr(&self) -> *const u8 "其内部无需任何计算只需
               强制类型转换即可
            c. 寻找字符串 s 内部的第 n 个字符不能直接通过 s[n] 得到，在 Rust 中而是应该通过
               " s.char().nth(n) "获取
            d. (类似数组) str 是 DST 类型，其对应的 &str 是"字符串切片类型"，&str 类型亦是
               一个胖指针(内部包含一个指向片段头部的指针和一个长度)
        6.2.2. String
            a. 与"&str"类型的主要区别是"String 有管理内存空间的权力"。"&str"类型是对一块字符串
               区间的借用,其对所指向的内存空间没有所有权即使" &mut str "亦如此([自]因 str 固定)
            b. String 类型在堆上动态申请了一块内存空间，其有权对这块内存空间进行扩容，其内部
               实现类似 std::Vec<u8> 类型(故可将此类型作为容纳字符串的容器使用)
    b. 引用 tao
        2.6.7 str 字符串类型
            Rust 提供了原始字符串类型 str,其通常以不可变借用形式存在即 &str (字符串切片)。出于
            内存安全考虑 Rust 将字符串分为两种类型：一种是固定长度(即不可更改其长度)字符串就是
            " &str "字符串；另一种是可增长(可改变其长度)字符串即是" String "字符串

11. 遍历字符串的方法(p192)
    a. 欲对每一个 Unicode 标量值都进行处理则最好的途径就是使用 chars 方法
        // 遍历方式1 ：chars 方法(可操作单独的 Unicode 标量值）
        for c in "नमस्ते".chars() {     // 返回 6 个类型为 char 的值
            println!("{}", c);
        }
    b. 欲返回每个原始字节则可使用 bytes 方法
        // 遍历方式2 ：bytes(返回每一个原始字节)
        for b in "नमस्ते".bytes() {
            print!("{}\t", b);
        }
    c. 标准库并未提供获取字形簇的方法
        // 从字符串中获取字形簇很复杂(标准库并未提供）
           但"crates.io"上有提供这样功能的 crate

12. 字节与字符串转换
     // 字符串转为字节
    for b in "静心道".bytes() {
        print!("{} \t", b)
        // console : " 233  157  153  229  191  131  233  129  147 "
    }
    // 字节到字符串
    let str = vec![233, 157, 153, 229, 191, 131, 233, 129, 147];
    let str = String::from_utf8_lossy(&str);
    println!("\n {} ",str)  // console : " 静心道  "

*/

/*
问号表达式：
    错误传播的模式在 Rust 编程非常常见，故 Rust 专门提供问号运算符(?)来简化

1. 传播错误的快捷方式: ? 运算符
    通过将 ? 放置于 Result 之后实现与使用 match 表达式来处理 Result 一样的
    功能。若 Result 的值为 OK 则包含 OK 的值会作为此表达式的值返回并继续执行
    程序；若值是 Err 则其会作为整个程序的结果提前返回(类比使用" return Err")

2.  ? 运算符与 match 区别
    a. 被 ? 运算符所接收的错误值会隐式地被 from 函数处理(这个函数定义于标准库
       的"From trait:用于错误类型之间进行转换")
    b. 当 ? 运算符调用 from 函数时，它就开始尝试将传入的错误类型转换为当前函数
       返回的错误类型。当一个函数拥有不同的失败原因，却使用了统一的错误返回类型
       来进行表达时，此功能会十分有用。只要每个错误类型都实现了转换为错误类型的
        from 函数，? 运算符会自动处理所有的转换过程)

3. ? 运算符只能被用于返回 Result 的函数
   因 ? 运算符类比 match 表达式(携带" return Err(e) "故返回 Result)，所以函数
   的返回类型亦必须是 Result 才能与 return 兼容

*/
fn main() {
    /* 编译错误：
       error[E0277]: the `?` operator can only be used in a function that returns
       `Result` or `Option` (or another type that implements `std::ops::Try`)
        / fn main() {
        |     let f = std::fs::read_to_string("file")?;
        |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot use the `?` operator
                                                   in a function that returns `()`
        | }
        |_- this function should return `Result` or `Option` to accept `?`
          = help: the trait `std::ops::Try` is not implemented for `()`
          = note: required by `std::ops::Try::from_error`
       // 错误提示信息指出：使用了 ? 运算符的函数必须返回" Result、Option 或任何实现
          了 std::ops::Try 的类型 "
    */
    let f = std::fs::read_to_string("file")?;
}

/*
for 循环
0. for 循环

1. 搜索" for 循环 "
    a.引用 dive
            13.3 内存不安全示例: 迭代器失效(p144~146[?])
                a. 在遍历一个数据结构的过程中修改这个数据结构，会导致迭代器失效
                b. 在 Rust 中 for 循环实质上是生成了一个迭代器，它一直持有一个指向容器的
                   引用，在迭代器的生命周期内，任何对容器的修改都无法编译通过
            3.3.3 for 循环
                a. Rust 中的 for 类比其它语言中的 for-each 循环( Golang 为" for range ")但其没有
                   三段式 for 循环语句(如 Golang 中的" for i:=0;i<len;i++ {} ")如
                       let arr = &[1, 2, 3];
                       for i in arr {
                           println!("The number is {}", i);
                       }
                   // for 循环的主要用途是利用迭代器对包含同类型的多元素容器执行遍历，如数组、链表、
                      HashMap、HashSet 等。在 Rust 中可轻松自定义定制容器和迭代器，因此也很容易使
                      for 循环支持自定义类型
                b. for 循环内部亦可使用 break、continue 控制执行流程
            24.2.3 for 循环(p285[*])
                a. Rust 中更简洁、更自然地使用迭代器的方式是使用 for 循环，本质上而言 for 循环就是专门
                   为迭代器设计的一种语法糖。
                b. for 循环可针对数组切片、字符串、Range、Vec、LinkedList、HashMap、BTreeMap 等所有
                   具有迭代器的类型执行循环，亦可允许针对自定义类型实现循环
                c. Rust 的" for <item> in <container> { <body> } "语法结构就是一个语法糖，其语法原理
                   就是调用 <container>.into_iter() 方法来获得迭代器，然后不断循环调用迭代器的 next()
                   方法将返回值解包赋值给 <item>,然后调用 <body> 语句块
                d. 使用 for 循环时可自主选择三种使用方式
                   // container 在循环后生命周期即结束
                   // (循环过程中每个 item 是从 container 中 move 出来)
                   for item in container{}
                   // 迭代器中只包含 container 的 & 型引用
                   // (循环过程中每个 item 都是 container 中元素的借用)
                   for item in &container{}
                   // 迭代器中包含 container 的 &mut 型引用
                   // (循环过程中每个 item 都是指向 container 中元素的可变借用)
                   for item in &mut container{}
                e. 只要某个类实现了 IntoIterator(扩展：From[源于] / Into[转为])那么调用 into_iter()
                   方法就可得到对应的迭代器，这个 into_iter() 方法的 receiver 是 self (而非 &self)故
                   执行的是 move 语义(此设计可同时支持 Item 类型为" T、&t、&mut T "可供用户选择)。
                f. 如 BTreeMap 容器类型，标准库对其 impl 了三次 IntoIterator，当 Self 类型为 BTreeMap
                   时 Item 类型为 (K,V)，意味着每次 next() 方法都是把内部元素 move 出来；当 Self 类型
                   为 &BTreeMap 时 Item 类型为 (&K,&V)，每次 next() 方法返回的是借用；当 Self 类型为
                   &mut BTreeMap 时 Item 类型为 (&K,&mut V)，每次 next() 方法返回的 key 是只读的而其
                   value 是可读写的。所以若有 BTreeMap 类型的变量 m 则可按需使用" m.into_iter() "或者
                   " (&m).into_iter() "或者" (&mut m).into_iter() "达到不同目的。
                        // trait 声明
                        trait IntoIterator {
                            type Item;
                            type IntoIter: IntoIterator<Item=Self::Item>;
                            fn into_iter(self)->Self::IntoIter;
                        }
                        // trait 实现
                        impl<K,V> IntoIterator for BTreeMap<K,V>{
                            type Item = (K,V);
                            type IntoIter = IntoIter<K,V>;
                        }
                        impl<'a, K:'a,V:'a> IntoIterator for &'a BTreeMap<K,V>{
                            type Item = (&'a K,&'a V);
                            type IntoIter = Iter<'a,K,V>;
                        }
                        impl<'a, K:'a,V:'a> IntoIterator for &'a mut BTreeMap<K,V>{
                            type Item = (&'a K,&'a mut V);
                            type IntoIter = Iter<'a,K,V>;
                        }
                g. Rust 的 IntoIterator trait 实际上就是 for 语法的扩展接口。欲自定义容器亦能在 for
                   循环中使用则可借鉴标准库的写法，自行实现 IntoIterator trait 即可
                   
    b. 引用 tao
            6.3.1 外部迭代器和内部迭代器(p194[*])
                a. 外部迭代器(External Iterator)：亦称主动迭代器(Active Iterator)独立于容器之外，通过
                   容器提供的方法(如"next"[即所谓的"游标"])来迭代下一个元素并需要考虑容器内可迭代的剩余
                   数量来进行迭代。外部迭代器的一个重要特点是"外部可以控制整个遍历进程"(如 Python、Java
                   及 C++ 语言中的迭代器就是外部迭代器)
                b. 内部迭代器(Internal Iterator)：内部迭代器则通过迭代器自身来控制迭代下一个元素，外部
                   无法干预，意味着只要调用了内部迭代器并通过闭包传入相关操作，就必须等待迭代器依次为其
                   内部的每个元素执行完相关操作后方可停止遍历(如 Ruby 语言中典型的内部迭代器 each )
                c. 早期的("1.0"版本之前) Rust 提供的是内部迭代器，而内部迭代器无法通过外部控制迭代进程
                   再加上 Rust 的所有权系统导致使用起来很复杂如
                       trait InIterator<T: Copy> {
                            fn each<F: Fn(T) -> T>(&mut self, f: F);
                        }
                        impl<T: Copy> InIterator<T> for Vec<T> {
                            fn each<F: Fn(T) -> T>(&mut self, f: F) {
                                let mut i = 0;
                                while i < self.len() {
                                    self[i] = f(self[i]);
                                    i += 1;
                                }
                            }
                        }
                        // 使用 Rust 提供的内部迭代器
                        fn main(){
                            let mut v = vec![1,2,3];
                            v.each(|i| i * 3);              // 调用内部迭代器(外部无法控制迭代进程)
                            assert_eq!([3, 6, 9], &v[..3]);
                        }
                d. 外部迭代器 : for 循环( Rust 中的 for 循环其实是一个语法糖)如
                        fn main() {
                            let v = vec![1, 2, 3, 4, 5];
                            for i in v {
                                println!("{}", i);
                            }
                        }
                        // for 循环的等价代码(简言之：for 循环是利用迭代器模式实现的一个语法糖)
                        fn main() {
                            let v = vec![1, 2, 3, 4, 5];
                            {  // 等价于for循环的scope
                                let mut _iterator = v.into_iter();
                                loop {
                                    match _iterator.next() {
                                        Some(i) => {
                                            println!("{}", i);
                                        }
                                       None => break,
                                   }
                               }
                           }
                        }
            6.3.5 消费器 Consumer (p207[*])
                a. Rust 中消费器都是惰性的(即不会自动发生遍历行为[除非调用 next 方法消费其中数据])
                b. 最直接消费迭代器数据的方法就是 for 循环(会隐式调用迭代器的 next 方法)
                c. 常见消费器
                   (1) any
                   (2) fold
                   (3) collect

    c. 引用 base
       使用 for 循环 ：增强代码安全性并消除可能由于下标越界而导致的 bug
       因其 for 循环安全性和简洁性( Rust 最常用）

*/

/*
字符串适用方法
    a. lines / contains ：逐行遍历 / 是否包含
        let contents = "\
                        Rust:
                        safe, fast, productive.
                        Pick three.";
        // 使用"lines"方法遍历字符串文本行返回一个迭代器
        for line in contents.lines() {
            // 用查询字符串搜索每一行(检查当前行是否包含[contains]查询字符串的功能)
            if line.contains("query") {   // 检查文本行是否包含 "query" 字符串

            }
        }

*/
