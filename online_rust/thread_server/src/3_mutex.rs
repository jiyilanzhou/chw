
/*
0. 循环创建多线程修改共享变量
   使用静态变量完成修改共享变量

*/

use std::thread;
// 静态变量宜声明在函数外部
static mut N: i32 = 0;

fn main_0() {
    // 静态变量声明(最好在函数外部声明)
    //static mut N: i32 = 0;
    // 绑定
    let mut n = 0;  // 可使用 Mutex 实现
    let mut pool = Vec::new();
    // 多线程
    for _ in 0..6 {
        pool.push(thread::spawn(move || {
            /* 通过 move 传入的 n 是原 n 的副本
               因为 i32 是可 Copy 的即操作的是副本(故对原数据无影响)
            */
            n += 1;
            unsafe {
                N += 1;
            }
        }));
    }
    // 等待
    for t in pool {
        t.join().unwrap();
    }
    // 最终结果
    println!("{}", n);  // Console:" 0 "
    unsafe {
        println!("{}", N);// Console" 6 "
    }
}

/*
1. 使用 Mutex 多线程修改共享变量

 */
use std::sync::{Arc, Mutex};

fn main() {
    let share_num = Arc::new(Mutex::new(0));
    let mut pool = Vec::new();
    // 多线程
    for _ in 0..6 {
        let share_num_thread = share_num.clone();
        pool.push(thread::spawn(move || {
            // 加锁
            let mut num = share_num_thread.lock().unwrap();
            *num += 1;
            // 离开作用域自动解锁
        }));
    }
    // 等待
    for t in pool {
        t.join().unwrap();
    }
    // 最终结果
    println!("{:?}", share_num);  // Console:" Mutex { data: 6 } "
    println!("{:?}", share_num.lock().unwrap());  // Console:" 6 "

}