
// guard [ɡɑːd]n.守卫,防护
/*
1、通道(类似于单所有权方式): 值传递到通道后发送者则无法再使用

2、互斥器共享内存(类似于多所有权): 即多个线程可并发访问相同的内存位置
   互斥器：mutex
      a、任意时刻仅允许单一线程访问某些数据
      b、互斥器使用前后需获取及释放锁
      c、互斥器" Mutex<T> "是智能指针调用 lock 返回 MutexGuard 智能指针
         其内部提供 drop 方法(实现当 MutexGuard 离开作用域时自动释放锁)

*/

use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        // 修改前先获取锁
        let mut num = m.lock().unwrap();
        *num = 6;
    }   // 离开作用域自动释放锁(即调用 drop 方法)

    println!("m = {:?}", m);    // Console:" m = Mutex { data: 6 } "
}


