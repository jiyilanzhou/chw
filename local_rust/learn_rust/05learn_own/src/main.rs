
/*
1. rust 通过所有权机制来管理内存
     编译器编译时会根据所有权规则对内存的使用进行检查

2. 堆和栈
    编译的时候数据的类型大小是固定的，就是分配在栈上的
    编译的时候数据类型大小不固定，就是分配堆上的

3. 作用域:{}

4. String内存回收

5. 移动

6. clone

7. 栈上数据拷贝

8. 函数和作用域

*/

fn main() {
    // x 作用域开始直至 main 函数" } "结束前(即词法 lexic 作用域)
    let x: i32 = 1;
    {
        // x, y 编译时数据类型("i32")大小固定:故分配于栈上
        // 作用域 scope 开始
        let y: i32 = 1;
        println!("x = {}", x);
        println!("y = {}", y);
        // 作用域 scope 结束
    }
    // println!("y = {}", y); // 作用域 scope 结束再使用则报错
    println!("x = {}", x);  // 仍在 x 作用域范围内故有效

    {
        /* s1 编译时数据类型("String"[无论是否自动推导类型])大小不固定故分配于堆上(
           Rust 中" s1 "如图" 0_字符串片.png " )：
           s1 是"携带指针(指向堆空间数据)ptr、长度 len 及容量 capacity 属性的 "切片
           故" s1.push_str(args) "即是更改指针 ptr 所指向的堆空间数据(当然亦会根据
           指针指针数据设置长度 len 及 容量 capacity )
        */
        let s1 = String::from("hello");
        //s1.push_str(" world"); // 欲执行先使 s1 声明为" mut "否则编译报错
        println!("s1 = {}", s1); // String 类型离开作用域的时候会调用 drop 方法

        /* 深浅拷贝：浅拷贝仅拷贝堆空间的引用(深拷贝还拷贝引用指向的堆空间数据)
           a. "C、Go"语言将"s1"赋值给"s2"相当于浅拷贝即仅拷贝引用(图"1_拷贝引用.png")
              从而当"s1、s2"离开作用域时又分别调用相应方法将引用释放(即引用计数递减)
           b. 在 Rust 中为避免重复回收(即释放引用)则采取移动(而非浅拷贝)方式，即先将 s1
              拷贝为 s2 (栈空间数据为直接拷贝)，再将 s1 的所有权(即指向堆空间的引用)移至
              s2 (从而使 s1 失效)。故当离开作用域时只需释放一次([自] Rust 中没有"浅拷贝")
        */
        let s2 = s1;

        /*// 违反借用规则:
          println!("s1= {}", s1);       // move to s2, s1 invalid
          // 编译报错：
            error[E0382]: borrow of moved value: `s1`
              let s2 = s1;
                        -- value moved here
               println!("s2= {}", s2);
               println!("s1= {}", s1);
                                  ^^ value borrowed here after move

        */
        println!("s2= {}", s2);

        // clone  // clone 为深拷贝如图" 2_深拷贝.png "
        let s3 = s2.clone();
        println!("s3= {}", s3);
        println!("s2= {}", s2);
    }

    // copy trait // 栈上基本数据(非变量)没有类似指向堆数据的引用故拷贝的即是数据本身
    let a = 1;  // " 1 "为固定大小故其为"栈上数据"
    let b = a;
    /*  copy trait 特性：
        现象：" a "已赋值给" b ",按借用规则则 a 应该失效但却能正常编译运行
              究其根本原因是" 栈上数据为直接拷贝 "(已知固定大小为栈上数据)
        特性： 实现 copy trait 即可实现拷贝功能(基础数据类型皆实现[String 为引用类型])
    */
    println!("a = {}, b = {}", a, b);
    /* 具有 copy trait 常用类型：
          整型
          浮点型
          布尔值
          字符类型 char
          元组
    */
}