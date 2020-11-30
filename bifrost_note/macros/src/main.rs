
/*
0. macro 宏
    macro 宏是在语法扩展一般性机制上构建的，而弄清语法扩展则应先阐述编译器处理 Rust 源代码的机制

1. 编译器处理 Rust 源代码的机制(源码解析过程)
    a. Rust 程序编译过程的第一阶段是标记解析(tokenization)
    b. 编译过程的下一个阶段是语法解析(parsing)

2. 标记解析(tokenization)
    这一过程中，源代码将被转换成一系列的 token (即无法被分割的词法单元[类比单词)。Rust包含多种标记，比如
        标识符(identifiers)：foo, Bambous, self, we_can_dance, LaCaravane, …
        整数(integers)：42, 72u32, 0_______0, …
        关键词(keywords)：_, fn, self, match, yield, macro, …
        生命周期(lifetimes)：'a, 'b, 'a_rare_long_lifetime_name, …
        字符串(strings)："", "Leicester", r##"venezuelan beaver"##, …
        符号(symbols)：[, :, ::, ->, @, <-, …







 */


fn main() {
    println!("Hello, world!");
}
