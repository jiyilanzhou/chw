
/*
1、创建一个空 String

2、通过字面值创建一个 String
    2.1、使用 String::from()
    2.2、使用 str 的方式

3、更新 String
    3.1、push_str
    3.2、push
    3.3、使用" + "合并字符串
    3.4、使用 format!

4、String 索引

5、str 索引

6、遍历
    6.1、chars
    6.2、bytes

*/

fn main() {
    //1、创建空的可变数据(空的不可变数据无多大意义[因不可写])
    let mut s0 = String::new(); // 通过 String::new 创建
    /* push_str 源码：
          pub fn push_str(&mut self, string: &str) {
              self.vec.extend_from_slice(string.as_bytes())
          }
     */
    s0.push_str("hello");
    println!("s0 = {}", s0);
    // 通过 String::from 创建
    let s1 = String::from("init some thing");
    println!("{}", s1);
    //2、通过字面值创建字符串
    let s1 = "init some thing".to_string();
    println!("{}", s1);

    //3、更新 String
    //3.1、push_str (源码" pub fn push_str(&mut self, string: &str){ //... } ")
    let mut s2 = String::from("hello");
    s2.push_str(", world");
    let ss = " !".to_string();
    s2.push_str(&ss);
    println!("{}", s2);
    println!("ss = {}", ss);
    //3.2、push (源码:" pub fn push(&mut self, ch: char){ //... } ")
    let mut s2 = String::from("tea");
    s2.push('m');
    s2.push('道');
    //s2.push('mx'); // 飘红报错:" too many characters in char literal "
    //s2.push("x");  // 飘红报错:" mismatched types [E0308] expected `char`, found `&str` "
    println!("{}", s2);

    //3.3、使用" + "合并字符串
    let s1 = "hello".to_string();
    let s2 = String::from(", world");
    /*a. 加法" + "源码：// RHS ： right hand side 右侧
           #[doc(alias = "+")]
           pub trait Add<Rhs=Self> {
               /// The resulting type after applying the `+` operator.
               #[stable(feature = "rust1", since = "1.0.0")]
               type Output;
               /// Performs the `+` operation.
               #[must_use]
               #[stable(feature = "rust1", since = "1.0.0")]
               fn add(self, rhs: Rhs) -> Self::Output;
           }
      b. String 实现 Add 源码：
           impl Add<&str> for String {
                type Output = String;
                #[inline]
                fn add(mut self, other: &str) -> String {
                    self.push_str(other);
                    self
                }
            }
    */
    let s3 = s1 + &s2; // 将 s1 所有权移至 s3
    println!("s3 = {}", s3);
    // println!("s1 = {}", s1); // 编译报错(因 s1 所有权已转移)
    println!("s2 = {}", s2);

    //3.4、使用 format!
    let s341 = String::from("tic");
    let s342 = String::from("tac");
    let s343 = String::from("toe");
    // format! 类比 println! 即其后仍可使用 s341,s342,s343
    let s344 = format!("{}-{}-{}", s341, s342, s343);
    println!("s344 = {}", s344);
    println!("s341 = {}", s341);
    println!("s342 = {}", s342);
    println!("s343 = {}", s343);

    //4、String 索引
    let s4 = String::from("hello");
    /* String 不可被索引:
        因其使用 unicode 编码如 let s4 = String::from("你好"); 占用 6 个字节
        而使用的索引获取未必正好是表示一个字符的边界索引故 String 禁止索引(安全)
    */
    // 编译错误:"the type `std::string::String` cannot be indexed by `{integer}`"
    //let s41 = s4[0];
    println!("s4.len = {}", s4.len());
    let s4 = String::from("你好");
    println!("s4.len = {}", s4.len());
    // 编译错误:"the type `std::string::String` cannot be indexed by `{integer}`"
    //let s41 = s4[0];

    //5、str 索引
    let hello = "你好";
    // str 允许索引(但编译仍然有可能会出错如"  let h5 = &hello[0..=3]; ")
    let h5 = &hello[0..3];
    println!("h5 = {}", h5);

    /* //编译正常：运行时错误
       // thread 'main' panicked at 'byte index 4 is not a char boundary;
       let h6 = &hello[0..=3];
       println!("h6 = {}", h6);
     */

    //6、遍历
    //6.1、chars
    for c in s4.chars() {
        println!("c = {}", c);
    }
    println!("+++++++++++++++");
    //6.2、bytes
    for b in s4.bytes() {
        println!("b = {}", b);
    }
}