use futures;

async fn step_one() {}
async fn step_two() {}
// 普通异步调用
async fn foo() {
	step_one().await;
	step_two().await;
	/* 编译器编译 foo 转化后的 Future 表象:
		struct FooSN {
			f1: Foo::First,
			f2: Foo::Second,
			state: FooState,
		}
		enum FooState {
			F1_Pending,
			F1_Ready,
			F2_Pending,
			F2_Ready,
			Ready
		}
		// generates a type like this:
		enum Foo{
			First(StepOne),
			Second(StepTwo),
		}
	 */
}

use futures::future::{BoxFuture, FutureExt};
/* 递归异步调用:
   编译报错：异步递归需要装箱
	  error[E0733]: recursion in an `async fn` requires boxing
	     async fn recur() {
	  					^ recursive `async fn`
	     = note: a recursive `async fn` must be rewritten to return a boxed `dyn Future`
   解决方案：使用 box
 */
/* // 暂且注释以通过编译
	async fn recur() {
		recur().await;
		recur().await;
		/* 编译器编译 recursive 转化后的 Future 表象:
			struct RecurSN {
				f1: Recur::First,
				f2: Recur::Second,
				state:RecurState
			}
			enum RecurState {
				// ...
			}
			// generates a type like this:
			enum Recur {
				First(Recur),
				Second(Recur),
			}
		*/
	}
 */

// 递归(使用 box 解决)
fn recur() -> BoxFuture<'static, ()> {
	async move{
		recur().await;
		recur().await;
	}.boxed()
}

fn main() {
	futures::executor::block_on(foo());
	//futures::executor::block_on(recur());

	/* 递归(使用 box ):直接调用
	   (即不再使用执行器 executor [因此时 recur 非 async 修饰])
	*/
	recur();
}

/* 后续深入：
0. 异步
	a. futures 文档、源码
	b. tokio 文档、源码
	c. smol、async-std 文档、源码

1. 开源项目
	libra

2. Rust 本身
	a.《 Rust 死灵书 》
	b. Rust 标准库


 */