
// hack[hæk]v&n.击打,([非法]入侵)骇客
// algebraic[ˌældʒɪˈbreɪk]adj.代数的
/*
0. 变量和类型

2. 变量声明
   模式解构(pattern-destructure): " let (mut a, mut b) = (1,2); "

2.1.4 静态变量(p15[*])
    a. Rust 中可用 static 关键字声明静态变量如" static GLOBAL:i32 = 0; "
       // 用 static 声明的变量生命周期为整个程序(从启动到退出)，static 变量
          的生命周期永远是 'static 其占用的内存空间不会在执行过程中回收。(
          这亦是 Rust 中唯一声明全局变量的方法)
    b. 示例(p15[*])

2.1.5 常量
    常量、静态变量、变量的声明及使用区别(p16[*])

2.2.2 char 类型(p17[*])
    Rust 中 char 类型设计目的是描述任意一个 unicode 字符，因此其占用 4 个
    字节(而非 1 个字节)。可用单字母 b 在"字符或字符串"前面代表此字面量存储于
    u8 类型数组中，这样占用空间比 char 型数组要小一些。如
    " let s:&[u8;5] = b"hello"; "

2.2.3 整数类型
    在所有数字字面量中，可在任意地方添加任意的下划线以方便阅读。如:
    " println!(" 9 power 3 = {} ", 9_i32.pow(3));

2.2.4 整数溢出(p20[*])
    a. Rust 编译器提供独立的编译开关设置处理溢出的策略
       " rustc -C overflow-check=no file.rs "
    b. 多数情况下,整数流出应该被处理为截断(即丢弃最高位)

2.2.6 指针类型(p23[*])
    常见的几种指针类型(表2-2、表2-3)

2.2.7 类型转换
    as 表达式允许的类型转换如"表2-4"(p24[*])

2.3 复合数据类型

2.3.1 tuple
    tuple 类型大小

2.3.2 struct

2.3.3 tuple struct
    a. tuple、struct、tuple struct 区别：如表2-5 (p27[*])
    b. tuple struct 典型使用场景 ： newtype

2.3.4 enum
    a. 相较于"tuple、struct、tuple struct"(代表的是多个类型间"与"关系)而言
       则 enum 类型在 Rust 中代表的是多个类型间的或关系
    b. Rust 亦支持 union 类型，与 C 语言中的 union 完全一致，但在 Rust 里面
       读取它内部的值被认为是 unsafe 行为，一般不会使用此种类型，其存在的目的
       是为了方便与 C 进行交互
    c. Rust 的 enum 实际上是一种代数类型系统(Algebraic Data Type,ADT)

*/