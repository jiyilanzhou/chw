/*
0. 数组   (文档" https://doc.rust-lang.org/std/primitive.array.html ")
    a. 大小固定的同一类型集合
    b. 表示：" [T; N] "( N 为非负数 )

1. 数组操作
    a. 创建数组的两种语法形式
        (0). 元素列表 ：[x,y,z]
              产生指定元素的数组
        (1). 重复表达式 : [x; N]
             产生 N 个 x 副本( x 必须为 Copy )的数组
    b. 若元素类型允许则大小范围从 0 ~ 32 的数组实现以下特殊：
        (0). Debug
        (1). IntoIterator (implemented for &[T; N] and &mut [T; N])
        (2). PartialEq, PartialOrd, Eq, Ord
        (3). Hash
        (4). AsRef, AsMut
        (5). Borrow, BorrowMut
        (6). Default
        // 若元素类型是 Copy 则其任何大小的数组都是 Copy；若元素类型是 Clone
           则任何大小的数组都是 Clone (源于 Copy、Clone 特征为编译器所知)
        // 数组可强制为 slices ([T])因此可在数组上调用 slice 方法。实际上切片
           提供了用于处理数组的大多数 API (切片具有动态大小且不强制转换为数组)
        // [自]调用切片方法时数组才会自动强制为切片
    c. 模式匹配
         let [john, roa] = ["John".to_string(), "Roa".to_string()];
         println!("{}",john); // Console: John

*/

fn main_0() {
    // 数组声明：使用重复表达式
    let mut array: [i32; 3] = [0; 3];
    array[1] = 1;
    array[2] = 2;
    /* 数组转为切片可使用范围表达式(亦可调用切片方法将 array 强制为 slice ) :
          如" [ (start) ..(=) (end) ] " // 其中 (args) 可省略
       // 切片转数组: TryFrom trait 之 " try_from "方法
     */
    assert_eq!([1, 2], array[1..]);
    assert_eq!([1, 2], &array[1..=2]);
    // This loop prints: 0 1 2
    for x in &array {
        print!("{} ", x);
    }
}

/*
2. 使用切片模式移出数组元素(待实现)
   可参阅" mem::replace "

 */