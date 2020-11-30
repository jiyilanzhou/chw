
// deque[dek]n.双端队列
// peek[piːk]v&n.窥视，窥探          // peer[pɪə(r)]n.同级,对等
/*
2.8 常用集合类型(序列、映射表[集合]、队列)
    Rust 标准库 std::collections 模块提供 4 种通用集合类型
    a. 线性序列：向量(Vec)、双端队列(VecDeque)、链表(LinkedList)
    b. Key-Value 映射表：无序哈希表(HashMap)、有序映射表(BTreeMap)
    c. 集合类型：无序集合(HashSet)、有序集合(BTreeSet)
    d. 优先队列：二叉堆(BinaryHeap)

2.8.1 线性序列：向量
    向量亦是一种数组(但区别于基本数据类型中的数组)：向量可动态增长

*/
// (代码清单 2-44 ) Vec<T> 示例
/* 三种初始化向量(参见"v1、v2、v3"初始化方法)：
   a. 向量用法与一般数组类似，变量通过索引访问元素但(若往向量中添加元素则需 mut 创建可变绑定)
      " vec! "宏可用于创建向量字面量。宏语句可用圆括号"()"、中括号"[]"、花括号"{}"，一般使用
      中括号"[]"来表示数组。可用 push 方法往向量数数组中追加新元素(向量亦内置许多方法)。
   b. Rust 向量及数组皆会进行越界检查以保证安全

*/
fn main_006_000() {
    let mut v1 = vec![];
    v1.push(1);  // 使用 push 方法追加元素
    v1.push(2);
    v1.push(3);
    assert_eq!(v1, [1, 2, 3]);
    assert_eq!(v1[1], 2);
    let mut v2 = vec![0; 10]; // 类似数组初始化
    let mut v3 = Vec::new();
    v3.push(4);
    v3.push(5);
    v3.push(6);
    // v3[4] = 8; // 编译错误:"'index out of bounds: the len is 3 but the index is 4',"
}

/*
2.8.2 线性序列：双端队列     // deque[dek]n.双端队列
    a. 双端队列(Double-ended Queue，缩写为"Deque")是一种同时具有队列(先进先出)和栈(先进后出)
       性质的数据结构。双端队列中的元素可从两端弹出。"插入及删除"操作被限定于队列两端进行。
    b. Rust 中的 VecDeque 是基于可增长的 RingBuffer 算法实现的双端队列
    c. 双端队列实现两种 push 方法：push_front (类比栈)及 push_back (类比队列)。通过 get 方法
       并传入索引可获取队列中相应值

*/
// (代码清单 2-45 ) VecDeque<T> 示例
// 引用作用域(对比"Vec<T> : 自动引入")
use std::collections::VecDeque;
fn main_006_001() {
    let mut buf = VecDeque::new();
    // 通过 push_front 先后添加元素"1、2"(其索引分别对应"1、0"[即栈数据结构体现])
    buf.push_front(1);
    buf.push_front(2);
    assert_eq!(buf.get(0), Some(&2));
    assert_eq!(buf.get(1), Some(&1));
    // 通过 push_back 先后添加元素"3、4"(其索引分别对应"2、3"[即队列数据结构体现])
    buf.push_back(3);
    buf.push_back(4);
    // 通过 get 方法并传入索引可获取队列中相应值
    assert_eq!(buf.get(2),Some(&3));
    assert_eq!(buf.get(3),Some(&4));
}

/*
2.8.3 线性序列：链表
   a. " Rust "提供的链表是双向链表，允许在任意一端插入或弹出元素(但一般推荐最好使用 Vec
      或 VecDeque 类型[因其比链表访问更快、效率更高及能更好利用 CPU 缓存])
   b. 因 LinkedList 为双向链表故分别提供 push_back/push_front 及 pop_back/pop_front
      操作链表。同时亦提供 append 方法连接两个链表

*/
// (代码清单 2-46 ) LinkedList<T> 示例
// 使用 use 显式引用作用域
use std::collections::LinkedList;
fn main_006_002() {
    // 显式声明元素类型
    //let mut list1:LinkedList<char> = LinkedList::new();
    // 未指明元素类型则由首添加的元素自动推导
    let mut list1 = LinkedList::new();
    list1.push_back('a');
    let mut list2 = LinkedList::new();
    list2.push_back('b');
    list2.push_back('c');
    list1.append(&mut list2);
    println!("{:?}",list1);     // Console:" ['a', 'b', 'c'] "
    println!("{:?}",list2);     // Console:" [] "
    list1.pop_front();
    println!("{:?}",list1);     // Console:" ['b', 'c'] "
    list1.push_front('e');
    println!("{:?}",list1);     // Console:" ['e', 'b', 'c'] "
    list2.push_front('f');
    list2.push_front('g');
    list2.pop_back();
    println!("{:?}",list2);     // Console:" ['g'] "
}

