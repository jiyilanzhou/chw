
// algebraic[ˌældʒɪˈbreɪk]adj.代数(的)
// product[ˈprɒdʌkt]n.产品,[数]乘积      // varient[ˈveəriənt]n.变体
/*
0. 结构体编程

7.1 结构体
    结构体(Struct)和枚举体(Enum)是 Rust 中最基本的两种复合类型。对于 Rust
    类型系统而言它们都属于"代数数据类型(Algebraic Data Type, ADT)"。代数
    数据类型概念源自"函数式语言"(尤其在 Haskell 中应用最广)
    a. 代数数据类型之积类型([自]Product Type)
       (1) 代数数据类型就是指具备代数能力的数据类型即数据类型可以进行代数
           运算并满足一定的运算规则(如加法或乘法满足交换律及结合律)。
       (2) 结构体的更新语法(update syntax)允许使用" .. "语法来减少代码重复
       (3) Rust 中的结构体属于代数数据类型中的"积类型"(源于范畴论术语)
    b. " std::default "模块中提供的" Default trait "，Rust 已经为内置的
        大部分类型实现了 Default

7.1.2 枚举体
    代数数据类型之和类型(Sum Type)
       枚举体属于代数数据类型中的和类型。枚举体中的成员是值而非类型(区别于
       结构体)，一般将其称作"变体(Variant)"

7.1.3 析构顺序
    本地变量
    元组
    结构体和枚举体

7.2 常用设计模式
    建造者模式(Builder Pattern)

7.2.2 访问者模式(Visitor Pattern)
    a. 访问者模式一般包含两个层次
       定义需要操作的元素
       定义相关操作
    b. 第三方库 Serde
       Serde 命名源于 Serialize(序列化) 和 Deserialize(反序列化) 组合

7.2.3 RAII 模式

*/