
// 枚举
enum Message {
    Quit,                   // (类比)单元结构体" Empty{} 或 Empty "
    Move{x: i32, y: i32},   // (类比)普通结构体
    Write(String),          // (类比)包装类(元组结构体)
    ChangeColor(i32, i32, i32), // (类比)元组结构体
}

fn main() {
    // 定义枚举
    let msg = Message::ChangeColor(0, 160, 255);
    // 解构枚举
    match msg {
        Message::Quit => {
            println!("quit");
        },
        Message::Move{x, y} => {
            println!("move, x: {}, y: {}", x, y);
        },
        Message::Write(text) => println!("write msg: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("color, r: {}, g: {}, b: {}", r, g, b);
        },
    };
    println!("Hello, world!");

}