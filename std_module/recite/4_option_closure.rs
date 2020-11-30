
/*
0. Optioin

 */

/*
1. 为何不能直接使用函数类型[参见如下分析]
   a. 直接使用函数类型" Fn(u32) -> u32 "
        struct Cacher {
            calculation: Fn(u32) -> u32,
            value: Option<u32>,
        }
        /* 编译报错：
        error[E0277]: the size for values of type `(dyn std::ops::Fn(u32) -> u32 + 'static)`
                                                        cannot be known at compilation time
            calculation: Fn(u32) -> u32,
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
          = help: the trait `std::marker::Sized` is not implemented for
                                                 `(dyn std::ops::Fn(u32) -> u32 + 'static)`
          = note: only the last field of a struct may have a dynamically sized type
        //分析:因为结构体各字段类型在定义时必须确定，但每一个闭包实例都有它自己的匿名类型。换言之
              即使两个闭包拥有完全相同的签名，它们的类型也被认为不一样。为了在结构体、枚举或函数
              参数中使用闭包，则需要使用在第10章讨论过的" 泛型及 trait 约束 "
        */
    b. 使用" trait bound "指定泛型 T (其使用 Fn 的闭包)
        struct Cacher<T>
            where T: Fn(u32) -> u32
        {
            calculation: T,      // "calculation"存放闭包
            value: Option<u32>, // "value"存放 Option 值
        }
       // 正常编译

*/