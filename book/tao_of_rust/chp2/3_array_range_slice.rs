
/*
2.6.4 数组类型
    a. 数组(Array)是 Rust 内建的原始集合类型，其有如下特性:
       (1). 数组大小固定
       (2). 元素均为同类型
       (3). 默认不可变
    b. 数组的类型签名为" [T; N] "，T 是泛型标记代表数组元素的具体类型；N 代表数组长度(
       是编译时常量[必须于编译时确定其值])

*/
#![feature(never_type)]
fn main_003_000() {
    /* 定义类型" [i32; 3] "的数组：
          a. 数组长度固定(不允许对其增删元素)，即使通过 let mut 关键字定义可变
             绑定 mut_arr 也只能修改索引范围内元素
          b. 另外可通过" [initVal; len] "语法创建初始值为"initVal"长度为"len"
             的数组(对于越界访问则编译报错[有效阻止内存不完全的操作])
          c. 对于原始固定长度数组，只有实现 Copy trait 的类型才能作为其元素。即
             只有可存放于栈上的元素才可置于该类型的数组中([自]数组仅能存放最基础
             的数据类型[如数字类型、布尔类型竺]：类比其它语言数组元素类型)
          d. (展望)将来 Rust 可能还支持 VLA(variable-length array) 即可变长度
             数组(基于栈上动态分配的内存函数来实现)
    */
    let arr: [i32; 3] = [1, 2, 3];  // 定义一个[i32; 3]类型的数组(默认不可变)
    // println!("{:?}", arr[5]); // 编译错误:"index out of bound"(索引越界)
    // 数组长度固定(不允许对其增删元素)即使通过 let mut 绑定亦只能操作索引内元素
    let mut mut_arr = [1, 2, 3];
    assert_eq!(1, mut_arr[0]); // 数组索引从0开始，验证第一位元素等于1
    mut_arr[0] = 3; // 修改 mut_arr 第一个元素为 3，因为它是可变数组
    assert_eq!(3, mut_arr[0]);  // 验证修改之后的mut_arr数组第一个元素为3
    // 通过" [initVal; len] "语法创建初始值为"initVal"长度为"len"的数组
    let init_arr = [0; 10]; // 创建一个初始值为0，长度为10的数组
    assert_eq!(0, init_arr[5]);// 通过索引验证 init_arr[5] 数组元素值是否为 0
    assert_eq!(10, init_arr.len()); // 验证数组的长度是否为10
}

/*
2.6.5 范围类型      // operator[ˈɒpəreɪtə(r)s]n.算子,操作符
    Rust 内置了范围(Range)类型，包括"左闭右开"和"全闭"规程区间
    a. " .. / ..= "分别为" std::ops::Range / std::ops::RangeInclusive"实例，分别
       表示"左闭右开区间、全闭"区间
    b. 范围类型绑定的部分方法(如" sum "可为范围内元素求和且每个范围都是一个迭代器,可
       直接使用 for 循环取出

*/
fn main_003_001() {
    // 标准库" std::ops::Range "
    assert_eq!((1..5), std::ops::Range { start: 1, end: 5 });
    // 标准库" std::ops::RangeInclusive "
    assert_eq!((1..=5), std::ops::RangeInclusive::new(1, 5));
    assert_eq!(3 + 4 + 5, (3..6).sum());
    // 左闭右开
    for i in 1..3 {
        print!("{}\t", i);  // Console:" 1 2 "
    }
    // 全闭
    for i in (1..=3) {  // "(1..=3) "括号可省略(因无歧义)
        print!("{}\t", i);   // Console:" 1 2 3 "
    }
}

/*
2.6.6 切片类型
    切片(Slice)类型是对一个数组(包括固定大小数组和动态数组)的引用片段，有利于安全有效
    地访问数组的一部分而无需拷贝(因理论上而言切片引用的是已存在的变量[在底层切片代表
    一个携带数组长度指向数组起始位置的指针])。若用 [T] 类型表示连续序列则切片类型就是
    " &[T] 或 &mut [T] "

*/
fn main_003_002() {
    // 数组声明绑定
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    /* 通过操作符 & 对数组进行引用就产生一个切片 &arr，亦可结合范围对数组进行切割比如
       " &arr[1..] "表示获取 arr 数组索引 1 后的所有元素
    */
    assert_eq!(&arr, &[1, 2, 3, 4, 5]);
    assert_eq!(&arr[1..], [2, 3, 4, 5]);
    /* 切片提供两个 const fn 方法: len (获取切片长度)和 is_empty (判断切片是否为空)
       运算符优先级：因"."优先级高于"&"故先运算"&"须使用"()"
    */
    assert_eq!((&arr).len(), 5);  // 先运算"&"须使用"()"：因"."优先级高于"&"
    assert_eq!((&arr).is_empty(), false);
    // 可通过 &mut 定义可变切片(能直接通过索引操作相应元素)
    let arr = &mut [1, 2, 3];
    arr[1] = 7;
    assert_eq!(arr, &[1, 7, 3]);
    // vec! 宏定义的动态数组亦可通过操作符 & 来得到一个切片
    let vec = vec![1,2,3];
    assert_eq!(&vec[..],[1,2,3]);
}

