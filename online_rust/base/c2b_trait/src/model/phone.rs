
/*
0.  phone 实体

*/

#[derive(Debug)]
pub struct Phone {
    pub id: i32,
    pub price: f32,
}

/*
// 直接使用 use 导入并使用相应模块
use crate::api::Product;
impl Product for Phone {
    fn new(id:i32,price:f32)->Phone{
        Phone{
            id,
            price
        }
    }
    fn get_price(&self) -> f32 {
        &self.price + 10.0
    }
}
*/
