
/*
0. 函数式语言特性：迭代器与闭包
   闭包和迭代器是编写符合语言风格、高性能代码的重要一环(务必掌握)

1. 使用闭包存储代码来进行重构
   闭包使用场景之一即是想要在一个地方定义要调用的代码，将其存储起来，并在稍后的地方按需调用

2. 使用泛型参数和 Fn trait 来存储闭包
    a. 因为结构体各字段类型在定义时必须确定，但每一个闭包实例都有它自己的匿名类型。换言之
       即使两个闭包拥有完全相同的签名，它们的类型也被认为不一样。为了在结构体、枚举或函数
       参数中使用闭包，则需要使用在第 10 章讨论过的" 泛型及 trait 约束 "
    b. 标准库提供一系列 Fn trait 而所有闭包都至少实现 Fn、FnMut 及 FnOnce 中的一个 trait

3. 使用闭包捕获上下文环境(p368[*])
    a. 闭包可捕获自己所在的环境并访问自身被定义时的作用域中的变量(区别于函数)
    b. 当闭包从环境中捕获值时会存储于闭包体中以供使用(会占用并产生额外开销)，在一般场景中
       若无需闭包来捕获环境则定义和使用函数(因函数从未允许捕获环境故不会有额外开销)
    c. 闭包可通过 3 种方式从环境中捕获值，这与函数接收参数的 3 种方式完全一致：获取所有权，
       可变借用及不可变借用。(分别对应" FnOnce trait、FnMut trait 及 Fn trait)
    d. 大多数情况下当需要指定某一个 Fn 系列的 trait 时可先尝试使用 Fn trait，编译器会根据
       闭包体中的具体情况来告知是否需要 FnMut 或 FnOnce
    e. 若希望强制闭包获取其使用环境值的所有权，则可在参数列表前使用 move 关键字(此用法技巧
       在将闭包传递给新线程以便将数据移动到新线程中时最为实用)

4. 使用迭代器处理元素序列
    a. 迭代器模式允许依次为序列中的每一个元素进行某些处理。迭代器(iterator)负责遍历序列中的
       每一项和决定序列何时结束的逻辑(使用迭代器可避免手动去实现这些逻辑)
    b. 在 Rust 中迭代器是惰性的(lazy)，即调用相应方法消费迭代器前不会产生任何的实际效果

5. Iterator trait 和 next 方法
   a. 迭代器都实现了定义于标准库的 Iterator trait ，类似：
       // " type Item "及" Self::Item "定义了 trait 的关联类型(associated type)
       pub trait Iterator {
           // 为实现 Iterator trait 必须定义一个具体的 Item 类型(用作 next 方法的返回值
           // 类型[换言之 Item 类型将是迭代器返回元素的类型])
           type Item;
           // Iterator trait 仅要求实现者手动实现 next 方法，其会在每一次调用时返回一个
           // 包裹在 Some 中的迭代器元素并在迭代器结束时返回 None
           fn next(&mut self) -> Option<Self::Item>;
           //...--snip---
       }
   b. 手动调用迭代器的 next 方法
        let v1 = vec![1, 2, 3];
        /* v1_iter 必须是可变的：因为调用 next 方法改变了迭代器内部用来记录序列位置的状态。
           换言之代码消耗或使用了迭代器，每次调用 next 都会从迭代器中消费一个元素，用 for
           循环时不要求 v1_iter 可变是因为 for 循环会取得了 v1_iter 的所有权并在内部使得
           它可变了。
           另外需要注意到，iter 方法生成一个不可变引用的迭代器，通过 next 取得的值实际上是
           指向动态数组(vector)中各个元素的不可变引用。
           若需要获取 v1 所有权并返回元素本身的迭代器则可使用 into_iter 方法；若需要可变
           引用的迭代器可使用 iter_mut 方法
        */
        let mut v1_iter = v1.iter();
        // 重复调用迭代器(由 vector 创建)的 next 方法获取值
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);

6. 消费迭代器的方法(p374)
   a.  Iterator trait 有一系列不同的由标准库提供默认实现的方法;可在 Iterator trait 标准库
       API 文档中查询(一些方法在其定义中调用了 next 方法[这亦是实现"Iterator trait"时要求
      手动实现 next 方法的原因])
   b. 调用 next 的方法称为"消费适配器(consuming adaptors):因其调用会消耗迭代器"，一个消费
      适配器的例子是 sum 方法(获取迭代器的所有权并反复调用 next 来遍历元素进而导致迭代器被
      消耗，在迭代过程中其会对所有元素进行求和并在迭代结束后将总和作为结果返回

7. 生成其它迭代器的方法
   a. Iterator trait 中还定义了迭代器适配器(iterator adaptor)的方法，这些方法可以将已有的
      迭代器转换成其它不同类型的迭代器。可链式调用多个迭代器适配器完成一些复杂操作且易读性强
   b. 迭代器适配器是惰性的(常用的迭代器适配器方法: map、filter、zip 等)
   c. 迭代器消费器：collect、sum 等
    
8. 使用 Iterator trait 来创建自定义迭代器(p379[*])
   一旦实现了 Iterator trait 即拥有了一个迭代器。提供 next 方法实现后可以使用迭代器的 next
   方法，亦可使用标准库那些拥有默认实现的 Iterator trait 方法(因它们皆依赖于 next 方法功能)

9. 改进 I/O 项目
   与其将迭代器产生的值收集至动态数组后再作为切片传入，不如选择直接传递迭代器本身

10. 使用迭代器让代码更加清晰(p385[*])
    函数式编程风格倾向于在程序中最小化可变状态的数量来使代码更加清晰(通过迭代器适配器方法)

*/