
use std::thread;
use std::time::Duration;

/* Option 用法：
        a. 初始化值;
        b. 作为在整个输入范围内没有定义的函数的返回值;
        c. 作为返回值，用 None 表示出现的简单错误;
        d. 作为结构体的可选字段(当无需为结构体字段设置值时可用 None );
        e. 作为结构体中可借出或者是可载入的字段;
        f. 作为函数的可选参数;
        g. 代表空指针;
        h. 用作复杂情况的返回值。
            pub enum Option<T> {
                None,
                Some(T),
            }

 */

// 示例：作为结构体可借出或可载入的字段
struct Worker {
    // Worker 定义及实现
    // thread: thread::JoinHandle<()>,      // 线程操作句柄
    // 使用 Option 作为结构体可借出或可载入的字段
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new() -> Worker {
        let thread = thread::spawn(move || {
            println!("start work 10 secs ... ");
            thread::sleep(Duration::from_secs(10));
            println!("work 10 secs finish... ");
        });
        //Worker { thread } // 源于" thread: thread::JoinHandle<()> "
        Worker { thread: Some(thread) }
    }
}

// ThreadPool 定义及实现
struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        for _ in 0..size {
            workers.push(Worker::new());
        }
        ThreadPool { workers }
    }
}

// 离开作用域则自动析构
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            /* 当" worker.thread "是" thread::JoinHandle<()> " 时编译报错：
                error[E0507]: cannot move out of `worker.thread` which is
                                               behind a mutable reference
                   worker.thread.join().unwrap();
                   ^^^^^^^^^^^^^ move occurs because `worker.thread` has
                      type `std::thread::JoinHandle<()>`, which does not
                                              implement the `Copy` trait
              // 分析：不可将 worker.thread 移出，因其跟在可变引用的后面[?]
             */
            // worker.thread.join().unwrap();

            /* 当" worker.thread "是" Option<thread::JoinHandle<()>> "
               (等待 worker 工作结束)仍然编译报错: 因为" worker.thread "
               不再是简单而已是通过 Option 包装的"线程操作句柄"
             */
            // worker.thread.join().unwrap();

            /* Option 之 take() 源码：
                /// Takes the value out of the option, leaving a [`None`]
                ///                                         in its place.
                /// [`None`]: #variant.None
                ///
                /// # Examples
                ///
                /// ```
                /// let mut x = Some(2);
                /// let y = x.take();
                /// assert_eq!(x, None);
                /// assert_eq!(y, Some(2));
                ///
                /// let mut x: Option<u32> = None;
                /// let y = x.take();
                /// assert_eq!(x, None);
                /// assert_eq!(y, None);
                /// ```
                #[inline]
                #[stable(feature = "rust1", since = "1.0.0")]
                pub fn take(&mut self) -> Option<T> {
                    mem::take(self)
                }
             */
            //" worker.thread: (Some(thread)) -> None "即取出值后为" None "
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
                println!("worker thread finished!");
            }
        }
    }
}

fn main() {
    // let _pool = ThreadPool::new(3);

    /* Option 之 is_some / is_none 方法：
        a. " is_some "源码：   pub fn is_some(&self) -> bool {
                                  matches!(*self, Some(_))
                               }
              // 当 Option 中有值时返回 true
         b. " is_none "源码：   pub fn is_none(&self) -> bool {
                                    !self.is_some()
                                }
              // 当 Option 为 None 时返回 true
         c. " contains "源码：  pub fn contains<U>(&self, x: &U) -> bool
                                where
                                    U: PartialEq<T>,
                                {
                                    match self {
                                        Some(y) => x == y,
                                        None => false,
                                    }
                                }
              // 当 Some 中包含给定的值时，返回 true (此为 nightly API )
     */
    let mut x = Some(2);
    let y = x.take();
    if !x.is_some() {
        println!("x : none!");
    }
    if !y.is_none() {
        println!("y : some!");
    }

    /*  // 飘红报错：待实现[?]
        #![feature(option_result_contains)]
        let x:Option<u32> = Some(3);
        assert_eq!(x.contains(&1),false)
        assert_eq!(x.contains(&3),true)
     */

    /* Option 之 as_ref 方法
        源码：   pub fn as_ref(&self) -> Option<&T> {
                    match *self {
                        Some(ref x) => Some(x),
                        None => None,
                    }
                }
         // 将 &Option<T> 转换为 Option<&T>
     */
    let text: Option<String> = Some("Hello, world!".to_string());
    let text_length: Option<usize> = text.as_ref().map(|s| s.len());
    println!("still can print text: {:?}", text);
    if let Some(length) = text_length {
        println!("string leng = {}", length);
    }

    /* Option 之 copied 方法：
        源码：  pub fn copied(self) -> Option<T> {
                    self.map(|&t| t)
                }
         // 从 Option<&T> 得到 Option<T> : 通过复制来实现( T 须实现 Copy trait)
     */
    let x = 12;
    let opt_x = Some(&x);
    assert_eq!(opt_x, Som(&12));
    let copied = opt_x.copied();
    assert_eq!(copied,Some(12));

    /* Option 之 cloned  方法：
         cloned 源码： pub fn cloned(self) -> Option<T> {
                          self.map(|t| t.clone())
                       }
          // 从 Option<&T> 得到 Option<T> : 通过克隆实现( T 须实现 Clone trait)
          // 调用 的 clone 源码： fn clone(&self) -> Self {
                                      match self {
                                          Some(x) => Some(x.clone()),
                                          None => None,
                                      }
                                  }
   */
    let x = 12;
    let opt_x = Some(&x);
    assert_eq!(opt_x, Som(&12));
    let copied = opt_x.cloned();
    assert_eq!(copied,Some(12));

    // 其余详情请参阅标准库文档
}