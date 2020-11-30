
fn main() {
    //1、定义结构体
    #[derive(Debug)]     // " {:?}/{:#?} "格式打印结构体
    struct User {
        name: String,    // 结构体首字母一般大写如"String"
        count: String,   // 源码:"pub struct String {vec: Vec<u8>,}"
        nonce: u64,      // 基础数据类型首字母小写如"u64、bool"
        active: bool,    // 最末尾成员后成员分隔符可省
    }

    //2、创建结构体实例
    let xiaoming = User {
        name: String::from("xiaoming"),
        count: String::from("80001000"),
        nonce: 10000,
        active:true,
    };
    // 打印结构体
    println!("xiaoming = {:?}", xiaoming);
    println!("xiaoming = {:#?}", xiaoming);

    //3、修改结构体字段
    let mut xiaohuang = User {
        name: String::from("xiaohuang"),
        count: String::from("80001000"),
        nonce: 10000,
        active:true,
    };
    xiaohuang.nonce = 20000;

    //4、参数名字和字段名字同名的简写方法
    let name = String::from("xiaoxiao");
    let count = String::from("89077777");
    let nonce = 200000;
    let active = false;

    /*
    let user1 = User {
        name: name,
        count: count,
        nonce: nonce,
        active: active,
    };
    */
    let user1 = User {
        name,
        count,
        nonce,
        active,
    };

    //5、从其它结构体创建实例
    let user2 = User {
        name: String::from("user2"),
        // count:user1.count,   // 传统书写方法
        ..user1
    };
    println!("name = {}", user2.name);
    println!("nonce = {}", user2.nonce);

    //6、元组结构体
    //(1)字段没有名字
    //(2)圆括号
    struct Point(i32, i32);

    let a = Point(10, 20);
    let b = Point(30, 11);
    println!("a.x = {}, a.y = {}", a.0, a.1);

    //7、没有任何字段的类单元结构体
    struct A{};     // 可省略其后的" {} "即" struct A; "

    //8、打印结构体(在结构体上标识" #[derive(Debug)] ")
    println!("xiaoming = {:?}", xiaoming);
    println!("xiaoming = {:#?}", xiaoming);

}