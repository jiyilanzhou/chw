
/*
0. str
    &str 并非整体("&"代表取地址而 str 是 Rust 中不可变的静态字符串)
    故 &str 是获取 str 的地址(即指向 str 的引用[" & "为取地址符])

1. 语句与表达式
    表达式有返回值如" if a==b {} "则" a==b "则为表达式(返回 bool)

 */
fn main_0() {
    // 将数据写入 str 并对其创建引用 & 赋给 variable (禁止操作 str)
    let variable = "chw";
    // 完整写法
    let variable: &'static str = "chw";
    println!("{}", func(0));
}

/* 编译报错： // 返回编译器不能推导生命周期的引用须标注 lifetime
    error[E0106]: missing lifetime specifier
       | fn func()->&str{
       |            ^ expected named lifetime parameter
       = help: this function's return type contains a borrowed value,
                    but there is no value for it to be borrowed from
    help: consider using the `'static` lifetime
*/
// fn func()->&str{ "Rust" }

/* 返回编译器不能推导生命周期的引用须标注 lifetime (但与返回引用值无关的
   形参则未强制要求标注生命周期)
   此例看似只有一个参数故应推断其生命周期应为返回引用值的生命周期，但仔细
   分析却知单个形参与返回的引用非同类型[此例无关联]
   若为" fn func(s: &str) -> &str { "Rust" } "则可自动推导
 */
fn func(u: u8) -> &'static str {
    println!("{}", u);
    "Rust"
}

/*
2. &str 操作
    a. len()
    b. chars()

3. match
    match expr {}
    // match 后跟表达式故不可写为" match expr; {} "

4. 范围表达式
    " .. " 或 " ..= "

 */
fn main_1() {
    let var_str = "Rust静心道";
    println!("{}", var_str.len()); // Console:" 13 "
    // 遍历字符
    for c in var_str.chars() {
        print!("{}\t", c);
    }
    // match
    match var_str.len() {
        /* [自] match 为穷尽枚举
               故不允许存在开区间(即模糊边界)匹配" 0..3 "(如" 2.999...")
         */
        0..=3 => println!("too short!"),
        3..=10 => println!("bingo!"),
        _ => println!("too long!")
    }
    // 范围表达式(允许使用" 0..6 "或" 0..=6 "范围表达式 )
    for i in 0..6 {
        if i == 3 {
            println!("{}", i);
        }
    }
}

/*
5. 字符串 String 拼接
    a. Add ：加法运算符
    b. format!
    c. push / push_str
    d. len / capacity
       长度容量存于栈空间，数据存于堆空间
    e. as_ptr 转为指针(即内存地址)

*/
fn main() {
    // 声明
    let mut first_name = String::new();
    let last_name: String = String::from("");
    //字符串拼接方式 1: Add // " &String 自动转换为 &str "
    //let mut name = first_name + &last_name + "_";
    // 字符串拼接方式 2: 使用 format!
    let mut name = format!("{}{}", first_name, last_name);
    // 追加字符
    name.push('静');
    //字符串拼接方式 3: push_str   //  追加字符串
    name.push_str("心道");
    println!("{}", name);

    /* 输出地址 :
        Rust 语言需要使用 as_ptr 输出内存地址(不可直接使用 &variable 打印)
        (一般其它编程语言皆可使用 & [如 Go:"fmt.printf("%p",&variable)"])
     */
    let name = "chw".to_string();
    println!("ptr is {:?}",name.as_ptr());  // Console：" 0x854b0 "

}