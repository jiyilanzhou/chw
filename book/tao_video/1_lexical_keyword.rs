
/*
0.  Rust 词法结构(包含六大部分)
    • 关键字（Keywords）
    • 标识符（Identifier）
    • 注释（Comment）
    • 空白（Whitespace）
    • 词条（Tokens）
    • 路径（Path）

1. 关键字
    • 严格关键字（Strict）：专用于特定上下文的关键字
            as/ break/ const/ continue/ crate/ if/ else/ struct/ enum/ true/ false/ fn/ for/ in/ let/
            loop/ impl/ mod/ match/ move/ mut /pub/ ref/ return/ self/ Self/ static/ super/ trait/
            type/ unsafe/ use/ where/ while /async/ await/ dyn / main
    • 保留字（Reserved）
            abstract/ become/ box/ do/ final/ macro/ override/ priv/ typeof/
            unsized/ virtual/ yield / try
    • 弱关键字（Weak）：只有在特定上下文中才有特殊意义的关键字
            2018 Edition: union(仅在其用于声明 union 联合体时才被看作关键字), ‘static
                ( union 仅在其用于声明 union 联合体时才被看作关键字,其它场景任意使用)
                ( 'static 生命周期：其不能当作普通的生命周期被使用，只能被用作静态生命周期)
            2015 Edition: dyn
                ( dyn 在 2015 版本中被当作弱关键字，到 2018 版本中被提升为了严格关键字)
            被保留的关键字不代表将来一定会使用


2. 标识符
    Rust 目前仅支持 ASCII 码的字符串作为标识符(目前正在支持非 ASCII 码字符串作为标识符：源于特定领域使用
    术语编写代码[可简化实现和讨论]，而无需从项目需求中转换为英文词汇[如" emoji "表情：非 ASCII 的字符串])

3. 注释、空白与路径
    a. 模块级、行级、块级 文档注释
    b. Rust 中空白字符包括：\n、\t、tab 等(任何形式的空白字符在 Rust 中只用于分隔标记，没有语义意义)
    c. 路径是由"名称空间限定符即双冒号 :: "在逻辑上分隔的一个或多个路径段组成的序列

4. 词条(Rust 源码在编译期会形成诸多类型的词条：未来亦可能加入新的词条类型)
    a. 语言项 （item）: 即 Rust 中的基本语法要素
                       (包括模块、函数、类型别名、trait、结构体、枚举体、宏、静态项、常量项等等，除了"宏"
                        之外其它"语言项"皆可设置可见性)
    b. 块（block）
    c. 语句（Stmt）
    d. 表达式 （Expr）
    e. 模式 （Pattern）
    f. 关键字 （Keyword）
    g. 标识符 （Ident）
    h. 字面量 （Literal）
    i. 生命周期 （Lifetime）
    j. 可见性 （Vis）
    k. 标点符号（Punctuation）
    l. 分隔符（delimiter）
    m. 词条树（Token Tree）
    n. 属性（Attribute）
    // 词条在编写"宏"的时候非常有用


 */