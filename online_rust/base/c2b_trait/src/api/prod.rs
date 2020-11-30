/*
0. 抽取共享行为

*/
pub trait Product {
    // Self 为具体实现类型
    fn new(id: i32, price: f32) -> Self;
    fn get_price(&self) -> f32;
}

/* // [自]一般 Trait 的单个具体实现应与具体类型在一起(但多个相同实现
   //     可置于 Trait 在一起使用泛型[降低冗余]；当然亦可使用宏展开)
    use crate::api::Product;
    impl Product for Book {
        fn new(id:i32,price:f32)->Book{
            Book{
                id,
                price
            }
        }
        fn get_price(&self) -> f32 {
            &self.price + 10.0
        }
    }
}*/


// 直接使用 use 导入并使用相应模块
use crate::model::book::Book;
impl Product for Book {
    fn new(id: i32, price: f32) -> Book {
        Book {
            id,
            price,
        }
    }
    fn get_price(&self) -> f32 {
        &self.price + 10.0
    }
}
// 重导出可跨过 phone 模块直接使用 Phone
// use crate::model::phone::Phone;
use crate::model::Phone;
impl Product for Phone {
    fn new(id: i32, price: f32) -> Phone {
        Phone {
            id,
            price,
        }
    }
    fn get_price(&self) -> f32 {
        &self.price + 10.0
    }
}


/*
/* // 编译错误：
    error[E0574]: expected struct, variant or union type, found type parameter `T`
       T {
       ^ not a struct, variant or union type
    // 原因分析：没有实例化泛型的用法(因实例化的具体类型未知[更何况结构体具体的相应字段])
 */
impl<T> Product for T {
    fn new(id: i32, price: f32) -> T {
        T {         // 不可直接将泛型当作具体类型实例化(因无法预知具体实现类型的字段)
            id,
            price,
        }
    }
    fn get_price(&self) -> f32 {
        &self.price + 10.0
    }
}
 */

/*
// 使用宏展开(待实现[???])
use crate::model::Book;
use crate::model::Phone;
decl_prod! {Book}
decl_prod! {Phone}

// 宏声明
macro_rules! decl_prod {
    ($x:expr)=>{
      /*  impl Product for x {
            fn new(id: i32, price: f32) -> x {
                x {
                    id,
                    price,
                }
            }
            fn get_price(&self) -> f32 {
                &self.price + 10.0
            }
        }*/
        println!("Hello world");
    }
}
 */