
/*
iter

*/

fn first_word_0(s: &String) -> &usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_1(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

/*
0. 搜索" iter().enumerate "
    a. 引用 base
            fn first_word_0(s: &String) -> usize {
                // 用 as_bytes 方法将 String 转化为字节数组
                let bytes = s.as_bytes();
                /* iter 方法返回集合中的每一个元素，而 enumerate 包装了 iter 的结果，
                   将这些元素作为元组的一部分来返回。enumerate 返回的元组中，第一个元
                   素是索引，第二个元素是集合中元素的引用
                */
                for (i, &item) in bytes.iter().enumerate() {
                    /*enumerate 方法返回一个元组，可用模式解构(故在for循环中指定模式，
                     其中元组中的i是索引而元组中的 &item 是单个字节[因从.iter().enumerate()
                     中获取集合元素的引用故模式中使用 &])
                    */
                    // 通过字节的字面值语法来寻找代表空格的字节
                    // (找到空格则返回其位置否则返回用s.len()返回字符串的长度)
                    if item == b' ' {
                        return i;   // 返回单词结尾的索引
                    }
                }
                s.len()
            }
            fn main_005_000() {
                let mut s = String::from("hello world");
                let word = first_word_0(&s); // word 的值为 5
                s.clear(); // 清空字符串使其等于 ""
                // word 此处值仍然是5(但已无用途)
                /* 1.程序正常编译时且调用 s.clear()后 word可正常使用(因为word与s状态无关
                     故word仍然包含值 5,可尝试用值 5 来提取变量 s 的第一个单词但有bug[因在
                     将5保存到word后s内容已改变])
                   2.故不得不时刻担心word索引与s数据不同步(啰嗦且易出错)
                     如再编写second_word函数的话则管理索引则更加容易出问题
                     // fn second_word_0(s: &String) -> (usize, usize) {//...}
                   3. 如上所述要跟踪开始索引和结尾索引同时记录更多从数据某个特定状态计算而来的值，
                      但都完全没有与这个状态相关联，故保持索引、计算值等飘忽不定且不相关变量同步
                      则比较繁锁且容易出错(解决方案 : 字符串 slice)
                */
            }

1. iter、into_iter 及 iter_mut 用法及场景[?]
      let v1 = vec![1, 2, 3];
      /* a. v1_iter 需要是可变的：在迭代器上调用 next 方法改变了迭代器中用来记录序列位置的状态。
            换言之代码消费(consume)了或使用了迭代器,每 next 调用都会从迭代器中消费一个项,用for
            循环时无需使 v1_iter 可变因为 for 循环会获取 v1_iter 所有权并在后台使 v1_iter可变
         b. 从 next 调用中得到的值是 vector 的不可变引用。iter 方法生成一个不可变引用的迭代器,
            若需要获取 v1 所有权并返回拥有所有权的迭代器则可调用 into_iter 而不是 iter；若希望
            迭代可变引用则可调用 iter_mut 而不是 iter
      */
      let mut v1_iter = v1.iter();
      // 重复调用由vector创建迭代器的next方法获取值
      assert_eq!(v1_iter.next(), Some(&1));
      assert_eq!(v1_iter.next(), Some(&2));
      assert_eq!(v1_iter.next(), Some(&3));
      assert_eq!(v1_iter.next(), None);

2. trait 关联类型
    impl Iterator for Counter {
        // 迭代器的关联类型 Item 设置为 u32,意味着迭代器会返回 u32 值集合(无需担心关联类型)
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {//...}
        //问题:为何不直接用类似" fn next(&mut self) -> Option<Self::u32> {//...} "的结构[?]
    }

3. 迭代器适配器 zip、map


*/