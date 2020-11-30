
/*
2.7 复合数据类型
    Rust 提供了四种复合数据类型，分别是：
        元组(Tuple)
        结构体(Struct)
        枚举休(Enum)
        联合体(Union)
    这 4 种数据类型都是异构数据，意味着可使用它们构建为统一的数据类型

2.7.1 元组
    元组是一种异构有限序列，形如(T,U,M,N)。异构即是指元组内的元素可为不同数据类型；有限即是指
    元组长度固定(注:当元组中仅有单值时亦需添加逗号如" (0,) "[区分于括号中的其它值如" (0) "])

*/
// coords [kɔːdz]n.坐标
// 元组可让函数返回多个值
fn move_coords(x: (i32, i32)) -> (i32, i32) {
    (x.0 + 1, x.1 + 1)
}
// 元组仅有单值亦需添加逗号：区分于括号内的其它值如" (0) "
fn move_single(x:(i32,))->(i32,){
    (x.0 + 1,)
}
fn main_004_000() {
    // 定义类型为" (&'static str, i32, char) "的元组 tuple (可通过索引获取元组内元素值)
    let tuple: (&'static str, i32, char) = ("hello", 5, 'c'); // 长度为 3 的异构序列
    assert_eq!(tuple.0, "hello");  // 通过索引获取元组元素
    assert_eq!(tuple.1, 5);
    assert_eq!(tuple.2, 'c');
    // 利用元组可让函数返回多个值：
    let coords = (0, 1);
    let result = move_coords(coords);
    assert_eq!(result, (1, 2));  //  move_coords 函数返回元组
    //  解构元组并绑定至匹配位置的"x,y"元素(因 let 支持模式匹配故可用来解构元组)
    let (x, y) = move_coords(coords);  // let 模式匹配解构
    assert_eq!(x, 1);
    assert_eq!(y, 2);
    // 元组中仅有单值时亦需添加逗号如" (0,) " : 区分于括号中的其它值如" (0) "
    let single_tuple = (2,);  // 没有逗号则为普通类型(而非元组)不可使用"."运算符
    let sin_tup = move_single(single_tuple);
    assert_eq!(sin_tup.0,3);
}

/*  // enum ['enəm]/[ɪˌnjuːm]n.枚举
2.7.2 结构体
    Rust 提供三种结构体：
        具名结构体(Named-Field Struct)
        元组结构体(Tuple-Like Struct)
        单元结构体(Unit-Like Struct)
    其中具名结构体(Named-Field Struct)最为常见

*/
// (代码清单 2-34 )具名结构体示例 :
// 自动实现 Debug trait 及 PartialEq trait (允许对结构体实例进行打印和比较)
#[derive(Debug, PartialEq)]
struct People { // struct 关键字定义结构体(名称遵从驼峰式命名)
    /* a. 结构体字段格式:
             " fieldName: fieldType "(故为"具名结构体")
       b. 结构体字段默认不可变，其可为任意类型(甚至是结构体本身)
    */
    name: &'static str,
    gender: u32,
}
/* 函数与方法：
     a. 非" impl "块内定义的函数为" 自由([自]普通)函数 "
     b. " impl "块定义则为"关联函数(未绑定 &self )"或"方法(绑定 &self : 代表
        对结构体实例自身的引用[面向对象思想]以便可使用圆点"."记号调用)"
*/
impl People {
    // 关联函数 new : 类比面向对象语言中类的"构造函数"( Rust 中无构造函数)
    fn new(name: &'static str, gender: u32) -> Self {
        People {
            name,
            gender,
        }
    }
    /* name 方法(类比 getter )及 set_name 方法(类比 setter )：用于获取和修改成员
       变量的具体值(注意两个方法签名中的" &self "及" &mut self "的用法)
    */
    fn name(&self) {
        println!("name: {:?}", self.name);
    }
    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }
    fn gender(&self) {
        let gender = if (self.gender == 1) { "boy" } else { "girl" };
        println!("gender: {:?}", self.gender);
    }
}

