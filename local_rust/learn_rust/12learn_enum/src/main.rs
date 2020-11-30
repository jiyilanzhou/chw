
//1、类似于 c 语言的方式定义
enum IpAddrKind {
    V4,
    V6,
}

// IpAddr 结构体
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

//2、rust 语言提倡的方式定义
enum IpAddr2 {
    V4(String),
    V6(String),
}

//3、可以是不同类型
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

//4、经典用法
enum Message {
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    Change(i32, i32, i32),
    /* 等同于：
        struct QuitMessage;     // 类单元结构体
        struct MoveMessage {    // 具名结构体
          x: i32,
          y: i32,
        }
        struct WriteMessage(String)     // New Type
        struct Change(i32, i32, i32)    // 元组结构体
    */
}

//5、枚举类型的方法以及 match
impl Message {
    fn print_custom(&self) {
        // " *self "为"解引用"
        match *self {
            Message::Quit => println!("Quit"),
            Message::Move{x, y} => println!("Move x = {}, y = {}", x, y),
            Message::Change(a, b, c) => println!("Change a = {}, b = {}, c = {}", a, b, c),
            /* 编译错误:
                error[E0507]: cannot move out of `self.0` which is behind a shared reference
                     match *self {
                           ^^^^^ help: consider borrowing here: `&*self`
                         Message::Write(s) => println!("Write = {}", s)
                                        -
                                        |
                                        data moved here
                                        move occurs because `s` has type `std::string::String`,
                                        which does not implement the `Copy` trait
                // 解决方案:据提示将" match "后的" *self "修改为" &*self "
            */
            // Message::Write(s) => println!("Write = {}", s)
            _ => println!("Write")
        }
    }
}

fn main() {
    // 类 C 定义
    let i1 = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let i2 = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    
    // Rust 提倡的定义
    let i1 = IpAddr2::V4(String::from("127.0.0.1"));
    let i2 = IpAddr2::V6(String::from("::1"));

    let i1 = IpAddr3::V4(127, 0, 0, 1);
    let i2 = IpAddr3::V6(String::from("::1"));

    //4、经典用法 
    let quit = Message::Quit;
    quit.print_custom();
    let mo = Message::Move{x: 10, y: 20};
    mo.print_custom();
    let write = Message::Write(String::from("Hello"));
    write.print_custom();
    let change = Message::Change(1, 2, 3);
    change.print_custom();
    
}
