
/* 线程与进程：
（1）进程是资源分配的最小单位，线程是 CPU 调度的最小单位
（2）在使用多线程时，经常会遇到的一些问题：
      1.竞争状态：多个线程以不一致的顺序访问数据或资源
      2.死锁：两个线程相互等待对方停止使用其所拥有的资源，从而造成两者都永久等待。
        如" A Thread 获取资源顺序: 1->2->3 ";  " B Thread 获取资源顺序: 2->1->3 "
        在"t1"时刻" A 获取到 1 资源, B 获取到 2 资源接着 A、B 相互等待对方释放所持有
        的资源从而造成死锁
      3.只会发生在特定情况下且难以稳定重现和修复的 bug (即难以定位 bug )
（3）编程语言提供的线程叫做绿色线程，如 go 语言在底层实现" M：N "的模型(即 M 个绿色线程
     对应 N 个 OS 线程[如 10 个协程可能对应操作系统的 2 个线程])。但 Rust 标准库只提供
      " 1：1 "的线程模型的实现即 1 个 Rust 线程对应 1 个 Os 线程
 (4) 运行时代表二进制文件中包含由语言本身提供的代码，这些代码根据语言不同可大可小但一般
     非汇编语言都会有一定数量的运行时代码[通常说某语言"没有运行时"是指此语言的" 运行时 "
     很小( Rust、C都是几乎没有运行时的)

*/

use std::thread;
use std::time::Duration;

fn main_0() {
    // 创建
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("number {} in spawn thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // 主线程
    for i in 1..5 {
        println!("number {} in main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // 阻塞等待子线程结束
    handle.join().unwrap();

    println!("Hello, World!");
}

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("number {} in spawn thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 阻塞等待子线程结束
    handle.join().unwrap();

    println!("+++++++++++++++++++++");
    for i in 1..5 {
        println!("number {} in main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

}

