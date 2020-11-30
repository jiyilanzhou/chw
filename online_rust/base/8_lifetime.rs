
/*
0. Lifetime 生命周期
   引用 & 才会涉及生命周期

*/
// 函数标注生命周期:最大值
fn max<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y {
        x
    } else {
        y
    }
}

// 结构体生命周期
struct Car0<'a> {
    name: &'a String
}
struct Car1<'a> {
    name: &'a str
}

fn main() {
    // 函数生命周期
    let a = 3;
    let b = 6;
    println!("max: {}", max(&a, &b));
    // 结构体(一般类型[如"i8、u8 ... String"]引用[但不包括" &str "])
    let mut name = "chw".to_string();
    // " name:&name "包括" & "故为引用
    let car0 = Car0{ name:&name };  // 车被借走
    //name = "683".to_string();    // 车归还前不能对其操作
    println!("{}", car0.name);     // 有使用则说明车未归还

    // 结构体(" &str "：为静态生命周期 static [保存于"二进制"文件中])
    let mut name = "chw";
    let car1 = Car1{ name:name };  // 车被借走
    /* 可理解为" name "未取地址" & "，故 name 不为引用因而不受引用限制(将
       " &str "看作一个整体类型)；亦可理解为" 对 str 的引用即 &str 存活于
       整个生命周期，且 str 固定不可改(其" name = xx "充其量只是更改 name
       的指向而之前赋值于结构体实例内的 name 指向的数据不曾也不会改变"),故
       针对 &str 此类用法有效(而指向堆空间普通数据的变量可修改其所指向的值
       [这才是赋值于引用后未使用前不可修改的根本原因])。另外" str "保存于
       " 二进制 "文件，只能以借用 & 方式使用(贯穿于整个程序)。总而言之将
       " &str "当作一个类型来看待则与普通类型用法无异。
     */
    name = "xx";    // 车归还前亦可对其操作
    println!("{}", car0.name);     // 有使用则说明车未归还

}
