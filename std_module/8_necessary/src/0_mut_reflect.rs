/*
0.  mut a: &T 和 a: &mut T
    a. " mut a : &T "即变量可变
    b. " a: &mut T "即引用可变

 */
#[derive(Debug)]
struct SN(usize);

fn main() {
    /* 0. mut 可用于修饰变量可变性:
          对于" mut a : &SN "而言，mut 修饰的是变量 a 本身(而非指向的不可变引用" &SN ")
          故可修改变量 a 的值(因其可变)但不能对 a 引用的值进行修改(因 a 引用的类型" &SN "
          不可变)；若将 a 作为参数传递则其类型为" &SN "即" func( [mut] prm : &Type) "，
          至于 prm 前是否有" mut "则是针对函数作用域内局部变量的处理。
          // 不存在" func (&mut prm : &Type ) "的用法：因对引用类型变量的引用即是二级
             指针即" func (mut prm : &&Type ) "
       1. mut 可用于修饰引用可变性：
          对于" a : &mut SN "而言，mut 修饰的是变量 a 指向类型的引用值(而非变量 a 本身)
          故可修改变量指向的引用值(因其可变即"&mut SN")但不能修改变量 a 的指向(因不可变)
          // 不存在" func ( i : mut Type ) "的用法(因类型不可能变[编译期已固定])
     */
    // mut 可用于修饰变量可变性
    let mut a: &SN = &SN(0);
    a = &SN(3);
    // Cannot assign to field of immutable binding [E0594]
    // a.0 = 7;
    fn mut_variable(mut i: &&SN) {}
    mut_variable(&mut a);
    println!("{:?}", a);

    // mut 用于修饰引用的可变性(即可变引用)
    let c = &mut SN(42);
    // Cannot assign twice to immutable variable [E0384]
    // c = &SN(36);
    c.0 = 100;

}