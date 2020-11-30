
// 使用" .. "忽略(但不能有歧义)
fn main() {
    let numbers = (1, 2, 3, 4, 5, 6);
    match numbers {
        // 全忽略
        //(..)=>println!("Hello World!"),

        // 部分忽略
        (first, .., last) => {
            println!("first: {}, last: {}", first, last);
        },
    };

    /* (有歧义)编译报错：每个元组模式只能使用一次
        error: `..` can only be used once per tuple pattern
            (.., second, ..) => {
             --          ^^ can only be used once per tuple pattern
             |
             previously used here
        // 每个元组模式" .. "仅能使用一次(否则会产生"歧义")
    */
    /* 暂且注释：以通过编译
        match numbers {
            (.., second, ..) => {
                println!("{}", second);
            },
        };
    */

}