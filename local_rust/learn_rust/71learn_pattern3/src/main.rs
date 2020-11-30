
/* 解构并分解值 :
     解构" 元祖、结构体、枚举、引用 "

*/

// 解构结构体
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point{x: 1, y: 2};
    //变量" a, b "匹配" x, y "
    let Point{x: a, y: b} = p;
    assert_eq!(1, a);
    assert_eq!(2, b);

    let Point{x, y} = p;
    assert_eq!(1, x);
    assert_eq!(2, y);

    let p = Point{x: 1, y: 0};
    match p {
        // 部分解构：匹配并绑定 x (" y "已自定义绑定为" 0 ")
        Point{x, y: 0} => println!("x axis"),   // X 轴上点
        // 部分解构：匹配并绑定 y (" x "已自定义绑定为" 0 ")
        Point{x: 0, y} => println!("y axis"),   // Y 轴上点
        // 全部解构：匹配并绑定 x , y
        Point{x, y} => println!("other"),
    };

}