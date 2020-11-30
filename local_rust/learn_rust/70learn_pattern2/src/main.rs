
//1、匹配字面值
fn main_0() {
    let x = 1;
    match x {   // 穷尽匹配
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("xx"),
    };
}

//2、匹配命名变量
fn main_1() {
    let x = Some(5);
    let y = 10; // 位置 1
    match x {
        Some(50) => println!("50"),
        /* 此处 y 为成功匹配后绑定的对象(相当于重新定义
           " 位置 1 "处的 y ),其后表达式使用的 y 则遵循
           就近原则。即如果:
               Some(z) => println!("value = {}", y),
           则输出" value = 10 "(此情况 y 为位置 1 处的 y )
        */
        Some(y) => println!("value = {}", y),
        _ => println!("other"),
    };
    // 此处 y 是" 位置 1 "的 y
    println!("x = {:?}, y = {:?}", x, y);
}

//3、多个模式
fn main_2() {
    let x = 1;
    match x {
        // " | "表示" 或 "(匹配 1 或 2 )
        1 | 2 => println!("1 or 2"),
        3 => println!("3"),
        _ => println!("xx"),
    };
}

//4、通过" .. "匹配
fn main_() {
    let x = 5;
    match x {
        // 1 | 2 | 3 | 4 | 5 => println!("1 to 5"),
        1..=5 => println!("1 to 5"),
        _ => println!("xx"),
    };

    let x = 'c';
    match x {
        'a'..='j' => println!("1"),
        'k'..='z' => println!("2"),
        _ => println!("other"),
    }
}