/*
0. 通用集合类型
    a. 大部分数据结构都代表着某个特定的值但集合却可以包含多个值。与内置的数组
       和元组不同，这些集合将自己持有的数据存储在堆上。这意味着数据的大小无需
       在编译时确定，并且可随着程序的运行按需扩大或缩小占用的空间。不同的集合
       类型有着不同的性能特性与开销。
    b. Rust 程序中 3 个被广泛使用的集合:动态数组(vector)、字符串、映射(Map)
       (0). 动态数组(vector)：可连续存储任意多个值
       (1). 字符串(string)是字符的集合
       (2). 哈希映射(hash map)：映射(map)的特殊实现

1. 创建动态数组(p176[*])
    a. 使用 Vec::new() 如
       let v:Vec<i32> = Vec::new(); // 从未添加首元素则需要指定类型
    b. 使用 vec! 宏，可根据提供的值创建新的动态数组
       let v = vec![1,2,3]; // 能从初始化值推断出类型故可不指定
    c. 使用迭代器
       let vec:Vec<i32> = (0..5).collect(); // 须指定类型

2. 读取动态数组中的元素
   a. 两种方式：使用" array[index] "或" array.get(index)"
   b. 使用索引 index (索引从"0"开始)，使用" & "与" [] "会直接返回元素的引用
      而接收索引 index 作为参数的 get 则会返回 Option<&T>
   c. " [idx] "会因为索引指向不存在的元素而导致程序触发 panic!，适用于预期尝试
      越界访问元素时使程序直接崩溃场景
   d. " get(idx) "方法会在检测到索引越界时简单地返回 None 而不是使程序直接崩溃
      适用于允许偶尔越界访问动态数组中的元素时是一个正常行为的场景(如用户误输入)

使用枚举来存储多个类型的值(p181[*])
   在动态数组中需要存储不同类型的元素时，可以定义并使用枚举来应对这种情况，因为
   枚举中的变体都被定义为同一种枚举类型

*/
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main_0() {
    // 若编程时未知 vector 类型则枚举行不通(可用 trait 对象)
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

// 动态数组遍历、读取、更新
fn main_1() {
    // 获取所有权：整型
    let mut vec = vec![1, 2, 3];
    /* [自]解析：因元素类型已实现" Copy trait "
                 故可获取所有权遍历[?]   // 待查证
       for 循环的本质
    */
    for i in vec {
        print!("{},", i);   // Console:" 1,2,3 "
    }
    /* 编译报错:
       error[E0382]: borrow of moved value: `vec`
           let mut vec = vec![1,2,3];
               ------- move occurs because `vec` has type `std::vec::Vec<i32>`,
                                    which does not implement the `Copy` trait
           for i in vec{
                    ---
                    |
                    value moved here
                    help:consider borrowing to avoid moving into the for loop:`&vec`
           println!("{:?}",vec);
                           ^^^ value borrowed here after move

    */
    // println!("{:?}",vec);
    // 获取所有权：字符串
    let vec = vec!["c".to_string(), "h".to_string(), "w".to_string()];
    for i in vec {
        print!("{} ", i);    // Console:" c h w "
    }
    // println!("{:?}",vec);    // 同理不可用

    // 使用索引" [index] "：其前是否有" & "的区别
    let v = vec![1, 2, 3];// 整型使用索引 ：" [index] "
    /* [自]解析：因元素类型已实现" Copy trait "
                故可获取所有权[?]  // 待查证
    */
    let m = v[1];
    //let m = &v[1]; // 是否有" & "皆可
    println!("{:?}", v);
    // 字符串使用索引 ：" [index] "
    let v = vec!["c".to_string(), "h".to_string(), "w".to_string()];
    /*编译报错：
       error[E0507]: cannot move out of index of `std::vec::Vec<std::string::String>`
           let m = v[1];
                   ^^^^
                   |
                   move occurs because value has type `std::string::String`, which
                                                does not implement the `Copy` trait
                   help: consider borrowing here: `&v[1]`
       //
    */
    //let m = v[1];   // 飘红报错: Cannot move
    let m = &v[1];// 必须使用" &[] "
    println!("{:?}", v);    // Console:" ["c", "h", "w"] "

    // for 遍历动态数组的引用
    let v = vec![1, 2, 3];
    // [自] i 是否可变未能影响 vec
    for mut i in &v {
        println!("{}", i);
        i = &6;
        print!("{} ", i);   // Console:" 6  6  6 "
    }
    println!("{:?}",v); // Console:" [1, 2, 3] "

    // 可变引用
    let mut vec = vec![1, 2, 3];
    for i in &mut vec{
        *i += 3;
    }
    println!("{:?}",vec); // Console:" [4, 5, 6] "

    /* // 无此语法
        for i in mut vec{
            println!("{}",i);
        }
    */

}

/*
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
   a. String::from("string_literal") 等同于 "string_literal".to_string()"
   b. let mut s = String::new(); / s.push('char'); / s.push_str(&str)
      push 方法接收"单个字符"作为参数 / push_str 接收"字符串切片"作为参数
   c. 强制解引用:编译器可自动将 &String 解引用为 &str (如" &s -> &s[..] ")

5. 字符串索引(p188[*])
    在许多编程语言中，往往可合法地通过索引来引用字符串中每一个单独的字符但
    在 Rust 语言中禁止通过索引来获得 String 中的字符(编译直接 panic! )

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

8. 字符串切片(p191[*])
   尝试通过索引引用字符串通常是一个坏主意，因为字符串索引操作返回的类型不确定：
   究竟应该是字节、字符还是字形簇、甚至是字符串切片呢，因此若真想要使用索引来
   创建字符串切片则 Rust 会要求做出更加明确的标记。即为明确表明需要一个字符串
   切片则需要在索引的" [] "中填写范围即" [start_idx..end_idx] " (而非单数字
   索引即" [ idx ] ")来指定所需的字节内容。切忌务必小心谨慎地使用范围语法创建
   字符串切片，因错误指令会导致程序崩溃(如索引非有效字符边界)。
   错误示例： let hello = "Здравствуйте";
             let s = &hello[0..1];     // 索引范围非有效字符边界则"panic!"

9. 遍历字符串的方法(p192)
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

10. 字符串的确没那么简单
    a. 总而言之字符串确实是挺复杂的，Rust 选择了将正确的 String 数据处理方法
       作为所有 Rust 程序的默认行为，这也意味着程序员需要提前理解 UTF-8 数据
       的处理流程
    b. 与某些编程语言相比，Rust 这一设计暴露了字符串中更多的复杂性，但却避免了
       开发周期邻近时再去处理那些涉及非 ASCII 字符的错误

11. 在哈希映射中存储键值对(p193[*])
    哈希映射在内部实现中使用了"哈希函数"，这同时决定它在内存中存储键值对的方式

12. 创建一个新的哈希映射(p194[*])
    a. 首先需要使用 use 将 HashMap 从标准库的集合部分引入当前作用域。由于哈希
       映射的使用频率比动态数组(vector)及字符串(string)低一些
    b. 可使用 new 来创建一个空哈希映射，并通过 insert 方法来添加元素
    c. 和动态数组一样，哈希映射也是将数据存储于堆上、亦是同质的(所有键类型必须
       相同，所有值类型亦必须相同)
    d. 另外构建哈希映射的方法是"在一个由键值对组成的元组动态数组上使用 collect
       方法 "(collect 方法可将数据收集到很多数据结构中[包括 HashMap ])。如在
       不同的动态数组(如分别存储队伍名字及分数)可用 zip 方法创建一个元组数组后
       使用 collect 方法来将动态数组转换为哈希映射。

*/
fn main() {
    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    /* a. 不同动态数组可用 zip 方法创建一个元组数组后使用 collect 方法将动态数组
          转换为哈希映射
       b. 此处类型标记" HashMap<_, _> "不能被省略，因为 collect 可作用于许多不同
          的数据结构，若未指明类型则 Rust 无法知晓预期的目标类型，但对于键值对类型
          参数可根据动态数组中的数据类型推导故可用下划线占位
    */
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

}

/*
13. 哈希映射与所有权
    对于实现"Copy trait"的类型(如"i32") : 其值会被简单地复制到哈希映射中
    而对于 String 这种持有所有权的类型 : 其值及所有权将被移动至哈希映射
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");
        let mut map = HashMap::new();
        // field_name 和 field_value 变量被移动到哈希映射中
        map.insert(field_name, field_value);
        // 至此 : field_name 和 field_value 不再有效(已移动)
        /* 若将值引用插入 HashMap 其值本身将不会被移动进 HashMap
           但引用指向的值必须至少在 HashMap 有效时也必须是有效的
           (参见第 10 章"使用生命周期保证引用的有效性"章节)
        */
        map.insert(&field_name, &field_value);

14. 访问哈希映射中的值
    a. 通过将"键的引用"传入 get 来获取哈希映射中的值。get 源码如下：
          pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
          where
              K: Borrow<Q>,
              Q: Hash + Eq,
          {
              self.base.get(k)
          }
          //  get 返回 Option<&V> 其结果被装进 Some；若 key 在映射
              中无对应值则 get 返回 None
    b. 可用 for 遍历哈希映射中所有的键值对
           // 遍历 HashMap : 获取所有权(遍历后 scores 不可再用)
            for (key, value) in scores {
                print!("{}: {}\t", key, value);
            }
            // println!("{:?}",score); // 遍历后 scores 不可再用

            // 遍历 HashMap : 遍历后 scores 仍可再用(因从未获取所有权)
            for (key, value) in &scores {
                print!("{}: {}\t", key, value);
            }
            println!("{:?}",score); // 遍历后 scores 仍可再用

15. 更新哈希映射
    a. 覆盖旧值: insert(key,value)
           let mut scores = HashMap::new();
           scores.insert(String::from("Blue"),10);
           scores.insert(String::from("Blue"),25); // 替换特定键存储的旧值
           println!("{:?}",scores);
    b. 保留旧值：entry(key).or_insert(value)
       (0). "entry"接收键作为参数并返回 Entry 枚举(指明键所对应的值是否存在)
       (1). Entry 的 or_insert 方法被定义为返回一个 Entry 键所指向值的"可变
            引用"(若键指向的值不存在则将参数作为键的新值插入哈希映射中并返回
            新值的"可变引用")，其" entry "方法源码如下：
                pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
                    map_entry(self.base.rustc_entry(key))
                }
        (2). 示例
               let mut scores = HashMap::new();
               scores.insert(String::from("Blue"), 10);
               // 键未关联值故执行 insert 操作
               scores.entry(String::from("Yellow")).or_insert(50);
               // 键关联值故无任何操作
               scores.entry(String::from("Blue")).or_insert(50);
               println!("{:?}",scores);
    c. 基于旧值更新
       哈希映射的另外一个常见用法是查找某个键所对应的值并基于这个值来进行更新

