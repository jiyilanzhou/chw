// experience[ɪkˈspɪəriəns]n. 经验
// experiment[ɪkˈsperɪmənt]v&n. 实验,尝试
#![feature(async_closure)]
#![feature(in_band_lifetimes)]

use std::future::Future;
/*
0. async 用法(主要是前两种) ：
	 a. 用于 function : 如" async fn func(){} "
	 b. 用于 block : 如" async {} "
	 c. 用于 closure : 如" async ||{}  "(可将其看作函数或代码块)

 */
// async 用于 closure
fn func() -> impl Future<Output=u8> {
    // 使用闭包
    let closure = async move |x: u8| {
        x
    };
    closure(5)
}

// 1. async Lifetimes
use futures::executor;

async fn foo(x: &u8) -> u8 { *x }

/*飘红报错：未标注" #![feature(in_band_lifetimes)] "
	in-band lifetimes is experimental [E0658]
		带内生命周期是实验性的(故须标注" #![feature(in_band_lifetimes)] ")
    " fn foo_expand(x:&'a u8)->impl Future<Output=u8> + 'a{ async{ *x }} "
    为" async fn foo(x: & u8) -> u8 { *x } "的等价写法
 */
/* 返回值 Future object 内含参数的引用需要标注生命周期:
	   因" bad()、good() "可能在不同的线程间调用(其内产生不同的 Future)
	   如" bad()、good() "内局部变量 x 会随着调用的结束而销毁故其引用需
	   标注生命周期(即在线程间传递 Future 需要标注生命周期)
 */
fn foo_expand(x: &'a u8) -> impl Future<Output=u8> + 'a {
    async move { *x }
}

fn bad() -> impl Future<Output=u8> {
    // let x = 5;
    let x: &'static u8 = &5;
    foo_expand(&x)
}

fn good() -> impl Future<Output=u8> {
    /* 生命周期：
          此代码块若没有 async 关键字修饰则出" {} "其 x 即销毁
          但使用 async 修饰则将 x 生命周期与 Future object (即
          " foo_expand(&x) "绑定在一起)
     */
    async {
        let x = 5;
        foo_expand(&x).await
    }
}

fn main_0() {
    let x = 5;
    let f = foo(&x);
    executor::block_on(f);
}

/*
2. async move
   [自]原理同于" std::thread::spawn(move closure){} "
       若未 move 则于 closure 应用时其可能由于外部环境改变或销毁
       从而引发安全(故使用　move 强制获取所有权)。再者也只有获取
       环境变量的所有权才能将变量在多线程间传递。

 */
async fn move_block() {
    let my_string = "my string".to_string();
    // 将环境变量 my_string 移入 Future
    let f = async move {
        println!("String: {}", my_string);
    };
    // println!("{}", my_string); // 移动后不可再用
    f.await
}

fn main() {
    executor::block_on(move_block());
}