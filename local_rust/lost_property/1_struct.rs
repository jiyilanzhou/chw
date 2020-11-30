use std::mem;

/* 内存对齐(属性最大占用的最小整数倍)
   " std::mem::size_of "查看
 */
struct A {
	a: u8,    // 1 byte
	//padding // 3 byte
	b: u32,   // 4 byte
	c: u16,   // 2 byte
	//padding // 2 byte
}

struct B {
	a: u32,
	b: u32,
	c: u32,
}

fn main() {
	let a = A {a: 1, b: 2, c: 3};
	println!("size = {}", mem::size_of::<A>()); // Console:" 8 "
	println!("size = {}", mem::size_of::<B>()); // Console:" 12 "
	println!("size = {}", mem::size_of_val(&a));// Console:" 8 "
}