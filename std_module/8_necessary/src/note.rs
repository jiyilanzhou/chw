/*
0. 惰性静态初始化 lazy_static 包(tao[p325])

1. #[macro_use] 使用


2. 闭包

3. 常用设计模块(tao[p233])

4. Cargo.toml 文件格式(tao[p331])

5. union 联合体(tao[p480])

6. 子类型与型变(tao[p489])

Unsafe

*/
use std::collections::hash_map::HashMap;

fn mutate(k:(char,char),f: fn((char,char)) -> (char, char)) ->(char,char){
    f(k)
}

fn main() {
    let mut map = HashMap::new();
    map.insert(0, ('a', 'b'));
    println!("{:?}", map);

    let c = mutate(('a','b'),|v|(v.0,'c'));
    let c = mutate(('a','b'),|(a,b)|(a,'c'));

    println!("{:?}", c);
}