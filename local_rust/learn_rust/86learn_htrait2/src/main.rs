
/*
2、默认泛型类型参数和运算符重载
    a. 使用泛型类型参数时可为泛型指定默认具体类型。
    b. 运算符重载是指在特定情况下自定义运算符行为。
    注: Rust 并不允许创建自定义运算符或重载运算符但对于"std::ops"
       中列出的运算符可通过实现与运算符相关 trait 来重载。

*/

use std::ops::Add;
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
impl Add for Point {
    /* trait 内使用 " type Output "
       a. 其 trait 实现类亦需使用" type Output "定义关联类型(因
          " trait "本身无法确定故其具体实现类型须指定)如：
          "type Output = Point;"/type Output = String;"
       b. 结构体" 实现 trait (内置" type Output ")"与"自身实现"
          区别 : 结构体自实现不能定义"type Output = Point;"[因其
          本为"确定的具体类型"])如：
               impl StructSelf {
                   fn new(param) -> AnotherStruct{
                       AnotherStruct{ //... }
                   }
               }
    */
    type Output = Point;    // 关联类型
    // 重载" + "运算符
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.y,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);
// 指定 RHS 类型为" Meters "(源于"  trait Add<RHS=Self> {//...} ")
impl Add<Meters> for Millimeters{
    type Output = Millimeters;  // 指定 Output 关联类型
    // RHS 用于指定" add "形参" rsh "类型(此处为" Meters ")
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + other.0 * 1000)
    }
}

fn main() {
    // 断言
    assert_eq!(Point{x: 1, y: 1} + Point{x:2, y: 2}, Point{x: 3, y: 3});
    let mi = Millimeters(1);
    let m = Meters(1);
    let r = mi + m;
    println!("r = {:?}", r);    // Console:" Millimeters(1001) "
}

/*  // 尖括号内为默认类型参数( RHS 是泛型类型参数[right hand side])
    trait Add<RHS=Self> {   // RHS 用于指定" add "形参" rsh "类型
        type Output;
        fn add(self, rhs: RHS) -> Self::Output;
    }
*/

trait Interface{
    type Output;
    fn method(self)->Self::Output;
}
struct ImplStruct;
/* 飘红报错 :
    " Not all trait items implemented, missing: `Output` [E0046]"
   原因分析 :
     trait 定义" type Output "其实现类亦必须定义相应的关联类型(否则
     无法确定其 Output 类型)
*/
impl Interface for ImplStruct{
    // type Output = String;    // 去除单行注释可解决飘红
    fn method(self) -> String {
        println!(" 返回基础数据类型");
        String::from("Hello")
    }
}
// 自实现
impl ImplStruct{
    /* 飘红报错：  // 内在 impl (实现)中不允许关联类型
        Associated types are not allowed in inherent impls [E0202]
    */
    type Output = String; // 添加单行注释可解决飘红
    fn method(self) -> String {
        println!(" 返回基础数据类型");
        String::from("Hello")
    }
    fn new()->Point{
        Point{
            x:1,
            y:3
        }
    }
}