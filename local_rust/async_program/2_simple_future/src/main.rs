
// poll[pəʊl]v&n.投票,轮询(顺序访问) // future[ˈfjuːtʃə(r)]n.未来(将来)
// pend[pend]v. 搁置      // pending[ˈpendɪŋ]n. 搁置,挂起
// react[riˈækt]v. 反应   // reactor[riˈæktə(r)]n. 反应器
/*
0. 自定义实现 Future
   a. " std::future::Future; / core::future::Future;"源码：
        pub trait Future {
            type Output;
            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
        }
        pub enum Poll<T> {
            Ready(#[stable(feature = "futures_api", since = "1.36.0")] T),
            Pending,
        }
        #[derive(Copy, Clone)]
        pub struct Pin<P> {
            pointer: P,
        }
    b. " async-book "之 SimpleFuture 源码：
        trait SimpleFuture {
            type Output;
            // wake 用于通知 Excutor
            fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
        }
        enum Poll<T> {
            Ready(T),       // 准备就绪
            Pending,        // 挂起
        }
        // 文档" https://rust-lang.github.io/async-book/02_execution/02_future.html "

*/
use std::future::Future;
use std::thread;
use std::time::Duration;
trait SimpleFuture {
    type Output;
    //fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
    fn poll(&mut self, wake: u32) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}

struct MySleeper {
    // 记录循环次数
    polls: u64,
    // 简单以序号表示(实际应传入 wake 函数)
    wake: u32,
}

static mut FINISHED: bool = false;

impl MySleeper {
    fn new(wake: u32) -> Self {
        MySleeper {
            polls: 0,
            wake: wake,
        }
    }
}

impl SimpleFuture for MySleeper {
    type Output = ();
    fn poll(&mut self, wake: u32) -> Poll<Self::Output> {
        unsafe {
            if FINISHED {
                Poll::Ready(())
            } else {
                self.wake = wake;
                self.polls += 1;
                println!("polls = {}", self.polls);
                Poll::Pending
            }
        }
    }
}

struct MyReactor {
    wake: u32,
    handle: Option<thread::JoinHandle<()>>,
}

impl MyReactor {
    fn new() -> Self {
        MyReactor {
            wake: 0,
            handle: None,
        }
    }

    fn add_wake(&mut self, wake: u32) {
        self.wake = wake;
    }

    fn check_status(&mut self) {
        if self.handle.is_none() {
            let _wake = self.wake;
            let handle = thread::spawn(|| loop {
                thread::sleep(Duration::from_secs(5));
                unsafe {
                    //模拟future就绪，然后调用wake
                    FINISHED = true;
                }
            });
            self.handle = Some(handle);
        }
    }
}

struct MyExecutor;

impl MyExecutor {
    fn block_on<F: SimpleFuture>(mut myfuture: F, wake: u32) {
        // 模拟不断循环检测任务
        loop {
            match myfuture.poll(wake) {
                Poll::Ready(_) => {
                    println!("my future is ok!");
                    break;
                }
                Poll::Pending => unsafe {
                    while !FINISHED {
                        thread::sleep(Duration::from_secs(1));
                    }
                },
            }
        }
    }
}

fn main() {
    let mut reactor = MyReactor::new();
    let sleeper = MySleeper::new(5);
    let wake = sleeper.wake;
    reactor.add_wake(wake);
    reactor.check_status();
    MyExecutor::block_on(sleeper, wake);
}
