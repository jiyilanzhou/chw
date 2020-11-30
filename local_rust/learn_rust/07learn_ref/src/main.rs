
// 所有权
fn gives_ownership() -> String {
    let s = String::from("hello");
    s
}

fn takes_and_gives_back(s: String) -> String {
    s
}

fn main_000() {
    /* 将"gives_ownership()"内建 s 所有权移至 s1 ：
          因为 string 为堆空间数据故函数返回值赋值于 s1 即是将引用移至 s1 后
          函数内部的 s 失效
    */
    let s1 = gives_ownership();
    println!("s1 = {}", s1);

    // 创建可变 s2 变量
    let mut s2 = String::from("hello");
    // 函数获取 s2 所有权，最终又将返回值所有权移至 s3 后 s2 生效
    let s3 = takes_and_gives_back(s2);
    // println!("{}",s2);  // 编译错误:" error[E0382]: borrow of moved value: `s2` "

    /*
       仅使用 s2 进行某些操作后续仍需使用 s2 但鉴于借用规则故不得不又调用将其返回(如此相互
       调用返回较为繁锁:解决方案[引用])
     */
    //函数获取 s3 所有权，最终又将返回值所有权移至 s2 后 s3 生效
    s2 = takes_and_gives_back(s3);
    //println!("s3 = {}", s3);  // 编译错误:" error[E0382]:borrow of moved value:`s3` "
    println!("s2 = {}", s2);

}

/*引用 & :
  创建一个指向但并不拥有值的引用(因未拥有这个值，故当引用离开其值指向的作用域后不会被丢弃)

*/
fn calcute_length(s: &String) -> usize {
    s.len()
}

/*借用 : & mut
    fn modify_s(s: &String) {
        s.push_str(", world");
    }
  // 编译报错：
    error[E0596]: cannot borrow `*s` as mutable, as it is behind a `&` reference
        fn modify_s(s: &String) {
                       ------- help: consider changing this to be a mutable reference:
                                                            `&mut std::string::String`
                       // 帮助提示: 考虑将其更改为可变引用(即"借用")
          s.push_str(", world");
          ^ `s` is a `&` reference,so the data it refers to cannot be borrowed as mutable
          // `s`是一个`&`引用，因此它引用的数据不能作为可变数据借用
*/
fn modify_s(s: &mut String) {
    s.push_str(" world");
}

fn main() {
    let s = String::from("hello");
    /* 飘红报错：Rust 是强类型语言
            mismatched types [E0308] expected `&String`, found `String`
            // 给予的实参不匹配函数声明的形参"s: &String"
       编译报错：类型匹配错误
        error[E0308]: mismatched types
          let len = calcute_length(s);
                                   ^
                                   |
                                   expected reference, found struct `std::string::String`
                                   help: consider borrowing here: `&s`
           = note: expected type `&std::string::String`
                      found type `std::string::String`
    */
    //let len = calcute_length(s);
    let len = calcute_length(&s);
    println!("len = {}", len);      // Console:" 5 "
    println!("s = {}", s);  // Console:" hello "

    /*
    // 错误使用方式1：将不可变引用当作"借用"(即可变引用)
    let s1 = String::from("hello");
    modify_s(&mut s1); // 飘红报错:" Cannot borrow immutable local variable `s1` as mutable "

    // 错误使用方式2：实参类型未匹配函数形参声明
    let mut s1 = String::from("hello");
    // 飘红报错:" mismatched types [E0308] expected `&mut String`, found `String` "
    modify_s(s1);
   */
    let mut s1 = String::from("chw");
    let ms = &mut s1;
    modify_s(ms);
    println!("{}", &mut s1); // Console:" chw world "
    println!("{}", s1);      // Console:" chw world "

    let mut s1 = String::from("683");
    let s = &s1;
    let ms = &mut s1;
    modify_s(ms);
    /* 编译错误 1 ：  // 不能将 's1' 作为可变项借用，因为它也作为不可变项借用
       error[E0502]: cannot borrow `s1` as mutable because it is also borrowed as immutable
            let s = &s1;
                    --- immutable borrow occurs here        // 不可变借用
            let ms = &mut s1;
                     ^^^^^^^ mutable borrow occurs here     // 可变借用
            println!("s = {}",s);
                              - immutable borrow later used here //不可变借用后使用
       // 分析：因 ms 定义为 s1 的可变引用故 modify_s(ms) 可能更改 s1 指向的数据。而 s 又定义
                为 s1 的不可变引用故不允许 s1 的指向改变。因此前后矛盾故报错
    */
    //println!("s = {}",s);   // 暂且注释：以通过编译

    println!("s1 = {}",s1);  // Console:" 683 world "

    /* 编译错误 2 ： // 不能将's1'作为不可变项借用，因为它也作为可变项借用
       error[E0502]: cannot borrow `s1` as immutable because it is also borrowed as mutable
           let ms = &mut s1;
                    ------- mutable borrow occurs here
           println!("s1 = {}",s1);
                              ^^ immutable borrow occurs here
           println!("ms = {}",ms);
                              -- mutable borrow later used here
        // 分析：因" s1 同时作为可变引用及不可变引用 "
        // 解决方案: 注释 println!("s1 = {}",s1); 或注释 println!("ms = {}",ms);
    */
    // println!("ms = {}",ms);     // 暂且注释：以通过编译

    // r1、r2 为引用
    let r1 = &s1;
    let r2 = &s1;
    //println!("{}, {}", r1, r2); // Console:" 683 world, 683 world "
    // r3 为借用
    let r3 = &mut s1;
    r3.push_str(", chw");
    //println!("{}",r3);      // Console:" 683 world, chw "
    /* 编译报错：
       error[E0502]: cannot borrow `s1` as mutable because it is also borrowed as immutable
           let r1 = &s1;
                    --- immutable borrow occurs here
           let r3 = &mut s1;
                    ^^^^^^^ mutable borrow occurs here
           println!("{},{},{}",r1,r2,r3);       // 即有了"借用"后不可再使用之前的"引用"
                               -- immutable borrow later used here
        // 有了可变的"借用"后不可再使用之前的不可变"引用"：因为借用后有可能修改，所以再使用之前的
           不可变引用其指向的数据可能改变(即与预期的不可变引用值相悖)。故编译报错(规避风险)
        // 解决方案：在"借用"(即可变引用)后不再使用不可变引用
    */
    println!("{},{},{}",r1,r2,r3);

}