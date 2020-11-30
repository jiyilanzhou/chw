
/*
1、模式是 Rust 中匹配值结构的特殊语法

2、模式由如下内容组成：
    （1）字面值
    （2）解构的数组、枚举、结构体或元组
    （3）变量
    （4）通配符
    （5）占位符

3、模式不尽相同：模式存在不可反驳的和可反驳的

*/

/*
 1、match 匹配
    match VALUE {
        PATTERN => EXPRESSION,
        PATTERN => EXPRESSION,
        PATTERN => EXPRESSION,
    }
*/
// match 须穷举匹配所有可能情况
fn main_0() {
    let a = 1;
    match a {
        0 => println!("Zero"),
        1 => println!("One"),
        _ => println!("other"),
    };
    println!("Hello, world!");
}

// 2. if let : 条件匹配
fn main_1() {
    let color: Option<&str> = None;
    let is_ok = true;
    let age: Result<u8, _> = "33".parse();

    // " if let ... else if "组合
    if let Some(c) = color {
        println!("color: {}", c);
    } else if is_ok {
        println!("is ok");
    } else if let Ok(a) = age {
        if a > 30 {
            println!("oh, mature man");
        } else {
            println!("oh, young man");
        }
    } else {
        println!("in else");
    }
}

//3. while let : 循环匹配
fn main_2() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    // 只要模式匹配 Some(value) 则一直循环
    while let Some(top) = stack.pop() {
        println!("top = {}", top);
    }
}

// 4. for 循环:其模式紧跟随 for 关键字(如" for x in y "其" x "即是对应模式)
fn main_3() {
    let v = vec!['a', 'b', 'c'];
    // 匹配模式： (index, value)
    for (index, value) in v.iter().enumerate() {
        println!("index: {}, value: {}", index, value);
    }
}

// 5. let 语句: " let PATTERN = EXPRESSION "
fn main_4() {
    // (1, 2, 3) 会匹配(x, y, z) : 将 1，2，3 分别绑定 x，y，z
    let (x, y, z) = (1, 2, 3);
    println!("{}, {}, {}", x, y, z);
    // 忽略
    let (x, .., z) = (1, 2, 3);
    println!("{}, {}", x, z);
}

// 6. 函数参数也是模式
fn print_point(&(x, y): &(i32, i32)) { // 参数为" 元组 "
    println!("x: {}, y: {}", x, y);
}
fn main() {
    let p = (3, 5);
    // &(3, 5) 匹配模式 &(x, y)
    print_point(&p);
}