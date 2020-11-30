
/*
1、字符串 slice 是 String 部分值的引用
2、字面值即是 slice
3、其它类型 slice ： 如数组 slice
*/

fn main() {
    let s = String::from("hello world");
    // String 部分值引用
    let h = &s[0..5];
    let h = &s[0..=4];
    let h = &s[..=4];
    let h = &s[..5];
    println!("h = {}", h);

    let w = &s[6..11];
    let w = &s[6..=10];
    let w = &s[6..];
    let w = &s[..];
    println!("w = {}", w);

    /* 字符串 slice 索引：
           须位于有效 UTF-8 字符边界，若尝试从中截取则报错
       编译报错：
        thread 'main' panicked at 'byte index 1 is not a char boundary;
                            it is inside '静' (bytes 0..3) of `静心道`'
    */
    /* 暂且注释：以通过编译
    let zh = String::from("静心道");
    let cn = &zh[0..1];
    println!("{}",cn);
    */

    // 字面值
    let literal = "chw"; // &str
    println!("{}",literal);
    let valueSegment = &literal[..1];
    println!("{}",valueSegment);        // Console:" c "

    // 数组 slice
    let sli = ['a', 'b', 'c', 'd'];
    let s = &sli[1..=3];
    println!("s0 = {}", s[0]);        // Console" b "
    println!("s1 = {}", s[1]);       // Console:" c "
    println!("len = {}", s.len());    // Console:" 3 "

}

/*
 0. Rust 字符串类型 &str 及 String :
    a. 前者是字符串的引用，后者是基于堆创建可增长的字符串
    b. " &str "常见形式是字符串字面量(如 let s = "chw"; 则 s 的类型即是字符串字面量 literal (程序
       编译成二进制后此字符串会被保存在文件内部故 s 是特定位置字符串的引用即 s 是 &str 类型)
    c. " &str "由于保存在二进制文件内故 &str 类型不存在生命周期的概念，其在整个程序生命周期 'static
       内都能访问
    d. " String "是最常用的字符串类型，其本质为" vector "

 1. String 与 &str (Rust官方解释):
    a. String 是一个堆上分配的可变 UTF-8 字节缓冲区，而 &str 是一个不可变的固定长度字符串。若切片
       是从 String 解引用而来则通常指向堆上;若是字符串字面值则指向静态内存
    b. &str 是一个由 Rust 语言实现的原生类型而 String 则是由标准库实现的
    c. 对于 Rust 而言字符串是 Unicode 标量值序列编码为 utf-8 的字节流。所有字符串都必须保证为有效
       的 utf-8 编码序列

*/