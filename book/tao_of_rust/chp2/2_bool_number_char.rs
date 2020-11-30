
/*
        基本数据类型

2.6.1 布尔类型
    可以使用 as 操作符将 bool 类型转换为"数字"(但 Rust 不支持数字转布尔类型)

*/
fn bool_type() {
    let x = true;
    let y = x as i32;
    println!("{}",y);   // Console:" 1 "
    //  Rust 不支持数字转布尔类型
    let z = y as bool;
    /* 编译报错:
       error[E0054]: cannot cast as `bool`
            let z = y as bool;
            ^^^^^^^^^ help: compare with zero instead: `y != 0`
    */
}

/*
2.6.2 基本数字类型(参阅" p26 ~ p27 ")
    Rust 提供的基本数字类型大致分为三类：固定取值范围类型、动态取值范围类型及浮点数类型
    a. 固定取值范围类型：包括符号整型(Signed Integer)及无符号整型(Unsigned Integer)
       (1). i8/u8、i16/u16、i32/u32、i64/u64、i128/u128
       (2). 无符号整型 u8 其数值范围为" 0 ~ 2^8-1 (即"255")"占用 1 个字节。u8 类型通常
            在 Rust 中表示字节序列，在文件 I/O 或网络 I/O 中读取数据流时需要使用" u8 "
    b. 动态取值范围类型
        (1). usize 数值范围为" 0 ~ 2^32-1 或 2^64-1 "(占用 4 或 8 个字节[具体取决于机器的字长])
        (2). isize 数值范围为" -2^31 ~ 2^31-1 或 -2^63-1 ~ 2^63-1 "(同理占用 4 或 8 个字节[亦
             取决于机器的字长])
    c. 浮点数类型
        (1). f32 单精度 32 位浮点数，至少 6 位有效数字，数值范围为" -3.4*10^38 ~ 3.4*10^38-1 "
        (2). f64 双精度 64 位浮点数，至少 15 位有效数字，数值范围为" -1.8*10^308 ~ 1.8*10^308-1 "
    d. 代码清单

*/
// infinity[ɪnˈfɪnəti]n.无穷(大)   // negative[ˈneɡətɪv]adj&n.负数(负的)，否定/消极
fn number_type() {
    /* 类型后缀：
       数字字面量("整型或浮点型"数值):其后可直接使用"类型后缀"指定类型
       否则 Rust 编译器会默认推断为" i32 类型(整型)或 f64 类型(浮点数)"
    */
    let num = 42u32;
    /* 类型前缀：
       可用前缀"0x、0o 和 0b"分别表示"十六进制、八进制及二进制"
    */
    // 十六进制
    let num = 0x2A;
    // 八进制
    let num = 0o106;
    // 二进制
    let num = 0b11011_1101;
    // 字节字面量
    //let b_str = b'静';//编译错误( error: byte constant must be ASCII... )
    assert_eq!(b'*', 42u8);
    let b_str = b"chw";
    println!("{:?}",b_str);
    // 浮点数
    let num = 3.1415926f64;
    assert_eq!(2., 2.0f64);
    assert_eq!(2e4, 20000f64);  // 即" 2e4 =  2*10^4 "
    /* 标准库" std::f32 "及" std::f64 "均提供了" IEEE "所需的特殊常量值：
       比如" INFINITY(无穷大)、NEG_INFINITY(负无穷大)、NAN(非数字值)、
       MIN(最小有限值)及 MAX(最大有限值) "等
    */
    println!("{:?}",std::f32::INFINITY);        // Console:" inf "
    println!("{:?}",std::f32::NEG_INFINITY);    // Console:" -inf "
    println!("{:?}",std::f32::NAN);             // Console:" NaN "
    // f32 范围" -3.4*10^38 ~ 3.4*10^38-1 "
    println!("{:?}",std::f32::MIN); // Console:"-3402823500"N个0"00.0"
    println!("{:?}",std::f32::MAX); // Console:"3402823500"N个0"00.0"
    // f64 范围" -1.8*10^308 ~ 1.8*10^308-1 "
    println!("{:?}",std::f64::MIN); // Console:"-1797693134862315700"N个0"00.0"
    println!("{:?}",std::f64::MAX); // Console:"1797693134862315700"N个0"00.0"
}

/*
2.6.3 字符类型
    Rust 中使用单引号 '' 来定义字符(char)类型。字符类型代表一个 Unicode 标量值，每个
    字符占 4 个字节(区别于其它语言[一般占用 1 个字节])故可存储" 1 "个汉字
*/
pub fn char_type(){
    // 使用 Unicode 值来定义字符(如 'Ú'、'ಠ'、'*' 等)
    let x = 'r';
    let x = 'Ú';
    // Rust 支持转义字符" \ "
    println!("{}", '\'');
    println!("{}", '\\');
    println!("{}", '\n');
    println!("{}", '\r');
    println!("{}", '\t');
    /* 字符可用 ASCII 码和 Unicode 码来定义:
       a. ASCII 码十六进制数格式: '\xHH'  如 '\x2A'、'\x25'
       b. Unicode 码十六进制数格式: 'u\{HHH}'  如 'u\{CAO}'、'\u{151}'
       问题：ASCII 码及 Unicode 码其它进制数格式表示
    */
    assert_eq!('\x2A', '*');
    assert_eq!('\x25', '%');
    assert_eq!('\u{CA0}', 'ಠ');
    assert_eq!('\u{151}', 'ő');
    /* 使用 as 操作符可将字符转为数字类型如:
       '%' 的十进制 ASCII 值为 37
       'ಠ' 转换为 i8 (该字符的高位会被截断)
    */
    assert_eq!('%' as i8, 37);
    assert_eq!('ಠ' as i8, -96); //该字符值的高位会被截断最终得到-96
    // 每个字符占 4 个字节(区别于其它语言[一般占用 1 个字节])故可存储" 1 "个汉字
    let chinese = '道';
    println!("{}",chinese);
}