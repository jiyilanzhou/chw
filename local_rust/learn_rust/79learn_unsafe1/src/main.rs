
/*
1. 安全 Rust ： 编译时会强制执行的内存安全保证
   不完全 Rust : 编译时不会强制执行内存安全保证

2、不安全的 Rust 存在的两大原因
    (1）静态分析本质上是保守的即意味着某些代码可能是合法的但 Rust 也会拒绝,
        此情况下可以使用不安全的代码即" unsafe 片段 "。
    (2）底层计算机硬件固有的不安全性(若禁止不安全操作则有些任务根本无法完成)

3、不安全 Rust 具有的超级力量(通过 unsafe 关键字切换到不安全的 Rust ):
    （1）解引用裸指针
    （2）调用不安全的函数或者方法
    （3）访问或修改可变静态变量
    （4）实现不安全的 trait
     // 注: unsafe 并不会关闭借用检查器或禁用任何其它的 Rust 安全检查规则，
           其仅提供上述几个不被编译器检查内存安全的功能。 unsafe 也不意味
           块中的代码一定就是不 ok 的(仅表示由程序员来确保安全)

4、解引用裸指针:不可变和可变分别写作 *const T, *mut T
   a. 允许忽略借用规则，可同时拥有不可变和可变指针或多个指向相同位置的可变指针
   b. 不保证指向的内存是有效的
   c. 允许为空
   d. 不能实现任何自动清理的功能

*/

fn main() {
    let mut num = 5;
    /* 可在安全代码内创建不可变和可变裸指针: (但不能在 unsafe 块之外解引用裸指针)
       裸指针可同时拥有不可变和可变指针或多个指向相同位置的可变指针(区别于引用[?])
     */
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    /* 编译错误: 将" &i32 "转换为" *mut i32 "无效
       error[E0606]: casting `&i32` as `*mut i32` is invalid
           let r3 = &num as *mut i32;
       // 原因分析:" &num "为取变量地址(默认即为不可变故不可作为" *mut i32")
          注:应更改为" &mut num as *mut i32; "即" r2 "
    */
    //let r3 = &num as *mut i32;    // 暂且注释：以通过编译

    /* (同理)编译错误：
        error[E0606]: casting `&mut &i32` as `*mut i32` is invalid
            let r3 = &mut &num as *mut i32;
       //[自]原因分析: " &mut num "即是获取变量地址(再" &num "则应为二级指针
                      故与" *mut i32 "一级指针不匹配[待验证])
    */
    // let r3 = &mut &num as *mut i32; // 暂且注释：以通过编译

    let _r4 = num as *mut i32;

    /* 解引用原生指针(裸指针)：飘红报错
         Dereference of raw pointer requires unsafe function or block [E0133]
        // 解引用裸指针须置于" unsafe 函数 "或" unsafe 块 "
    */
    // println!("r1 is: {}", *r1);
    // println!("r2 is: {}", *r2);

    // 解引用裸指针(须置于" unsafe "块)
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);

        /* 问题:编译通过但" cargo run "运行报错：
           error:process didn't exit successfully:`target\debug\learn_unsafe1.exe`
             (exit code: 0xc0000005, STATUS_ACCESS_VIOLATION)
           Segmentation fault
           // [自]原因分析: " num as *mut i32 "即是将变量" num "(而非变量地址"&num")
              作为" *mut i32 "可变指针使用(故应更改为" &mut num as *mut i32 "即"r2")
           // [自]注:没有"&num as *mut i32;"的用法(因不可`&i32` as `*mut i32`:即不可
                                   将不可变"&num"当作可变"*mut i32"])  // 类型不匹配
              亦没有" &mut &num as *mut i32; "用法(因不可`&mut &i32` as `*mut i32`：
                                 即不可将" &mut &i32 "当作" *mut i32 ")// 类型不匹配
           // 待更深层次理解[?]
        */
       // println!("r3 is: {}", *_r4);// 暂且注释：以通过" cargo run "
    }

    // 裸指针定义与使用
    let addr = 0x12345usize;
    let _r = addr as *const i32;
    unsafe {
        println!("r is: {}", *_r);  // Console:" 0 "
    }
    let _r = addr as *mut i32;
    unsafe{
        *_r = 6;
        println!("r is: {}", *_r); // Console:" 6 "
    }

}