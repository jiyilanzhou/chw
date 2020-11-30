
// 结构体
struct Point{
    x: i32, 
    y: i32,
}

fn main() {
    // 解构" 元组、结构体 "组合
    let ((a, b), Point{x, y}) = ((1, 2), Point{x: 3, y: 4});
    println!("a: {}, b: {}, x: {}, y: {}", a, b, x, y);

}