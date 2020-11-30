/*
0. 使用私有属性
   在另一文件内如" model.rs "内 AccountId32 的定义为" pub struct AccountId32([u8; 32]); "
        pub struct AccountId32([u8; 32]);
        impl From<[u8; 32]> for AccountId32 {
            fn from(x: [u8; 32]) -> AccountId32 {
                AccountId32(x)
            }
        }
        impl From<AccountId32> for [u8; 32] {
            fn from(x: AccountId32) -> [u8; 32] {
                x.0
            }
        }
 */
mod model;
fn main_0() {
    /* 编译错误：使用私有属性构建实例
        error[E0423]: expected function, tuple struct or tuple variant, found struct `AccountId32`
            let b = AccountId32([0 as u8; 32]);
                    ^^^^^^^^^^^ constructor is not visible here due to private fields
       //解析：另一文件内 AccountId32 的属性声明为私有即" pub struct AccountId32([u8; 32]); "故不可直接初始化
           解决方案 1 ：将私有属性暴露为公开即将其修改为" pub struct AccountId32(pub [u8; 32]); "
           解决方案 2 ：调用对象相应关联函数(一般将属性设置为私有都会提供相应的如 from_xx 关联函数或实现 From trait
                      来初始化)
     */
    // model::AccountId32([0 as u8; 32]);   // 暂且注释：以通过编译

    model::AccountId32::from([0 as u8;32]);

}

// 切片转数组(待深入，此例可删除)
use std::convert::AsMut;
fn clone_into_array<A, T>(slice: &[T]) -> A
    where
        A: Default + AsMut<[T]>,
        T: Clone,
{
    let mut a = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}
fn main() {
    let original = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let e:[i32;4]=  clone_into_array(&original[0..4]);
    println!("{:?}", e);
}