
/*
匹配守卫 ：提供额外条件
    匹配守卫是指定于 match 分支模式后的额外 if 条件
    (必须满足 if 条件才能选择此分支）
*/

fn main_0() {
    let num = Some(4);
    match num {
        // 使用匹配守卫
        Some(x) if x < 5 => println!("< 5"),
        Some(x) => println!("x: {}", x),
        None => (), // 须穷尽枚举
    };

    println!("Hello, world!");
}

fn main() {
    let num = Some(4);
    let y = 10; // 位置 1
    match num {
        // 此处 y 是" 位置1 "处的 y
        Some(x) if x == y => println!("num == y"),
        Some(x) => println!("x: {}", x),
        None => (),
    };

    let x = 4;
    let y = false;
    match x {
        // " 4|5|6 if y "意为" (4|5|6) if y "
        4|5|6 if y => println!("1"),
        _ => println!("2"),
    }

}
