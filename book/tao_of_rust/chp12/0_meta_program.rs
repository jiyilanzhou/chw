
// hygiene[ˈhaɪdʒiːn]n.卫生 // hygienic[haɪˈdʒiːnɪk]adj.卫生的
// token[ˈtəʊkən]n.标记,令牌,词条
// TokenTree 词条树        // TokenStream 词条流
/*
0. 元编程
   a. 元编程源于" Meta-Programming "一词。" Meta "表示" 关于某事本身的某事 "
      (如 Meta-Knowledge 表示"关于知识本身的知识"即"元知识"；Meta-Cognition
      表示" 关于认知本身的认知 "即"元认识"；故 Meta-Programming 表示" 关于编程
      本身的编程 "即" 元编程 ")。" 元 "即" 本源、开端 "之意
   b. 元编程技术大致可分为以下几类
       (0). 简单文件替换
       (1). 类型模板
       (2). 反射：如 Ruby,Java,Go 及 Rust 等或多或少都支持反射、
                 (在运行时或编译时获取程序的内部信息)
       (3). 语法扩展：如 Rust 的 drive 属性
       (4). 代码自动生成：如 Go 提供 go generate 命令来根据指定注释自动生成代码
       // Rust 通过 反射 和 AST语法扩展 来支持元编程

12.1 反射(p436[*])
    Rust 提供 std::any::Any 来支持运行时反射

12.1.1 通过 is 函数判断类型

12.1.2 转换到具体类型
    downcast_ref / downcast_mau : 向下转型

12.1.3 非静态生命周期类型

12.2 宏系统
    Rust 还提供功能强大的 宏(Macro)来支持 元编程。宏是一种批处理称谓，通常来说是
    根据预定义的规则转换成相应的输出，这种转换过程叫作宏展开(Macro Expansion)

12.2.1 起源
    宏操作大致可分为两类: 文本替换 及 语法扩展
     a. C 语言中的宏属于"文本替换"
     b. Lisp 语言中的宏属于"语法扩展"。Lisp 宏可利用 S 表达式(S-Expr)将代码作为
        数据生成新的代码，而这些代码又可以被执行，这就赋予了 Lisp 宏强大的可能性
        包括可由此进行语法扩展甚至创建新的语法。简言之 Lisp 宏就是将一个 S 表达式
        转变为另一个 S 表达式
     c. 宏调用与函数调用区别
        宏调用产生的是 S 表达式而函数调用会产生具体的值

12.2.2 Rust 中宏的种类
    a. Rust 宏系统按定义可分为两大类
       声明宏(Declaration Macro)
       过程宏(Procedural Macro)
    b. 具体到宏使用的语法又可分为以下两种
       调用宏：如 println!、assert_eq!、thread_local! 等
       属性宏：如 #[derive(Debug)]、#[cfg] 等
    c. 按宏的来源可分为以下两类
       内置宏
       自定义宏

12.2.3 编译过程
    词条流
    抽象语法树

12.2.4 声明宏(p445[*])
    a. 声明宏的定义和展开过程
       (0). unless! 宏定义示例
       (1). 通用解析器(Normal Parser)、宏解析器(Macro Parser)
    b. 声明宏工作机制
    c. 声明宏的实现技巧
        hashmap! 宏用法示例
    d. 调试宏
       调试 hashmap! 宏
    e. 声明宏的卫生性
       // hygiene[ˈhaɪdʒiːn]n.卫生 // hygienic[haɪˈdʒiːnɪk]adj.卫生的
       声明宏在展开后不会污染原来的词法作用域，具有这种特性的宏叫卫生宏
       (Hygienic Macro)。Rust 的声明宏具有部分卫生性
    f. 导入/导出
        #[macro_use] / #[macro_export]

12.2.5 过程宏(p458[*])
    a. 过程宏定义于内置的 libproc_macro 包中，其建立在词条流(TokenStream)
       基础上(优点:未来无论语法如何变化都不会影响到过程宏的使用)
    b. 目前使用过程宏可实现以下三种类型宏:
       (0). 自定义派生属性
       (1). 自定义属性
       (2). Bang 宏 ：以 Bang 符号(即是叹号" ! ")结尾的宏
    c. 自定义派生属性
    d. 自定义属性
    e. 编写 Bang 宏

12.3 编译器插件
    Rust 中最强大的元编程工具非编译器插件(目前还没有稳定)莫属，编译器插件由
    内置的 librustc_plugin 包提供

*/