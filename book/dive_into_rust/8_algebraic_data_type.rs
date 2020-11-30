
// cardinality[kɑːdɪ'nælɪtɪ]n.基数
/*
0. 深入类型系统
   Rust 类型系统实际上是一种代数类型系统(Algebraic data type)

8.1 代数类型系统(p92~94[*])  // cardinality[kɑːdɪ'nælɪtɪ]n.基数
    a. 类型的"基数"(cardinality): 即一种类型所有取值的可能性。
       (0). 如最简单的类型 unit () 的基数就是 1，其可能的取值范围只能是 ();
       (1). 又如 bool 类型的基数为 2，可能的取值范围有两个(即 false 和 true);
       (2). 对于 i32 类型基数，其取值范围是 2^32，用 cardinality(i32) 代表
    b. 多个类型组合形成新的复合类型，则这个新的类型就会有新的基数

8.2 never Type (p94)
    a. Rust 中使用" ! "代表 never type (类比使用空 tuple () 代表 unit 类型)
    a. 发散函数(diverging function)、continue 返回 never type (即" ! "类型)

8.3 再谈 Option<T> 类型(p97): Option<T> 提供许多便于调用的成员函数
    a. map 方法可将一个 Option<U> 类型转为另外一个 Option<V> 类型如:
           fn main() {
               let maybe_some_string = Some(String::from("Hello, World!"));
               let maybe_some_len = maybe_some_string.map(|s|s.len());
               assert_eq!(maybe_some_len,Some(13));
           }
    b. unwrap 方法是从 Option<T> 中提取出 T，若当前状态为 None 则导致 panic!。
       故推荐使用 expect 替代 unwrap

*/