/*
2.6.7 str 字符串类型
    Rust 提供了原始字符串类型 str,其通常以不可变借用形式存在即 &str (字符串切片)。出于
    内存安全考虑 Rust 将字符串分为两种类型：一种是固定长度(即不可更改其长度)字符串就是
    " &str "字符串；另一种是可增长(可改变其长度)字符串即是" String "字符串

*/
fn main_003_003() {
    /* 定义字符串字面量 truth :
        a. 本质上字符串字面量亦属于 &str 类型(本例 truth 是静态生命周期[可理解为该字符串
           持续有效]字符串 &'static str )
        b. " &str "由两部分组成：指向字符串序列的指针及记录的长度值(可分别通过 as_prt 及
           " len "方法获取)
        c. Rust 中的字符串本质上是一段有效的 UTF-8 字节序列故可将一段字节序列转换为 &str
           字符串。
    */
    let truth: &'static str = " Rust 是一门优雅的语言";
    let ptr = truth.as_ptr();
    println!("{:?}", ptr);   // Console:" 0x473120 "
    let len = truth.len();
    assert_eq!(len, 30);
    /*通过调用" std::slice::from_raw_parts "函数传入"指针及长度"，可将相应的字节序列
      转换切片类型" &[u8] "，然后再使用" std::str::from_utf8 "函数将得到的切片转换为
      " &str "字符串。因整个过程并未验证字节序列是否为合法的 UTF-8 字符串所以需要置于
      "unsafe"块中执行转换过程("unsafe"块意味着 Rust 编译器将内存安全交由开发者负责)
    */
    // Interview : 如何将相应的字节序列转换为" &str "字符串?
    let s = unsafe {
        let slice = std::slice::from_raw_parts(ptr, len);
        // 注：末尾是否携带分号用于区分表达式(有返回值)与语句(同时决定"块表达式"返回值)
        std::str::from_utf8(slice)
    };
    assert_eq!(s, Ok(truth));
}

/*
2.6.8 原生指针  // raw[rɔː]adj&n.原始(的)，未处理/未加工(的)
    a. 将可表示内存地址的类型称为指针。Rust 提供很多类型的指针包括"引用(Reference)、原生
       指针(Raw Pointer)、函数指针(fn Pointer)和智能指针(Smart Pointer)"
    b. 引用(Reference)本质是一种非空指针。Rust 可划分为 Safe Rust 及 Unsafe Rust 两部分
       引用主要应用于 Safe Rust 中(其编译器会对引用进行借用检查以保证内存安全及类型安全)；
       原生指针主要用于 Unsafe Rust 中。直接使用原生指针是不安全的，比如原生指针可能指向
       一个 Null 或者已被释放的内存区域，因原生指针使用不在 Safe Rust 可控范围内所以需要
       开发者自己保证安全。Rust 支持两种原生指针：不可变原生指针" *const T "及 可变原生
       指针" *mut T "
    Interview : 引用与原生指针应用场景

*/
// primitive[ˈprɪmətɪv]adj&n.原始(的)，基本/简单(的)
// primary[ˈpraɪməri]adj&n.主要(的),基本(的)
fn main_003_004() {
    // 声明绑定可变变量(获取可变变量前须先声明为 mut )
    let mut x = 10;
    // 通过 as 操作符将 &mut x 可变引用转换为 *mut i32 可变原生指针 ptr_x
    let ptr_x = &mut x as *mut i32;
    // 使用" Box::new "在堆内存中存储数字
    let y = Box::new(20);
    /* a. [自]"y"指向堆空间地址其解引用"*y"为获取堆空间数据故" &(*y) "为获取存储数据
          的堆空间地址。但不能直接使用变量 y (其虽指向堆空间数据地址[即" &(*y) "语义
          类比" y "]然非原始类型故需" &(*y) ")转换为原生指针
       b. 使用"  let ptr_y = y as *const i32; "编译错误：
          error[E0605]:non-primitive cast:`std::boxed::Box<i32>` as `*const i32`
            let ptr_y = y as *const i32;
                        ^^^^^^^^^^^^^^^
          = note:an `as` expression can only be used to convert between primitive
                                           types. Consider using the `From` trait

    */
    //let ptr_y = y as *const i32;
    let ptr_y = &*y as *const i32;// 转成不可变原生指针 *const T
    // 操作原生指针需使用 unsafe 块
    unsafe{
        *ptr_x += *ptr_y;   // 分别解引用求和
    }
    assert_eq!(30,x);
}

/*
2.6.9 never 类型
    Rust 提供一种特殊的数据类型 never (即" ! ")：该类型用于表示永远不可能有返回值的
    计算类型。比如线程退出时就不可能有返回值。Rust 是一个类型安全的语言故需将这种情况
    纳入类型系统进行统一管理

*/
/* " #![feature(never_type)] "特性：
     never 类型属于实验特性故须在 Nightly 版本下方可显示使用
*/
// #![feature(never_type)]
fn foo() -> i32 {
    /* 定义 x 绑定 never 类型：
          因 return 结束 foo() 函数并将 123 返回，故绑定的 x 永远都不会被赋值。所以
          此处使用 never 类型不会出现编译错误(与" return "表达式类似的有" break 及
          continue "关键字)
    */
    let x: ! = {
        //  [自] return 即将其后携带的数值返回(故数据值后分号有无皆可[因根本不会执行到])
        return 123;
    };
}

fn main() {
    let num: Option<u32> = Some(42);
    /* match 匹配表达式(要求:所有分支都必须返回相同的类型)
         a. None 分支使用 panic! 宏(其 panic! 宏会返回 never 类型即" ! ")
         b. match 分支必须返回相同数据类型，其 Some(num) 分支返回 u32 类型而 None 分支
            返回 never 类型，但编译却未报错(因 never 类型可强制转换为其它任何类型)
    */
    match num {
        Some(num) => num,
        None => panic!("nothing"),
    };
}