
// 数据类型：基本数据类型及引用数据类型
fn main() {
    // bool
    let is_true: bool = true;
    println!("is_true = {}", is_true);
    // 自动推导类型
    let is_false = false;
    println!("is_false = {}, {}", is_false, is_true);

    // char 在 rust 里面，char 是 32 位的 ( C 语言中的 char 为 8 位)
    // 故 rust 中的 char 可以是一个" 汉字 "
    // ( C 语言中 char 只能为一个"字符"[因 8 bit 无法容纳一个汉字])
    let a = 'a';    // 字符
    println!("a = {}", a);

    let b = '你';    // 汉字
    println!("b = {}", b);
    
    // 数字类型：i8, i16, i32, i64, u8, u16, u32, u64, f32, f64
    let c: i8 = -111;
    println!("c = {}", c);
    let d: f32 = 0.0009;
    println!("d = {}", d);

    // 自适应类型 isize(有符号), usize(无符号) ：与操作系统相关
    println!("max = {}", usize::max_value());   // Console:" 1844 京 "
    println!("max = {}", isize::max_value());    // Console:" 1844 京 / 2 "

    // 数组
    // [Type; size] , size 也是数组类型的一部分
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    // 通过"[0;N]"形式来初始化初始值为"0"、长度为"N"的数组：
    // let arr = [0;5]; // 自动推导类型
    // 自动推导类型
    let arr1 = [1, 2, 3];
    println!("arr[0] = {}", arr[0]);
    show(arr1);

    // 元组
    let tup: (i32, f32, char) = (-3, 3.69, '好');
    // 自动推导类型
    let tup = (-3, 3.69, '好');
    println!("--------------------");
    // 可用" 元组.索引 "访问
    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);
    println!("--------------------");
    // 解构元组
    let (x, y, z) = tup;
    println!("{}", x);
    println!("{}", y);
    println!("{}", z);
}

fn show(arr:[u32;3]) {
    println!("--------------------");
    /* 使用" for i in arr { //... } "则编译报错：
      error[E0277]: `[u32; 3]` is not an iterator
        for i in arr {
                ^^ borrow the array with `&` or call `.iter()` on it to iterate over it
       = help: the trait `std::iter::Iterator` is not implemented for `[u32; 3]`
       = note: arrays are not iterators, but slices like the following are: `&[1, 2, 3]`
       = note: required by `std::iter::IntoIterator::into_iter`
       // [自] iterator 仅接收" 引用& "类型?
    */
    //for i in arr { //... }
    for i in &arr {     // 或者可为" for i in arr.iter() {  "
        println!("{}", i);
    }
    println!("--------------------");

}
