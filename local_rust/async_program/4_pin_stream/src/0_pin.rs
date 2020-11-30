use futures::executor;

/*
0. 编译器对 async 代码块展开分析

 */
// async 将对应的函数或代码块转化为实现" Future trait "的状态机
async fn async_fn1() {
    println!("async fn1 ... ");
}
async fn async_fn2() {
    println!("async fn2 ... ");
}
async fn async_main() {
    // 因" async_fn "为异步函数故" async_fn() "并未真实调用
    let fn1 = async_fn1();
    let fn2 = async_fn2();
    // 同理异步块将其转化为实现" Future trait "的状态机
    let f = async move {
        // 顺序执行
        fn1.await;
        fn2.await;
    };
    /* 分析编译器展开" let f = async move {...} "异步代码块的过程:
        a. 创建一个匿名结构体
        b. 为结构体定义了对应的状态
        c. 实现 Future trait
             struct AsyncFuture {
                fut_one: FutFunction1,
                fut_two: FutFunction2,
                state: State,
             }
             enum State {
                AwaitFut1,
                AwaitFut2,
                Done,
             }
            // 因其为 async 代码块故转化为 Futrue (即实现 Future)
            impl Future for AsyncFuture {
               type Output = ();
               fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
                   loop {
                       match self.state {
                           State::AwaitFut1 => match self.fut_one.poll(...) {
                               Poll::Ready(()) => self.state = State::AwaitFut2,
                               Poll::Pending => return Poll::Pending,
                           }
                           State::AwaitFut2 => match self.fut_two.poll(...) {
                               Poll::Ready(()) => self.state = State::Done,
                               Poll::Pending => return Poll::Pending,
                           }
                           State::Done => return Poll::Ready(());
                       }
                   }
               }
            }
     */
    f.await;

    // f(其内"async_fn1、async_fn2"同步) 与 async_fn3 实现异步
    // futures::join(f, async_fn3());
}
async fn async_fn3() {
    println!("async fn3 ... ");
}
fn main_0() {
    executor::block_on(async_main());
}

/*
1. Pin 针(固定之意)
   await 实现原理[???]

 */
async fn async_put_data_to_buf(mut buf: &[u8]) {
    //to do something
}
async fn async_main_pin() {
    /* 编译器展开" let f = async {...} "异步代码块分析：
        a. // 展开" let mut x = [0; 128]; "
            struct AsyncFuture {
               x: [u8: 128],
               async_put: PutIntoBuf<'what_lifetime>,
            }
        b. // 展开" async_put_data_to_buf(&mut x) "
            struct PutIntoBuf {
               buf: &'a mut[u8],
            }
        c. AsyncFuture 移动分析(图"0_Pin解决地址随之更新问题.png")：
            (0). 如果 AsyncFuture 发生移动，那么 x 和 async_put 都发生移动，
                 但是 async_put.buf 还是指向移动之前的 x，显然不是所期望的
            (1). 预期 AsyncFuture 发生移动后 async_put.buf 指向移动之后的 x
            (2). Pin<T> 类型包裹着指针类型(保证指针类型背后的值不被移动)。且
                大多数类型都不存移动问题(其实现 UnPin trait [如" u8 "])。其
                表象类似：
                    fn my_function<T>(T: impl UnPin){}
                    fn main(){
                        let f = async {};    // let f = Pin<AsyncFuture>
                        let fut = Box::pin(f);
                        //pin_mut!(fut);    // 可用于处理数据
                        my_function(fut);
                    }

     */
    let f = async {
        let mut x = [0; 128];
        let async_put = async_put_data_to_buf(&mut x);
        async_put.await;
    };
}

fn main() {
    executor::block_on(async_main_pin());
}