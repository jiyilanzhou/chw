
// outlive[ˌaʊtˈlɪv]v.比...活得长
use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    /* 编译报错:
        error[E0373]: closure may outlive the current function, but it
                   borrows `v`, which is owned by the current function
            let handle = thread::spawn(|| {
                                       ^^ may outlive borrowed value `v`
                println!("v: {:?}", v);
                                    - `v` is borrowed here
         = note: function requires argument type to outlive `'static`
                   let handle = thread::spawn(|| {
              __________________^
             |         println!("v: {:?}", v);
             |     });
             |______^
         = help: to force the closure to take ownership of `v` (and any other
                             referenced variables), use the `move` keyword
                 let handle = thread::spawn(move || {
       // 分析：子线程内欲使用外部的变量值但不知其能存活时间，故子线程内无法保证
                外部变量值在使用期间持续有效(违反 Rust 安全的设计规则)如：
                    let handle = thread::spawn(|| {
                        // sleep(10); // 子线程未来得及开始使用就被主线程 drop
                        println!("v: {:?}", v);
                    });
                    drop(v);
       // 解决方案: 可使用" move "关键字
    */
    /* // 子线程不能保证外部变量于使用期间持续有效
        let handle = thread::spawn(|| {
            println!("v: {:?}", v);
        });
    */

    // 使用" move "关键字将变量移至子线程
    let handle = thread::spawn(move || {
        println!("v: {:?}", v);
    });

    // 编译报错" error[E0382]: borrow of moved value: `v` "
    // println!("in main thread, v: {:?}", v); // 移到后不可再用

    handle.join().unwrap();

}