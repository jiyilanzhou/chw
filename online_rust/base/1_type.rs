/*
0. 基本类型、变量和函数
    常见的基本类型（标量类型）
    有符号整型（signed integers）:i8、i16、i32、i64 和 size（指针大小）
    无符号整型（unsigned integers）:u8、u16、u32、u64 和 usize（指针大小）
    浮点类型（float point）:f32、f64
    char（字符）:单个 Unicode 字符如 'a'
    bool（布尔型）: true、false
    单元类型（unit type）: 有且仅有一个值即" () "

1. 复合类型
    数组(array) : 如 [1,2,3]
    元组(tuple) : 如 (1,true)

 */
fn main() {
    // 类型最值     // Console:" 255, 0 "
    println!("u8 最大值: {},\tu8 最小值: {}",u8::max_value(),u8::min_value());
}
