

/*
0. 模拟 substrate


 */
// Trait
trait Trait {}

struct Balance<T> {
    name: T,
}

impl<T: Trait> Balance<T> {
    fn func() {
        println!(" impl B")
    }
}

#[derive(Debug)]
struct Module<T> {
    id: T,
}

impl<T: Trait> Module<T> {
    fn hello(&self) {
        <Balance<T>>::func();
        Balance::<T>::func();
    }
}

fn main() {
    let s = Module { id: 1 };
    // Module::<i32>::hello(&s);    // 编译错误(待解决[???])
}