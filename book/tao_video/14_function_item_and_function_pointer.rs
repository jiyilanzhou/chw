
/*
0. 函数与闭包(Rust 一切皆类型且由 trait 来掌控类型的行为逻辑)
    a. 常规函数：函数项类型
    b. 函数指针：函数指针类型
    c. 闭包: 闭包与函数的异同

1. 常规函数特点
    a. 函数都拥有显式的类型签名
    b. 函数可以分为三种类型：自由函数、关联函数 和 方法
    c. 函数自身也是一种类型

2. (常规函数)函数项类型

 */
// (常规函数)自由函数(普通函数)：参数及返回值类型皆一目了然
fn sum(a: i32, b: i32) -> i32 {
    a + b
}
// (常规函数)关联函数及方法
struct A(i32, i32);
impl A {
    // 关联函数(在结构体内部实现故为结构体的关联函数[而非自由函数]故调用需使用" A:: "前缀约束)
    fn sum(a: i32, b: i32) -> i32 {
        a + b
    }
    // 方法 ：即首参数为与 self (具体类型实例)相关( self、&self 或 &mut self )的关联函数
    fn math(&self, other: i32) -> i32 {
        Self::sum(self.0, self.1) + other
    }
}
// 常规函数
fn main_0() {
    // 普通函数
    assert_eq!(3, sum(1, 2));
    // 编译器在编译期识别出违反契约的调用
    // sum(1, "2");    // 飘红报错：不满足函数签名类型约定

    // 关联函数及方法
    let a = A(1, 2);
    assert_eq!(3, A::sum(1, 2));
    assert_eq!(6, a.math(3));
    assert_eq!(6, A::math(&a, 3));

    /* 函数项类型:
        a. 变量 add 被赋值于结构体 A 的函数名，等号右侧是值表达式，故此函数名亦可看作是一个表达式，它的值
           是函数的相关信息(如函数名、参数、生命周期等)，它的类型是函数项类型即" A::sum "(赋值于变量 add)
           从而 add 就拥有了函数 sum 的信息(故 add 可被当作函数被调用).
        b. 同理 A::math 方法亦是函数(可通过实例和点操作符来调用)，但被赋值后的变量 add_math 变量只能通过
           普通函数方式调用即" add_math(&a, 3) "(而不能使用点操作符方式进行调用)
        c. 值得注意的是：函数项是一个零大小的类型，记录函数信息(优化函数调用)。同样 Rust 中的枚举体及元素
           结构体等类型构造体，与函数项一致，都是携带了类型信息的零大小类型，享受零大小类型的所有优化。如：
                enum Color {
                    R(i16),
                    G(i16),
                    B(i16),
                }
                // " Color::R "即是一个类型构造体(零大小类型):等价于一个函数项
                // fn Color::R(_1: i16) -> Color { ... }
                // fn Color::G(_1: i16) -> Color { ... }
                // fn Color::G(_1: i16) -> Color { ... }
                println!("{:?}",std::mem::size_of_val(&Color::R));  // Console:" 0 "
                println!("{:?}",std::mem::size_of::<Color>());  // Console:" 4 "
        d. Rust 默认给函数项实现一些 Trait 如" Copy、Clone、Sync、Send、Fn、FnMut、FnOnce "等(为方便
           复用、传递及和闭包转换等)
     */
    let add = A::sum;     // A::sum 是一个 Fn item 类型
    let add_math = A::math; // A::math 也是一个 Fn item 类型
    assert_eq!(add(1, 2), A::sum(1, 2));
    assert_eq!(add_math(&a, 3), A::math(&a, 3));

}

/*
3. 函数项隐式转换为函数指针(图"17_函数项与函数指针.png")
    a. 函数项亦可作为函数参数
    b. 函数项类型可以通过显式指定函数类型转换为一个函数指针类型
    c. 编码时尽可能地使用函数项类型(享受零大小类型的优化),不到万不得已不要使用函数指针类型(占用空间)

 */
