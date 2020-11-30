// stock[stɒk]n. 库存,股份
/*
0. 库存

 */

pub trait Stock {
    fn get_stock(&self) -> i32;
}

use crate::model::Book;
impl Stock for Book{
    fn get_stock(&self) -> i32 {
        100
    }
}

use crate::model::Phone;
impl Stock for Phone{
    fn get_stock(&self) -> i32 {
        68
    }
}