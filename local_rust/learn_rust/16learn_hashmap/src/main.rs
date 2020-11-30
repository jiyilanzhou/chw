
/*
1、HashMap<K, V>
2、创建 HashMap
3、读取
4、遍历
5、更新
*/

// 引入
use std::collections::HashMap;

fn main() {
    // 创建空的可变 HashMap (同理创建不可变空容器没有多大意义)
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);

    let keys = vec![String::from("Blue"), String::from("Red")];
    let values = vec![10, 20];
    let scores: HashMap<_, _> = keys.iter().zip(values.iter()).collect();

    // 使用 if let
    let k = String::from("Blue");
    if let Some(v) = scores.get(&k) { // get 返回的是 Option
        println!("v = {}", v);
    }
    // 使用 match
    let k = String::from("Yellow");
    let v = scores.get(&k); // get 返回 Option
    match v {
        Some(value) => println!("v = {}", value),
        None => println!("None"),
    }
    println!("++++++++++++");
    // 遍历 : 每遍历顺序可能不一致(因 Map 无序)
    // 同理若为" for (key, value) in scores { "则为获取所有权，则其后不可再使用
    for (key, value) in &scores {
        println!("{}, {}", key, value);
    }
    println!("++++++++++++");

    // 直接插入
    let mut ss = HashMap::new();
    ss.insert(String::from("one"), 1);
    ss.insert(String::from("two"), 2);
    ss.insert(String::from("three"), 3);
    ss.insert(String::from("one"), 3);  // 覆盖
    println!("{:?}", ss);

    // 键不存在的时候才插入
    let mut ss1 = HashMap::new();
    ss1.insert(String::from("one"), 1);
    ss1.insert(String::from("two"), 2);
    ss1.insert(String::from("three"), 3);
    ss1.entry(String::from("one")).or_insert(3);
    println!("ss1 = {:?}", ss1);

    // 根据旧值来更新一个值(统计单词出现次数)
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // map 中不存在 word 时才插入" word : 0 "键值对
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("map = {:?}", map);

}