// 定义类型别名
type RGB = (i16, i16, i16);
fn color(c: &str) -> RGB {
    (1, 1, 1)
}
// 参数为函数类型
fn show(c: fn(&str) -> RGB) {
    println!("{:?}", c("black"));
}
fn main_1() {
    /* rgb 为函数项类型(源于函数项赋值):
       而 show 函数参数明确指定函数类型即" fn(&str) -> RGB ",故当 rgb(函数项类型) 传入 show 函数时,会被
       隐式转换为"函数指针类型"，类似于如下代码：
           let rgb = color；    // 函数项类型( Fn item Type )
           let c: fn(&str) -> RGB = rgb;   // 隐式转换为"函数指针类型( fn pointer Type )"
           // 使用 std::mem::size_of_val 去测量函数项与函数指针：
           println!("{:?}",std::mem::size_of_val(&rgb));  // Console:" 0 "
           // 函数指针实际上占用 8 个字节大小的空间
           println!("{:?}",std::mem::size_of_val(&c));  // Console:" 8 "
     */
    let rgb = color;
    show(rgb);  // Console:" (1, 1, 1) "
    println!("{:?}",std::mem::size_of_val(&rgb));  // Console:" 0 "
    // 函数指针实际上占用 8 个字节大小的空间
    let c: fn(&str) -> RGB = rgb;
    println!("{:?}",std::mem::size_of_val(&c));  // Console:" 8 "
}

// 函数无法捕获环境变量
fn counter_0(i: i32) -> fn(i32) -> i32 {
    /* 函数没有捕获环境变量中自由变量的权利：
         inc 函数想要捕获自由变量 i 但 Rust 属于执法作用域。i 是 counter 函数的参数，虽然 inc 在 counter
         内部但是 i 依然是 inc 函数的外部，并不允许被 inc 函数访问。(欲捕获环境变量可使用"闭包")
     */
    fn inc(n: i32) -> i32 {
        n + i       // error[E0434]: can't capture dynamic environment in a fn item
    }
    inc
}

/*
4. 闭包
    a. 闭包可捕获环境中的自由变量
    b. Rust 中闭包实现基于 trait (故其返回值可用 impl trait 的语法[代表实现 FnMut trait 并携带参数以及
       返回值的类型])
    c. 闭包除可捕获环境变量外亦可与指针互通(闭包用法与函数项、函数指针类似)

5. 小结(函数、函数项、函数指针及闭包)
    a. Rust 中函数签名要求显式指定类型(便于编译器检查，保证类型安全)
    b. 函数名亦是表达式，其表达式的值是函数的相关信息如类型名、参数类型名、生命周期、函数名等，它的类型是
       函数项类型(零大小类型)
    c. 通过显式指定函数类型，函数项类型就可被隐式转化为函数指针类型，因携带指针信息故占用额外空间，故非必要
       时无需使用函数指针类型(占用额外空间)
    d. 通过函数项和函数指针，函数作为 Rust 一等公民可以在函数间进行传递，也就是函数是语言中的高阶函数。函数
       虽然强大但其无法捕获环境变量，此时可用闭包
    e. Rust 中的闭包基于 trait 来实现，其行为与函数项、函数指针一致，并且闭包亦可作为函数的参数及返回值

 */
// counter 函数返回"函数指针类型"
fn counter(i: i32) -> impl FnMut(i32) -> i32 {
    // 闭包捕获环境变量 ：move 关键字将变量参数 i 所有权转移到闭包中
    move |n| n + i
}
fn main() {
    // 函数项、函数指针
    let mut f = counter(2);
    println!("{}",std::mem::size_of_val(&f));   // 函数指针占用 4 byte
    assert_eq!(3, f(1));

    // 闭包与函数指针互通
    let rgb = color;
    show(rgb);  // Console:" (1, 1, 1) "
    // 此处闭包须显式声明类型" s: &str "(因闭包作为函数参数传入时编译器无法正确推断其类型)
    let c = |s: &str|{(1, 2, 3)};// 定义实现" Fn(&str) -> RGB " trait 的闭包类型
    show(c);
}
