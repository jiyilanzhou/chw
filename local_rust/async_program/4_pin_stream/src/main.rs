
/*
0. Stream (图" 1_Stream.png ")
    a. Stream 是由一系列的 Future 组成，可从 Stream 读取各个 Future 的结果
       直到 Stream 结束。
    b. Stream trait 定义：
        trait Stream {
            type Item;
            fn poll_next(self: Pin<&mut Self>, lw: &LocalWaker)
                -> Poll<Option<Self::Item>>;
        }

 */

fn main() {

}