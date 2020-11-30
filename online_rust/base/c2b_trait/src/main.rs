/*


 */
// 导入(欲使用先导入)
mod model;
mod api;

// 使用
/* 对应" mod.rs "向外暴露：
    a. "api/mod.rs"向外暴露模块
        use crate::api::prod::Produt;  // 对应 mod.rs 内" pub mod prod; "
    b. "api/mod.rs"向外暴露模块内细节
        use api::Produt; // 对应 mod.rs 内" pub use mod prod::* "(即重导出)
 */
//use crate::api::prod::Produt; // 须将 trait 引入作用域否则不可调用相应方法
// (条件:"api/mod.rs"向外暴露模块内细节[重导出])
use api::Product;
// 导入父级目录
use model::book;
// 直接使用" use crate "的可不必事先导入模块(因其从项目根目录开始索引相应 mod )
use crate::model::{Phone, Book};
use crate::api::Stock;    // 使用重导出

fn main() {
    // 方式 1 ：使用具体实现类调用关联函数
    let b1 = book::Book::new(3, 6.80);
    // 方式 2 ：使用 Trait 调用(条件：必须显式指明实例化的具体类型[因无法推断])
    let b2: book::Book = Product::new(3, 6.80);
    let phone: Phone = Product::new(6, 3.80);
    println!("b1:{},\tb2:{},\tphone:{}",
             b1.get_price(), b2.get_price(), phone.get_price());

    // 多态
    show_price(&b1);
    // 多态(同一类型常使用泛型)
    total_price_0(&b1, &b2);
    // 多态(不同类型常使用 Trait 对象[当然亦可使用泛型])
    total_price_1(&b1, &phone);
    total_price_2(&b1, &phone);

    // 商品库存
    println!("--------\nthe product stock is : {}", b1.get_stock());
    // 显示详情(泛型)
    show_detail_0(b1);
    // 显示详情(Trait对象)
    show_detail_1(b2);

    // 运算符重载(" + ")
    let b1 = Book::new(0,0.0);
    let b2 = Book::new(0,0.0);
    println!("the total stock is : {}", b1 + b2);
}

// 使用 trait bound (Trait 约束)
// fn show_price<T:Product>(prod:T){ ... }
// 使用 trait object (Trait 对象)
fn show_price(prod: &impl Product) {
    println!("the product price is : {}", prod.get_price());
}

// p1、p2 为同一类型
fn total_price_0<T: Product>(p1: &T, p2: &T) {
    println!("the product total price is : {}", p1.get_price() + p2.get_price());
}

// p1、p2 可为不同类型(使用泛型)
fn total_price_1<T: Product, U: Product>(p1: &T, p2: &U) {
    println!("the product total price is : {}", p1.get_price() + p2.get_price());
}

// p1、p2 可为不同类型(使用动态类型)
fn total_price_2(p1: &impl Product, p2: &impl Product) {
    println!("the product total price is : {}", p1.get_price() + p2.get_price());
}

// 显示详情(泛型)
/*
    fn show_detail_0<T: Product + Stock>(ps: T) {
        println!("price:{}\t stock:{}\t", ps.get_price(), ps.get_stock())
    }
 */
fn show_detail_0<T>(ps: T)
    where T: Product + Stock
{
    println!("price:{}\t stock:{}\t", ps.get_price(), ps.get_stock())
}

// 显示详情(trait 对象)
fn show_detail_1(ps: impl Product + Stock) {
    println!("price:{}\t stock:{}\t", ps.get_price(), ps.get_stock())
}

/*
加号" + "操作符重载：" std::ops::Add "源码：
     #[doc(alias = "+")]
     pub trait Add<Rhs = Self> {
          /// The resulting type after applying the `+` operator.
          #[stable(feature = "rust1", since = "1.0.0")]
          type Output;

          /// Performs the `+` operation.
          #[must_use]
          #[stable(feature = "rust1", since = "1.0.0")]
          fn add(self, rhs: Rhs) -> Self::Output;
      }
 */
use std::ops::Add;
/*
    impl Add for Book {
        type Output = i32;
        fn add(self, rhs: Self) -> Self::Output {
            self.get_stock() + rhs.get_stock()
        }
    }
*/
impl Add<Book> for Book{
    // 定义输出的关联类型
    type Output = i32;
    // rhs ：与 self 操作相关的类型
    fn add(self, rhs: Book) -> Self::Output {
        self.get_stock() + rhs.get_stock()
    }
}