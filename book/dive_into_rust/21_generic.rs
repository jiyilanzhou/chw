
/*
0. 泛型(Generics)

21.1 数据结构中的泛型(p234[*])
    泛型参数可有多个亦可有默认值如" struct S<T=i32>{ data:T } "

21.2 函数中的泛型

21.3 impl 块中的泛型
    a. "impl<T> Trait for Type {} "泛型可出现于 Trait、Type 位置
    b. 标准库" Into trait "及" From trait "

21.4 泛型参数约束
    a. 在 Rust 中只有 impl 了 PartialOrd 的类型才能支持比较运算符
    b. 存在关联类型时 where 子句比参数中的冒号约束具有更强的表达能力(参数中
       的冒号约束有时无法表达"关联类型"约束)如:
           trait Iterator {
                type Item;
               // fn max(self:Sized)->Option<Self::Item:Ord>{} //飘红报错
               fn max(self)->Option<Self::Item>
                   where Self:Sized,Self::Item:Ord
               {...}
            }
    c. " 冒号约束 "一定可转换为 where 子句表达(反之不成立)

21.5 关联类型(associated type)：(p241[*])
    a. 关联类型亦是 trait 的" 泛型参数 "
    b. 可读性可扩展性：某些情况下"关联类型"比"普通泛型参数"更具可读性
    c. trait 的 impl 匹配规则
       采用"关联类型"不能针对同一类型有多个实现

21.6 何时使用关联类型(p244[*])
    a. 输入类型参数 ：尖括号存在的泛型参数
    b. 输出类型参数 ：trait 内部存在的关联类型

21.7 泛型特化(p246[*])

21.7.1 特化的意义

21.7.2 default 上下文关键字

21.7.3 交叉 impl

*/