/*
0. 模块导入不支持类似" ../ "的语法
   解决方案：使用" use crate "(即项目[可理解为顶级目录])
             (直接使用 use 无需再使用 mod 字样)
*/
// mod "../api/product "; // 模块导入不支持类似" ../ "的语法

#[derive(Debug)]
pub struct Book {
    pub id: i32,
    pub price: f32,
}

/*
// 直接使用 use 导入并使用相应模块
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
 */
