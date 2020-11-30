
// Rc 系列：非线程安全
use std::rc::Rc;

#[derive(Default)]
struct NoSend(Rc<()>);
async fn bar() {}
async fn foo() {
	/* Send trait：
			若所有子类型皆实现 Send trait 则其本身亦实现 Send Trait
		a. " NoSend::default(); "则异步 foo 编译转化的 Future 表象:
			   struct Foo {
					f: Future,     // impl Send Trait
				}
		b. " let x = NoSend::default();"则异步 foo 编译转化的 Future 表象：
				struct Foo {
					x: NoSend,      // not impl Send Trait
					f: Future,      // impl Send Trait
				}
	 */
	// NoSend::default();

	/* 编译报错：
	   error: future cannot be sent between threads safely
		  fn required_send(_: impl Send) {}
		  						 ---- required by this bound in `required_send`
		  	required_send(foo());
		  	^^^^^^^^^^^^^ future returned by `foo` is not `Send`
	      = help: within `impl std::future::Future`, the trait `std::marker::Send` is
	        not implemented for `std::rc::Rc<()>`
	    分析：源于表象结构体" struct Foo {
								x: NoSend,      // not impl Send Trait
								f: Future,      // impl Send Trait
							}
				           " 之" x: NoSend "字段未实现 Send trait
		// 问题：为何直接使用" NoSend::default(); "却可正常编译执行呢[???]
				 难道因为没有使用" let x = "绑定到变量故没有变量字段?[待理解]
		   [自]若定义" let x = "接收则后续可能会使用到 x (须保证其有效性[生命周期相关])
	 */
	//let x = NoSend::default();

	{
		let x = NoSend::default();
		/* 出了作用域其 x 自动析构(即不会成为相应 Future 表象结构体"
		   struct Foo { ... } "的字段)故可正常编译执行
		*/
	}

	bar().await;
}

fn required_send(_: impl Send) {}

fn main() {
	required_send(foo());
    println!("Hello, world!");
}
