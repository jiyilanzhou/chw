
/*
0. 集合
    可自动扩容(存储于堆上)

1. 集合初始化方式
    a. 使用宏静态初始化： vec![elem0,elem1...]
    b. 动态初始化：Vec::new()

*/

fn main() {
    // 静态初始化
    let v0 = vec![1,2,3];
    // 动态初始化
    let mut v = Vec::new();
    // 范围遍历
    for i in 0..v.len(){
        print!("{}\t",v[i]);
    }
    v.push(1);
    v.push(2);
    v.push(3);
    // 迭代器遍历
    //for i in v.into_iter(){
    // 如何知道 &v 调用的是哪个方法[???]
    //for i in v{
    //for i in &v{
    for i in v.iter(){
        print!("{}\t",i);
    }
    // 修改
    // for i in &mut v{
    for i in v.iter_mut(){
        *i+=10
    }
    println!("{:?}",v);
}