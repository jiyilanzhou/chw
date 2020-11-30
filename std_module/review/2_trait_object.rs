
/*
0.  Result<(), Box<dyn Error>> 与  Result<(),E> 区别
    a. Result<(), Box<dyn Error>> 返回一个实现了 Error trait 的类型但无需
       指定具体类型，意味着可在不同错误场景下返回不同的错误类型。语句中 dyn
       关键字所表达的正是这种"动态(dynamic)"的含义
    b. Result<(),E> 为普通 Result<T,E> 类型
    // 具体区别及应用场景待查阅[?]

1. DerefMut<Target=Self> 的用法[?]

2. trait object 与 trait bound 区别
    a. trait
           pub trait Draw {}
    b. trait 对象
           pub struct Screen {
               pub components: Vec<Box<dyn Draw>>,
           }
    c. trait 约束
           pub struct Screen<T:Draw> {
               pub components: Vec<Box<T>>,
           }
    // trait 对象不同于带有 trait bound 泛型类型参数的结构体 ：trait 对象
       允许在运行时替代多种具体类型而泛型类型参数一次只能替代一个具体类型
    // 对泛型使用 trait bound 时编译器执行的单态化过程 ：编译器为每一个被泛型类型参数代替的
       具体类型生成了非泛型的函数和方法实现。单态化所产生的代码进行静态分发(static dispatch:
       即编译器知道在编译时调用的方法),而动态分发（dynamic dispatch）编译器在编译时无法知晓
       调用的方法(即编译器在运行时才能确定调用的方法)
    // 使用 trait 对象时 Rust 必须使用动态分发: 因编译器无法知晓所有可能用于 trait 对象代码
       的类型故未知应调用的方法，而运行时 Rust 使用 trait 对象内的指针可知晓调用的方法(查找
       会有运行时开销[静态分发不会有],动态分发也阻止编译器选择内联方法的代码[会阻止一些优化])
       故灵活性、优化及性能应权衡取舍

*/