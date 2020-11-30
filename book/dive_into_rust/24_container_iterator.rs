
// container[kənˈteɪnə(r)]n.容器(集装箱)
/*
0. 容器与迭代器

24.1 容器
     (0) Vec
     (1) VecDeque
     (2) LinkedList
     (3) HashMap
     (4) BTeeMap
     (5) HashSet
     (6) BTeeSet
     (7) BinaryHeap

24.1.1 Vec (p275)

24.1.2 VecDeque (p277)

24.1.3 HashMap (p277)

24.1.4 BTreeMap (p281)

24.2 迭代器(p283)
    Rust 中的迭代器是指实现了 Iterator trait 的类型

24.2.1 实现迭代器

24.2.2 迭代器的组合

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
                fn into_iter(self) -> Self::IntoIter {
                    unimplemented!()
                }
            }
            impl<'a, K:'a,V:'a> IntoIterator for &'a BTreeMap<K,V>{
                type Item = (&'a K,&'a V);
                type IntoIter = Iter<'a,K,V>;
                fn into_iter(self) -> Self::IntoIter {
                    unimplemented!()
                }
            }
            impl<'a, K:'a,V:'a> IntoIterator for &'a mut BTreeMap<K,V>{
                type Item = (&'a K,&'a mut V);
                type IntoIter = Iter<'a,K,V>;
                fn into_iter(self) -> Self::IntoIter {
                    unimplemented!()
                }
            }
    g. Rust 的 IntoIterator trait 实际上就是 for 语法的扩展接口。欲自定义容器亦能在 for
       循环中使用则可借鉴标准库的写法，自行实现 IntoIterator trait 即可

*/
