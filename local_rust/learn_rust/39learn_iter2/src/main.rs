
// 自定义迭代器

struct Counter { // 结构体及其实现
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter {count: 0}
    }
}
// 实现迭代功能：Rust 中的迭代器不会造成额外的性能开销
impl Iterator for Counter {
    type Item = u32;    // 关联类型
    // 问题：为何不可直接关联为具体类型[?]:
    // 如" fn next(&mut self) -> Option<Self::u32>{//...} "
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let mut counter = Counter::new();
    for i in (0..6) {
        if let Some(v) = counter.next() {
            println!("i = {}, v = {}", i, v);
        } else {
            println!("i = {}, at end", i);
            break;
        }
    }
}
