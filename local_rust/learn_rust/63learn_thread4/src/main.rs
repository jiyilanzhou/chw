
/* 线程间使用 mutex */
use std::sync::Mutex;
use std::thread;

fn main_0() {
    // 定义计数器
    let counter = Mutex::new(0);
    /* (将外部 counter 移至多个子线程[违反所有权])编译报错:
       error[E0382]: use of moved value: `counter`
         let counter = Mutex::new(0);
             ------- move occurs because `counter` has type `std::sync::Mutex<i32>`,
                                          which does not implement the `Copy` trait
             let handle = thread::spawn(move || {
                                        ^^^^^^^ value moved into closure here,
                                                      in previous iteration of loop
                 let mut num = counter.lock().unwrap();
                               ------- use occurs due to use in closure
        // 解决参考方案: 尝试使用" Rc<T> "实现共享(如"main_1"方法[同样编译错误])
    */
    // 定义线程数组存储于 vec
    let mut handles = vec![];
    // 创建子线程
    for _ in 0..10 {
        let handle = thread::spawn(move || {
            /* 暂且注释:以通过编译
                // 欲修改须先获取锁
                let mut num = counter.lock().unwrap();
                *num += 1;
            */
        });
        handles.push(handle);
    }
    // 等待子线程结束
    for handle in handles {
        handle.join().unwrap();
    }
    println!("resut = {}", *counter.lock().unwrap());
}

use std::rc::Rc;

fn main_1() {
    // Rc<T> 非线程安全
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];
    /* 编译错误：// " Rc "在线程间不能安全发送(即"Rc<T>"非线程安全)
        error[E0277]: `std::rc::Rc<std::sync::Mutex<i32>>` cannot be sent between
                                                                   threads safely
          let handle = thread::spawn(move || {
                       ^^^^^^^^^^^^^ `std::rc::Rc<std::sync::Mutex<i32>>` cannot
                                                   be sent between threads safely

           = help: within `[closure@src\main.rs:48:36: 51:10 cnt:std::rc::Rc<std
             ::sync::Mutex<i32>>]`, the trait `std::marker::Send` is not implemented
                                             for `std::rc::Rc<std::sync::Mutex<i32>>`
           = note: required because it appears within the type `[closure@src\main.rs
                               :48:36: 51:10 cnt:std::rc::Rc<std::sync::Mutex<i32>>]`
           = note: required by `std::thread::spawn`
        // 解决方案: Arc<T>
    */
    // 创建子线程
    for _ in 0..10 {
        let cnt = Rc::clone(&counter);  // Rc<T> 非线程安全
        let handle = thread::spawn(move || {
            /* 暂且注释以通过编译:
                let mut num = cnt.lock().unwrap();
                *num += 1;
            */
        });
        handles.push(handle);
    }
    // 等待子线程结束
    for handle in handles {
        handle.join().unwrap();
    }
    println!("resut = {}", *counter.lock().unwrap());
}

use std::sync::Arc;

// 使用 Arc 智能指针
fn main() {
    // 定义计数器
    //let counter = Mutex::new(0);
    //let counter = Rc::new(Mutex::new(0));
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    // 创建子线程
    for _ in 0..10 {
        let cnt = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = cnt.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    // 等待子线程结束
    for handle in handles {
        handle.join().unwrap();
    }

    // Console:" 10 "(因 10 个线程都进行了" *num += 1 "的操作)
    println!("resut = {}", *counter.lock().unwrap());

}

/* 智能指针:
RefCell<T> / Rc<T> 与 Mutex<T> / Arc<T>
    a、Mutex<T> 提供内部可变性，类似于 RefCell
    b、RefCell<T> / Rc<T> 是非线程安全的， Mutex<T> / Arc<T> 是线程安全的

*/