/*
2.8.4 key-Value映射表：HashMap 和 BTreeMap
    Rust 集合模块提供两个 Key-Value 哈希映射表，其签名如下：
        HashMap<K,V> ：无序(因 Key 顺序随机)
        BTreeMap<K,V> ：有序
    其中 HashMap<K,V>要求 Key 必须是可哈希的类型，而 BTreeMap 的 Key 必须可排序。
    另外 Value 必须是在编译期已知大小的类型

*/
// (代码清单 2-47 ) HashMap<K,V> 和 BTreeMap<K,V> 示例
use std::collections::{HashMap, BTreeMap};  // 引入作用域
fn main_006_003() {
    // HashMap<K,V> ：无序
    let mut hmap = HashMap::new();
    hmap.insert(1,"a");
    hmap.insert(2,"b");
    hmap.insert(3,"c");
    // Console:"{3: "c", 1: "a", 2: "b"}"
    println!("{:?}",hmap);      // 无序：每次执行效果不一
    // BTreeMap<K,V> : 有序
    let mut bmap = BTreeMap::new();
    bmap.insert(1,"a");
    bmap.insert(2,"b");
    bmap.insert(3,"c");
    // Console:"{1: "a", 2: "b", 3: "c"}"
    println!("{:?}",bmap);       // 有序：多次执行效果一致
}

/*
2.8.5 集合：HashSet 和 BTreeSet
    HashSet<K> 和 BTreeSet<K> 其实就是 HashMap<K,V>、BTreeMap<K,V> 将 Value 设置
    为空的特定类型，等价于" HashMap<K,()>、BTreeMap<K,()> "故其特性如下：
        a. 集合中元素唯一：因其为 Key-Value 映射表的 Key
        b. (同理) HashSet 中的元素应该都是可哈希的类型而 BTreeSet 中的元素须可排序
        c. HashSet 应无序而 BTreeSet 应有序

*/
// (代码清单 2-48 ) HashSet<K> 和 BTreeSet<T> 示例
use std::collections::{HashSet, BTreeSet};
fn main_006_004() {
    // HashSet : 无序(多次输出不一致)
    let mut hbooks = HashSet::new();
    hbooks.insert("A Song or Ice and Fire");
    hbooks.insert("The Emerald City");
    hbooks.insert("The Odyssey");
    if !hbooks.contains("The Emerald City") {
        println!("We have {} books,but The Emerald City ain't one.", hbooks.len());
    }
    println!("{:?}", hbooks);   // 无序(每输出顺序随机[因 HashSet 无序])
    // BTreeSet : 有序(多次输出一致)
    let mut bbooks = BTreeSet::new();
    bbooks.insert("A Song or Ice and Fire");
    bbooks.insert("The Emerald City");
    bbooks.insert("The Odyssey");
    println!("{:?}",bbooks);    // 有序(多次输出顺序一致[因 BTreeSet 有序])
}

/*  // peek[piːk]v&n.窥视，窥探          // peer[pɪə(r)]n.同级,对等
2.8.6 优先队列
    Rust 提供的优先队列是基于二叉最大堆(Binary Heap)实现

*/
// (代码清单 2-49 ) BinaryHeap<T> 示例
use std::collections::BinaryHeap;
fn main_006_005() {
    // 使用 BinaryHeap::new 创建了空的最大堆
    let mut heap = BinaryHeap::new();
    // 使用 peek 方法读取堆中最大值(此处因堆中无任何值故 peek 方法取出的是 None)
    assert_eq!(heap.peek(), None);
    let arr = [80, 93, 48, 53, 20, 18, 36, 15, 35, 45];
    // 迭代将数组中的元素依次投入堆中
    for &i in arr.iter() {
        heap.push(i);
    }
    // peek 读取堆中最大元素
    assert_eq!(heap.peek(), Some(&93));
    println!("{:?}", heap.peek());  // Console:" Some(93) "
}