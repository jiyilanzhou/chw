
// 用链表实现栈
#[derive(Debug)]
struct StackNode<T> {
    data: T,
    // 因可能为 None 故使用 Option
    next: Option<Box<StackNode<T>>>,    // Box<T> 相当于" *T "(大小固定)
}

#[derive(Debug)]
struct Stack<T> {
    top: Option<Box<StackNode<T>>>,
}

impl <T> Stack<T> {
    fn new() -> Stack<T> {
        Stack{top: None}
    }

    fn push(&mut self, data: T) {
        let mut node = StackNode{data: data, next: None};
        let next = self.top.take();
        node.next = next;
        self.top = Some(Box::new(node));
    }

    fn pop(&mut self) -> Option<T> {
        let node = self.top.take();
        match node {
            None => None,
            Some(mut x) => {
                self.top = x.next.take();
                Some(x.data)
            }
        }
    }
}

fn main() {
    let mut ss = Stack::new();
    ss.push(1);
    ss.push(2);
    ss.push(3);
    println!("after push, ss: {:#?}", ss);

    // ss.pop();
    // ss.pop();
    // ss.pop();
    for _ in 0..4 {
        if let Some(data) = ss.pop() {
            println!("data: {}", data);
        } else {
            println!("empty");
        }
    }

    println!("after pop, ss: {:#?}", ss);
}