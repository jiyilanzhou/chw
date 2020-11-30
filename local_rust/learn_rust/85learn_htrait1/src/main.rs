
/*
1、关联类型在 trait 定义中指定占位符类型
    a. 关联类型：将类型占位符(" Item ")与 trait 相关联
    b. trait 实现：针对特定实现(在此类型的位置)指定相应的具体类型。
       (通过此种方式可定义多种类型[如" T "指定为" i32,String "
       类型等]的 trait )

*/
/*
    // 实现方式 1：使用占位符(" Item "一旦定义则只能实现单一类型)
    pub trait Iterator {
        // 占位符类型
        type Item;     // 返回值 Option 内使用" 占位符 "类型
        fn next(&mut self) -> Option<Self::Item>;
    }
*/

// 实现方式 2：使用泛型可实现多种类型(如"i32/String"等[因从外部传入])
pub trait Iterator1<T> {
    fn next(&mut self) -> Option<T>;
}
struct A {
    value: i32,
}
impl Iterator1<i32> for A {     // Iterator i32 实现
    fn next(&mut self) -> Option<i32> {
        println!("in i32");
        if self.value > 3 {
            self.value += 1;
            Some(self.value)
        } else {
            None
        }
    }
}
impl Iterator1<String> for A {  // Iterator String 实现
    fn next(&mut self) -> Option<String> {
        println!("in string");
        if self.value > 3 {
            self.value += 1;
            Some(String::from("hello"))
        } else {
            None
        }
    }
}

fn main() {
    let mut a = A{value: 3};
    /* 编译报错：不能推导具体类型
        error[E0282]: type annotations needed
            a.next();
              ^^^^ cannot infer type for `T`

    */
    // a.next(); // 暂且注释：以通过编译

    /* 完全限定语法：指定具体类型
       a. 使用泛型需标注具体类型：如"Iterator1<i32>/Iterator1<String>"
       b. 若使用"占位符"类型则无需指定具体类型如
             pub trait Iterator {
                 type Item;
                 fn next(&mut self) -> Option<Self::Item>;
             }
    */
    <A as Iterator1<i32>>::next(&mut a);
    <A as Iterator1<String>>::next(&mut a);

}