fn main_004_001() {
    // 通过 People::new 方法来创建 People 结构体实例
    let alex = People::new("Alex", 1);
    // 通过圆点记号调用结构体中的方法( Rust 具名结构体是面向对象思想的一种体现)
    alex.name();
    alex.gender();
    /* 结构体初始化必须指明成员: 错误示范(" People {"Alex", 1} ")
       (即使是全属性顺序初始化[因 Rust 注重安全]:区别于" Golang ")
    */
    assert_eq!(alex, People { name: "Alex", gender: 1 });
    let mut alice = People::new("Alice", 0);
    // 圆点"."记号调用结构体上定义的方法
    alice.name();
    alice.gender();
    assert_eq!(alice, People { name: "Alice", gender: 0 });
    alice.set_name("Rose");
    alice.name();
    assert_eq!(alice, People { name: "Rose", gender: 0 });
}

// (代码清单 2-36 ):元组结构体示例
/* 元组结构体：其特点是仅有类型而无字段名表象类似元组和结构体的混合体。
   使用" () "定义元组结构体(表象类似"具名的元组")：区别于结构体" {} "。
   元组结构体访问方式：亦是使用圆点记号按索引位置访问(同元组访问一样)。
   当元组结构体仅有单一字段时称之为" New Type "模式([自]即包装为新类型)。
*/
struct Color(i32, i32, i32);
fn main_004_002() {
    let color = Color(0, 1, 2);
    assert_eq!(color.0, 0);
    assert_eq!(color.1, 1);
    assert_eq!(color.2, 2);
}

// (代码清单 2-37 ): New Type 模式示例
/* 定义 Integer 单字段( u32 类型)结构体：New Type 模式
   (相当于将 u32 类型包装成新 Integer 类型[类比 Golang 中未携带等号"="
    的类型别名如" type Integer uint32 "])。
   (相比类型别名如" type Int = i32; "则 New Type 模式属于自定义类型故
   更加灵活)
*/
struct Integer(u32);
/* type 关键字定义类型别名(其本质还是原类型[类比 Golang 中携带等号
   的类型别名如" type Int = int32 "])。
*/
type Int = i32; // 为 i32 类型创建别名 Int
fn main_004_003() {
    let int = Integer(10);
    assert_eq!(int.0, 10);
    let int: Int = 10;
    assert_eq!(int, 10);
}

// (代码清单 2-38 ):单元结构体示例
/* Rust 中可定义无字段的结构体即"单元结构体"：
   单元结构体实例即是其本身。为同一单元结构体创建的多个实例是否为同一对象
   取决于编译模式(如"Debug、Release"编译模式)
*/
// 定义 Empty 结构体:等价于" Empty{} "
struct Empty;
fn main_004_004() {
    let x = Empty;
    println!("{:p}",&x);
    /* 将 x 赋值给新绑定的 y :
       其内存地址会转移至新的位置表达式 y(因为此时 x 是位置表达式而处于值上下文中)
    */
    let y = x;
    println!("{:p}",&y);
    // 将新的单元结构体实例赋予 z
    let z = Empty;
    /* 通过"{:p}"格式符在 println! 宏语句中打印 &x、&y 和 &z 的内存地址则：
        a. 在" Debug "编译模式下，" x、y "和 z 是不同的内存地址
        b. 在" Release "编译模式下，" x、y "和 z 是相同的内存地址
           // 证明 " Release "编译模式下单元结构体实例会被优化为同一个对象，而
              " Debug "模式下不会进行这样的优化(即非同一对象)
        c. 单元结构体与 New Type 模式类似。单元结构体一般用于一些特定场景，标准库
           中表示全范围(..)的 RangeFull 就是一个单元结构体
    */
    println!("{:p}",&z);
    // 标准库源码中 RangeFull 就是一个单元结构体
    assert_eq!((..),std::ops::RangeFull); //  RangeFull 就是(..)，表示全范围
}

/* 问题：
0. " struct Empty "与" struct Empty() "区别与应用场景
1. 具体结构体 与 元组结构体
   [自]具名结构体用于封装类而元组结构体用于包装类如" struct Integer(u32); "
2. 单元结构体
    一般用于一些特定场景：如标准库中表示全范围(..)的 RangeFull 就是一个单元结构体

*/