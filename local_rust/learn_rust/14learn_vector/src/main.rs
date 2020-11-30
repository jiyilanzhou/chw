
/*
1、创建空的 vector: Vec<T>
2、创建包含初始值的 vector
3、丢弃 vector
4、读取元素
5、更新
6、遍历
7、使用枚举
*/

fn main() {
    //1、创建可变的空 vector: Vec<T>
    //let v: Vec<i32> = Vec::new(); // 不可变的空 Vector 无多大意义(因不可写)
    let mut v: Vec<i32> = Vec::new();
    //v.push(1);    // 修改须先声明为" mut "

    //2、创建包含初始值的 vector
    let v = vec![1, 2, 3];

    //3、丢弃 vector
    {
        let v1 = vec![1, 2, 3];
    }

    //4、读取元素
    // 方式1 ： v[idx]       // 不推荐(因可能出现数组下标越界)
    let one: &i32 = &v[0];
    //let four: &i32 = &v[3];
    println!("one = {}", one);
    println!("one = {}", *one);
    // 方式2 : get(idx)      // 推荐(因不会出现下标越界)
    //match v.get(1) {
    match v.get(3) {
        Some(value) => println!("value = {}", value),
        _ => println!("None"),
    }

    //5、更新
    let mut v2: Vec<i32> = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);

    //6、遍历
    //(1)不可变的遍历
    for i in &v2 {  //若为" for i in v2 { "则会获取 v2 所有权其后 v2 不同可用
        println!("i = {}", i);
    }
    //(2)可变的遍历
    for i in &mut v2 {
        *i += 1;        // 通过指针操作原数组(虽为拷贝份但其解引用后仍为原数据)
    }
    println!("可变遍历后:{}",v2[0]);

    //7、使用枚举：将不同类型置入同一 Vector
    enum Context {
        Text(String),
        Float(f32),
        Int(i32),
    };
    let c = vec![
        Context::Text(String::from("string")),
        Context::Int(-1),
        Context::Float(0.001)
    ];

    //8、补充
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
    // 不能在"可变引用后"再使用"不可变引用"(因可变引用可能已修改"原不可变引用指向数据"的地址)
    println!("first = {}", first);
    /* 编译报错:
        error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
           let first = &v[0];
                        - immutable borrow occurs here
           v.push(6);
           ^^^^^^^^^ mutable borrow occurs here
           println!("first = {}", first);
                                  ----- immutable borrow later used here
    */

}
