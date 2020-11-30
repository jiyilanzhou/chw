
/*
6.3 迭代器
    迭代器(Iterator)模式(亦称"游标[Cursor]模式")：使用迭代器可极大简化数据操作，其提供的
    方法可顺序访问集合容器中的元素(而又无需暴露该容器的内部结构和实现细节)

6.3.1 外部迭代器和内部迭代器(p194[*])
    a. 外部迭代器(External Iterator)：亦称主动迭代器(Active Iterator)独立于容器之外，通过
       容器提供的方法(如"next"[即所谓的"游标"])来迭代下一个元素并需要考虑容器内可迭代的剩余
       数量来进行迭代。外部迭代器的一个重要特点是"外部可以控制整个遍历进程"(如 Python、Java
       及 C++ 语言中的迭代器就是外部迭代器)
    b. 内部迭代器(Internal Iterator)：内部迭代器则通过迭代器自身来控制迭代下一个元素，外部
       无法干预，意味着只要调用了内部迭代器并通过闭包传入相关操作，就必须等待迭代器依次为其
       内部的每个元素执行完相关操作后方可停止遍历(如 Ruby 语言中典型的内部迭代器 each )
    c. 早期的("1.0"版本之前) Rust 提供的是内部迭代器，而内部迭代器无法通过外部控制迭代进程
       再加上 Rust 的所有权系统导致使用起来很复杂如
           trait InIterator<T: Copy> {
                fn each<F: Fn(T) -> T>(&mut self, f: F);
            }
            impl<T: Copy> InIterator<T> for Vec<T> {
                fn each<F: Fn(T) -> T>(&mut self, f: F) {
                    let mut i = 0;
                    while i < self.len() {
                        self[i] = f(self[i]);
                        i += 1;
                    }
                }
            }
            // 使用 Rust 提供的内部迭代器
            fn main(){
                let mut v = vec![1,2,3];
                v.each(|i| i * 3);              // 调用内部迭代器(外部无法控制迭代进程)
                assert_eq!([3, 6, 9], &v[..3]);
            }
    d. 外部迭代器 : for 循环( Rust 中的 for 循环其实是一个语法糖)如
            fn main() {
                let v = vec![1, 2, 3, 4, 5];
                for i in v {
                    println!("{}", i);
                }
            }
            // for 循环的等价代码(简言之：for 循环是利用迭代器模式实现的一个语法糖)
            fn main() {
                let v = vec![1, 2, 3, 4, 5];
                {  // 等价于for循环的scope
                    let mut _iterator = v.into_iter();
                    loop {
                        match _iterator.next() {
                            Some(i) => {
                                println!("{}", i);
                            }
                           None => break,
                       }
                   }
               }
            }

6.3.2 Iterator trait (p196[*])      // hint[hɪnt]v&n.提示,暗示,示意
    a. 通过实现 Iterator trait 创建自定义迭代器
    b. Iterator trait 提供的 size_hint 方法源码

6.3.3 IntoIterator trait 和迭代器
    a. IntoIterator (从指定类型转为迭代器) ：
       与 FromIterator (从迭代器转为指定类型) 互为反操作
    b. slice 类型数组循环、Iter 及 IterMut 迭代器源码示例(p201[*])
        Iter 迭代器源码:
            pub struct Iter<'a, T: 'a> {
                ptr: *const T,
                end: *const T,
                _marker: marker::PhantomData<&'a T>,
            }
        IterMut 迭代器源码:
            pub struct IterMut<'a, T: 'a> {
                ptr: *mut T,
                end: *mut T,
                _marker: marker::PhantomData<&'a mut T>,
            }

6.3.4 迭代器适配器
    a. 通过适配器模式可将一个接口转换成所需的另一个接口，适配器模式使得接口不兼容的
       类型在一起工作(适配器别名:包装器[Wrapper])。Rust 在迭代器基础上增加了适配器
       模式，极大地增强迭代器的表现力
    b. Map 适配器(p203[*])
       (1) 迭代器 map 方法源码
       (2) 迭代器适配器 Map 源码( Map 是一个泛型结构体)
    c. 常用适配器
        (1) Map
        (2) Chain
        (3) Cloned
        (4) Cycle
        (5) Enumerate
        (6) Filter
        (7) FlatMap                 // flat[flæt]adj&n.平坦(的)
        (8) FilterMap
        (9) Fuse
        (0) Rev

6.3.5 消费器 Consumer (p207[*])
    a. Rust 中消费器都是惰性的(即不会自动发生遍历行为[除非调用 next 方法消费其中数据])
    b. 最直接消费迭代器数据的方法就是 for 循环(会隐式调用迭代器的 next 方法)
    c. 常见消费器
       (1) any
       (2) fold
       (3) collect

6.3.6 自定义迭代器适配器(p211[*])
    a. 定义迭代器适配器(如" Step<I> ")
    b. 为定义的适配器实现 Iterator
    c. 创建相应方法(如"step"方法)来产生定义的适配器(如"Step"适配器)
    d. 为所有迭代器实现创建的方法(如"step"方法)
    e. 应用迭代器适配器(如"Step"适配器)

*/