*/
fn main_2() {
    // 根据旧值更新
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    /* 统计文本中每个单词出现次数
        使用以单词作为键的哈希映射来记录出现次数
    */
    for word in text.split_whitespace() {
        /*  or_insert 方法为传入的键返回指向关联值的可变引用(&mut V)，此可变
            引用进而被储存到变量 count 上，为了对这个值进行赋值操作，必须先用
            " * "来对 count 解引用(*count)，由于这个可变引用在 for 循环结尾
            离开作用域，故在代码中的所有修改都是安全的并符合借用规则
        */
        // 首次出现的单词插入值 0
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map); //console : {"hello":1,"world":2,"wonderful":1}
}

/*
16.哈希函数 :
   为了提供抵抗拒绝服务（Denial of Service, DoS）攻击的能力，HashMap 默认使用
   了一个在密码学上安全的（cryptographically strong）哈希函数。但其并非最快的
   哈希算法(不过为更高的安全性付出一些性能代价通常是值得的)。如若性能监测显示此
   哈希函数成为性能热点并导致性能受损，则可指定不同的哈希计算工具(hasher)来切换
   为其它函数(这里哈希计算工具[hasher]特指实现了 BuildHasher trait 的类型[可于
   " crate.io "搜寻不同的哈希算法)

*/
