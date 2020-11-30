
/*
0. for 循环

1. 搜索" for 循环 "
    a.引用 dive
            13.3 内存不安全示例: 迭代器失效(p144~146[?])
                a. 在遍历一个数据结构的过程中修改这个数据结构，会导致迭代器失效
                b. 在 Rust 中 for 循环实质上是生成了一个迭代器，它一直持有一个指向容器的
                   引用，在迭代器的生命周期内，任何对容器的修改都无法编译通过
            3.3.3 for 循环
                a. Rust 中的 for 类比其它语言中的 for-each 循环( Golang 为" for range ")但其没有
                   三段式 for 循环语句(如 Golang 中的" for i:=0;i<len;i++ {} ")如
                       let arr = &[1, 2, 3];
                       for i in arr {
                           println!("The number is {}", i);
                       }
                   // for 循环的主要用途是利用迭代器对包含同类型的多元素容器执行遍历，如数组、链表、
                      HashMap、HashSet 等。在 Rust 中可轻松自定义定制容器和迭代器，因此也很容易使
                      for 循环支持自定义类型
                b. for 循环内部亦可使用 break、continue 控制执行流程
            24.2.3 for 循环(p285[*])
                a. Rust 中更简洁、更自然地使用迭代器的方式是使用 for 循环，本质上而言 for 循环就是专门
                   为迭代器设计的一种语法糖。
                b. for 循环可针对数组切片、字符串、Range、Vec、LinkedList、HashMap、BTreeMap 等所有
                   具有迭代器的类型执行循环，亦可允许针对自定义类型实现循环
                c. Rust 的" for <item> in <container> { <body> } "语法结构就是一个语法糖，其语法原理
                   就是调用 <container>.into_iter() 方法来获得迭代器，然后不断循环调用迭代器的 next()
                   方法将返回值解包赋值给 <item>,然后调用 <body> 语句块
                d. 使用 for 循环时可自主选择三种使用方式
                   // container 在循环后生命周期即结束
                   // (循环过程中每个 item 是从 container 中 move 出来)
                   for item in container{}
                   // 迭代器中只包含 container 的 & 型引用
                   // (循环过程中每个 item 都是 container 中元素的借用)
                   for item in &container{}
                   // 迭代器中包含 container 的 &mut 型引用
                   // (循环过程中每个 item 都是指向 container 中元素的可变借用)
                   for item in &mut container{}
                e. 只要某个类实现了 IntoIterator(扩展：From[源于] / Into[转为])那么调用 into_iter()
                   方法就可得到对应的迭代器，这个 into_iter() 方法的 receiver 是 self (而非 &self)故
                   执行的是 move 语义(此设计可同时支持 Item 类型为" T、&t、&mut T "可供用户选择)。
                f. 如 BTreeMap 容器类型，标准库对其 impl 了三次 IntoIterator，当 Self 类型为 BTreeMap
                   时 Item 类型为 (K,V)，意味着每次 next() 方法都是把内部元素 move 出来；当 Self 类型
                   为 &BTreeMap 时 Item 类型为 (&K,&V)，每次 next() 方法返回的是借用；当 Self 类型为
                   &mut BTreeMap 时 Item 类型为 (&K,&mut V)，每次 next() 方法返回的 key 是只读的而其
                   value 是可读写的。所以若有 BTreeMap 类型的变量 m 则可按需使用" m.into_iter() "或者
                   " (&m).into_iter() "或者" (&mut m).into_iter() "达到不同目的。
                        // trait 声明
                        trait IntoIterator {
                            type Item;
                            type IntoIter: IntoIterator<Item=Self::Item>;
                            fn into_iter(self)->Self::IntoIter;
                        }
                        // trait 实现
                        impl<K,V> IntoIterator for BTreeMap<K,V>{
                            type Item = (K,V);
                            type IntoIter = IntoIter<K,V>;
                        }
                        impl<'a, K:'a,V:'a> IntoIterator for &'a BTreeMap<K,V>{
                            type Item = (&'a K,&'a V);
                            type IntoIter = Iter<'a,K,V>;
                        }
                        impl<'a, K:'a,V:'a> IntoIterator for &'a mut BTreeMap<K,V>{
                            type Item = (&'a K,&'a mut V);
                            type IntoIter = Iter<'a,K,V>;
                        }
                g. Rust 的 IntoIterator trait 实际上就是 for 语法的扩展接口。欲自定义容器亦能在 for
                   循环中使用则可借鉴标准库的写法，自行实现 IntoIterator trait 即可

    b. 引用 tao
            6.3.1 外部迭代器和内部迭代器(p194[*])
                a. 外部迭代器(External Iterator)：亦称主动迭代器(Active Iterator)独立于容器之外，通过
                   容器提供的方法(如"next"[即所谓的"游标"])来迭代下一个元素并需要考虑容器内可迭代的剩余
                   数量来进行迭代。外部迭代器的一个重要特点是"外部可以控制整个遍历进程"(如 Python、Java
                   及 C++ 语言中的迭代器就是外部迭代器)
                b. 内部迭代器(Internal Iterator)：内部迭代器则通过迭代器自身来控制迭代下一个元素，外部
                   无法干预，意味着只要调用了内部迭代器并通过闭包传入相关操作，就必须等待迭代器依次为其
                   内部的每个元素执行完相关操作后方可停止遍历(如 Ruby 语言中典型的内部迭代器 each )
                c. 早期的("1.0"版本之前) Rust 提供的是内部迭代器，而内部迭代器无法通过外部控制迭代进程
                   再加上 Rust 的所有权系统导致使用起来很复杂如
                       trait InIterator<T: Copy> {
                            fn each<F: Fn(T) -> T>(&mut self, f: F);
                        }
                        impl<T: Copy> InIterator<T> for Vec<T> {
                            fn each<F: Fn(T) -> T>(&mut self, f: F) {
                                let mut i = 0;
                                while i < self.len() {
                                    self[i] = f(self[i]);
                                    i += 1;
                                }
                            }
                        }
                        // 使用 Rust 提供的内部迭代器
                        fn main(){
                            let mut v = vec![1,2,3];
                            v.each(|i| i * 3);              // 调用内部迭代器(外部无法控制迭代进程)
                            assert_eq!([3, 6, 9], &v[..3]);
                        }
                d. 外部迭代器 : for 循环( Rust 中的 for 循环其实是一个语法糖)如
                        fn main() {
                            let v = vec![1, 2, 3, 4, 5];
                            for i in v {
                                println!("{}", i);
                            }
                        }
                        // for 循环的等价代码(简言之：for 循环是利用迭代器模式实现的一个语法糖)
                        fn main() {
                            let v = vec![1, 2, 3, 4, 5];
                            {  // 等价于for循环的scope
                                let mut _iterator = v.into_iter();
                                loop {
                                    match _iterator.next() {
                                        Some(i) => {
                                            println!("{}", i);
                                        }
                                       None => break,
                                   }
                               }
                           }
                        }
            6.3.5 消费器 Consumer (p207[*])
                a. Rust 中消费器都是惰性的(即不会自动发生遍历行为[除非调用 next 方法消费其中数据])
                b. 最直接消费迭代器数据的方法就是 for 循环(会隐式调用迭代器的 next 方法)
                c. 常见消费器
                   (1) any
                   (2) fold
                   (3) collect

    c. 引用 base
       使用 for 循环 ：增强代码安全性并消除可能由于下标越界而导致的 bug
       因其 for 循环安全性和简洁性( Rust 最常用）

*/
fn main() {
    // 使用 ref 即" let ref arr = [1, 2, 3]; "等价于" let arr = &[1,2,3]; "
    let ref arr = [1, 2, 3];
    for i in arr {
        println!("{}", i);
    }
    // 数组的引用即是 slice
    let arr = [1, 2, 3];
    for i in &arr {
        println!("{}", i);
    }
    // 使用 iter()
    for i in arr.iter() {
        println!("{}", i);
    }
}
