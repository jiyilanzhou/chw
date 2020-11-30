/*
0. Rust 所有权语义在表达式上的体现
     当位置表达式出现在了值上下文中
      let stack_a = 42;   // a 是位置表达式，因为是 42, 实现了 Copy,所以是栈内存
      let stack_b = stack_a;  // a 出现在了值上下文中
      let heap_a = "hello".to_string();   // a 是位置表达式，未实现 Copy,堆内存
      let heap_b = heap_a;    // a 出现在值上下文中，发生了 Move
      // 一般而言，位置表达式通常会出现在位置上下文中，但也会出现在值上下文中.如从语义上来看，定义的
          " stack_a "及" heap_a "代表的都是一个内存位置的值，后被赋值于" static_b "及" heap_b ",这
          就说明其被当作了数据进行传递。而具体位置是在栈内存、堆内存还是静态区域，需要看值表达式的类型。
     // Rust 能保证内在安全的本质原因就是依赖类型系统对值类型的各种检查.上述代码" static_a "变量会被
        存到栈内存，因为值表达式 42 是一个基本数据类型，实现了 Copy trait ，有 Copy 语义，故可安全在
        栈内存上进行复制，这意味着当 static_a 出现在值上下文的时候也就是其被当作数据传递给新的绑定变量
        static_b 的时候，它会复制一份新的值给 static_b，而 static_a 变量仍可继续使用
     // 而 heap_a 是一个动态可增长的 String 类型的字符串，在运行时可动态增长的字符串需要存储于堆上，而
        在栈内存上只允许存储指针。其作为值表达式即 heap_a 被当作数据赋值给一个新的绑定 heap_b 的时候，
        它实际上只是一个携带有长度等信息的栈上指针，实际的数据仍存储于堆中，若此时复制一个 heap_a 那么
        栈内存会出现两个指向同一堆内存的指针，故对于 String 类型在绑定给新的变量时则不支持 Copy 语义，
        而是 Move 语义，如此才能完全管理内存(即只有一个所有者)，Move 后 heap_a 将不能再被使用，但指向
        堆内存的指针未曾改变，只是移动到了新的栈内存位置(即" heap_b ")被存储而已。所以当位置表达式出现
        在值上下文的时候，意味着存储位置的 Move 或 Copy. 这就是所有权机制在表达式上的体现
   (图"6_Copy 语义代表可以安全在栈内存复制.png"及图"7_Move 语义代表必须旧的绑定失效(避免内存不安全).png")

1. 不可变与可变
    a. Rust 借鉴了函数式语言的不可变(即默认"不可变"),包括：
        • 不可变绑定与可变绑定
        • 不可变引用与可变引用
    b. 不可变绑定
        默认不可变(图"8_不可变绑定(默认不可变).png")
        let answer = 42;
        // answer 不可变即意味着其背后内存上的数据不允许再次改变(不可多次给不可变变量赋值)
    c. 变量遮蔽 ：继承式可变(inherited mutable)
        let answer = 42;
        let answer = 43;
        // 本质上前后两个 answer 已经是两个不同变量的绑定了，只是变量名相同而已
        // 继承式可变包括不可变和可变的统称：因其是和内部可变性相对应的概念
    d. 可变绑定：使用 mut[mʌt] 修饰符(图"9_可变绑定.png")
        let mut answer = 42;   // answer 可变意味着其背后内存上的数据允许再次修改(可多次赋值)
        // 无论"不可变绑定"还是"可变绑定"，Rust 的所有权机制，每个内存位置只允许有唯一的绑定存在。Rust 也
           支持类似于 C 语言" *const T 及 * mut T "的原始指针，但只能在 Unsafe Rust 中才能对这种指针进行
           操作。Rust 认为原始指针不是很安全，容易引起内存不完全的问题，所以在 Safe Rust 中通常使用"引用"
    e. 不可变引用
        let answer = 42;
        let r = &answer;
        // r 引用本质上是一个地址指针，但却比指针多了很多安全检查. 单元引用操作符 & 实际上是从 answer 绑定
           中得到一个内存地址。值得注意的是引用(" let r = &answer; ")也是不可变的，所以即使得到了内存地址
           也无法对内存中的数据进行更改(同理欲更改则需要可变引用)
    f. 可变引用
        let mut answer = 42;  // 此处 answer 使用 mut 修饰表示绑定的内存数据可更改
        let r = &mut answer;  // [自]此处使用 &mut 修饰表示 r 指向的引用可变
                              // 获取 answer 绑定的可变引用（获取可变引用方能修改）
        *r = 43;
        println!("{:?}",answer);        // 43
        // 注: 不可变引用可同时有多个，因其不能修改内存位置的数据；但可变引用同时只能存在唯一一个，因为它会
                修改内存位置数据，所以只能保持独占，为了保证内存安全. 可变引用就好比一把内存锁，拿到可变
                引用方能修改。所以把不可变引用称为共享引用、不可变引用称为独占引用。

2. 小结
    a. Rust 没有 GC 却可安全控制资源管理内存，因其首先依赖 Rust 中"位置表达式"及"值表达式"，位置表达式代表
       的是内存位置，而值表达式代表的是数据值本身。
    b. let 声明的绑定是代表了内存位置和数据的绑定，可使用此绑定来访问数据，但其默认不可变。欲修改可用 mut
       修饰符使其为"可变绑定"或使用变量遮蔽的方式。 Rust 中的一些资源把这种不可变与可变的特性统一称为继承式
       可变( mut 修饰或变量遮蔽)，而数据值在内存中如何存储由其类型决定。
    c. Rust 编译器会判断值表达式的类型是否实现了 Copy 语义来决定其存储在栈上或堆上。
    d. 当位置表达式处于值上下文时即是传递内存位置绑定的数据( Copy 语义还是 Move 语义取决于大数据类型)，若
       其实现 Copy 语义则为复制，否则就仅仅是转移栈上的指针即移动语义，这都是 Rust 为了保证内存安全而制定
       的规则，因为每个内存位置只允许有唯一的绑定(在实现开发中非常不便故 Rust 也支持类 C 语言" *const T "
       及" * mut T "的指针，但仅允许在" Unsafe Rust "中使用，为了保证安全 Rust 推荐使用"引用")
    e. 引用分为不可变引用及可变引用(亦称为"共享引用"及"独占引用")：引用实际上就是从绑定变量获取内存地址(与
       指针类型但其还多了一个"类型安全的检查")，引用可直接访问内存数据(而无需绑定变量去访问，所以在性能上会
       更快，可避免绑定变量的一些复制开销)

作业：
    除了 let 赋值语句的值上下文之外，你还能找出哪些值上下文？请写一些代码示例去尝试一下

 */
fn main() {}