
/*
0.元组与数组区别：
     数组只能存储数据类型一致的元素而元组可存储数据类型相异的元素

1. Rust 编译过程：
     " 生成 AST -> HIR -> MIR -> LLVM -> 机器码 "
      // 在线编译查看" MIR "代码

*/
fn main_0() {
    /*[自] trait bounds 用于 Type 实现的 trait
         1. trait 限定用于 <Type as trait>
            而 i32 非 trait(更别说是 Vec 实现的 trait)
         2. trait 限定用于 trait 内定义的函数
     */
    // let vec = <Vec as i32> ::new();  // 飘红报错

    // 使用泛型
    // let vec:Vec<i32> = Vec::new(); // 单独声明需指定类型

    // 通过首追加的元素推断类型
    let vec = Vec::new();   // 单独声明(即从未添加首元素)编译错误
    vec.push(3);

    // 使用迭代器 ..
    let vec: Vec<i32> = (0..5).collect();
}

/*
字符串常用操作
    String::new("")
    String::with_capacity(n)
    iter.collect()
    len()
    is_empty()
    slice[range]
    push(ch)
    push_str(slice)
    extend(iter)
    insert(i,ch)
    clear
    pop
    find

// 源码查询：
    github -> rust ("All GitHub"搜索)-> rust-lang/rust -> src
           -> liballoc (字符串所在包) -> string.rs/str.rs -> 搜索
    如搜索" struct String "即可查找到" Stirng "结构体源码：
        pub struct String {
            vec: Vec<u8>,
        }

*/
fn main_1() {
    let mut s = String::from("hello");
    let emojy = " world";
    s.push_str(emojy);
    println!("{}", s);
    s.extend(" ! c h w ".split_whitespace());
    println!("{}", s);
}

// 内部迭代器：不可控制
impl<T: Copy> InIterator<T> for Vec<T> {
    fn each<F: Fn(T) -> T>(&mut self, f: F) {
        let mut i = 0;
        while i < self.len() {
            self[i] = f(self[i]);
            i += 1;
        }
    }
}

fn main_2() {
    // 外部迭代
    let mut i = 0;
    loop {
        if i == 3 {
            break;  // 外部循环可控：故可用 break/continue 等
        }
        i += 1;
    }
    println!("{}",i);

    // 内部迭代
    let mut v = vec![1, 2, 3];
    //v.each(|i| i + 3);
    v.each(|i| {
        println!("{}", i);
        /* 飘红报错:
           `break` cannot be used in closures, only inside `loop`
            and `while` blocks [E0267]
        */
        // 内部迭代不可控：故不可用 break/continue 等
        //break;   // continue;
        i + 3
    });
    println!("{:?}", v)
}

fn main() {
    let vec = vec![1, 2, 3];
    // 虽然 vec[0] 拥有所有权，但其实现了" Copy trait "
    // 故 latest 获取的是 vec[0] 副本的所有权(对原 vec[0] 无影响)
    let mut latest = vec[0];
    for i in vec {
        if *i > latest {
            latest = *i;
        }
    }
    print!("{}", latest);
    //print!("{:?}", vec);  //编译报错：所有权已转移
}