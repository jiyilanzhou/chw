
/*
0. 队列介绍：
    队列是一种只允许在一端进行插入，而在另一端进行删除操作的数据结构
    即先进先出(FIFO)

1. 队列操作
    初始化
    入队
    出队
    队列长度

 */

#[derive(Debug)]
struct Queue<T> {
    qdata: Vec<T>,
    capacity: usize,
}

impl<T> Queue<T> {
    //fn new() -> Self {
    fn new(size: usize) -> Self {
        Queue {
            qdata: Vec::with_capacity(size),
            capacity: size,
        }
    }
    // 入队
    fn entry_queue(&mut self, item: T) -> Result<(), String> {
        if self.qdata.len() == self.capacity {
            /* 此处不可直接使用:" Err("No space in queue!".to_string()) "
                  原因分析：if 未携带 else 分支故未必能执行，故默认返回 ()
               # 编译错误：
               // Err("No space in queue!".to_string())
                                          ^^^^ expected `()`,found enum
                                                   `std::result::Result`
               # 另外即使拥有 else 分支亦必须置于相应代码块末尾，否则也无法
                 正常返回(因其后执行返回的是后续执行的结果)如将其修正为:
                        if self.qdata.len() == self.capacity {
                            Err("No space in queue!".to_string())
                        }else{
                            Ok(())
                        }
                  且置于" fn entry_queue... { } "函数体末尾才可正常执行
             */
            return Err("No space in queue!".to_string());
        }
        self.qdata.push(item);
        Ok(())
    }
    // 出队
    fn quit_queue(&mut self) -> Option<T> {
        let size = self.qdata.len();
        if size > 0 {
            let v = self.qdata.remove(0);
            Some(v)
        } else {
            None
        }
    }
    // 长度
    fn size(&self) -> usize {
        self.qdata.len()
    }
}

fn main() {
    let mut q = Queue::new(2);
    q.entry_queue(1);
    println!("1 enqueue!");
    q.entry_queue(2);
    println!("2 enqueue!");

    if let Err(error) = q.entry_queue(3) {
        println!("enqueue error: {}", error);
    }

    println!("++++++++++++++++++");

    println!("size: {}", q.size());
    println!("q: {:#?}", q);

    println!("++++++++++++++++++");

    for _ in 0..3 {
        if let Some(data) = q.quit_queue() {
            println!("data: {}", data);
        } else {
            println!("queue empty!");
        }
    }
}