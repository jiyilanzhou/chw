
//1、类型别名
type Kilometers = i32;  // " Kilometers "是" i32 "的同义词(即完全一样)

fn main() {
    let x: i32 = 5;
    // 使用别名
    let y: Kilometers = 6;

    let r: i32 = x + y;
    println!("x + y = {}", r);
}

/*
类型别名用途 ：主用于减少重复
1. " Box<dyn Fn() + Send + 'static> "类型使用示例
  a. 使用全名：如" Box<dyn Fn() + Send + 'static> "类型用于如下代码
     let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
     fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {//...}
     fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {//...}
  b. 使用类型别名：如" type Thunk = Box<dyn Fn() + Send + 'static>; "
     let f: Thunk = Box::new(|| println!("hi"));
     fn takes_long_type(f: Thunk) {//...}
     fn returns_long_type() -> Thunk {//...}

2. 标准库" std::io::Error "结构体代表所有可能的 I/O 错误
   a. 使用全名
      use std::io::Error;
      use std::fmt;
      pub trait Write {
         fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
         fn flush(&mut self) -> Result<(), Error>;
         fn write_all(&mut self, buf:&[u8]) -> Result<(), Error>;
         fn write_fmt(&mut self,fmt:fmt::Arguments)->Result<(),Error>;
      }
   b. 使用别名  // " result<T, E> "中 E 放入了" std::io::Error "
      type Result<T> = std::result::Result<T, std::io::Error>;
      pub trait Write {
          fn write(&mut self, buf: &[u8]) -> Result<usize>;
          fn flush(&mut self) -> Result<()>;
          fn write_all(&mut self, buf: &[u8]) -> Result<()>;
          fn write_fmt(&mut self, fmt: Arguments) -> Result<()>;
      }

*/