
// boilerplate[ˈbɔɪləpleɪt]n.样板
// expander[ɪk'spændɚ]n.扩充器     // transcriber[træns'kraibə]n.转换器
/*
0. 宏

9.1 简介 macro
    a. Rust 中的" 宏(macro) "是一种编译器扩展，其调用方式为" some_macro!(...) "。
       宏亦可通过"some_macro![...]"或"some_macro!{...}"调用,只要括号能正确匹配即可
    b. 可把" 宏 "视为" 元编程 "的一种方式，它是一种" 生成程序的程序 "

9.1.1 实现编译阶段检查

9.1.2 实现编译期计算

9.1.3 实现自动代码生成

9.1.4 实现语法扩展
    如使用 vec! 宏初始化动态数组" let v = vec![1,2,3]; "

9.2 示范型宏
    a. 格式
           macro_rules! macro_name{
                // 左侧 expander 为宏扩展的语法定义；
                // 右侧 transcriber 为宏扩展的转换机制
                expander => { transcriber }
           }
    b. 语法定义标识符以 $ 开头(类型支持 item,block,stmt,pat,expr,ty,ident,path,tt)
    c. (类似正则表达式)" + "(模式)代表一个或多个重复；" * "(模式)代表零个或多个重复
       且重复部分需使用括号括起来并加上逗号分隔符。

*/
// 实现 hashmap!['A'=>0,'B'=>1,'C'=>2] 初始化功能
macro_rules! hashmap {
   ($($key:expr=>$val:expr),*)=>{
        {
          let mut map = ::std::collections::HashMap::new();
          $(
            map.insert($key,$val);
          )*
          map
       }
   }
}
/* // 检查宏展开执行效果命令
   Administrator@CHW MINGW64 /e/project/src (master)
   $ rustc -Z unstable-options --pretty=expanded main.rs
       let counts =
            {
                let mut map = ::std::collections::HashMap::new();
                map.insert('A', 0);
                map.insert('B', 1);
                map.insert('C', 2);
                map
            };

*/
fn main() {
    let counts = hashmap!['A'=>0,'B'=>1,'C'=>2];
    println!("{:?}", counts);
}

/*
9.3 过程宏(procedural macro) ：p105~108
    对于一些简单的宏，"示例型"宏(声明宏)即可满足。但更为复杂的逻辑需要通过更为复杂的方式
    实现，即" 过程宏 "(直接用 Rust 语言编写，相当于一个编译器插件[但编译器插件最大的问题
    是其依赖于编译器的内部实现方式：一旦编译器内部有所变化则对应的宏就有可能出现编译错误，
    需要修改。因此 Rust 中的宏一直难以稳定])

*/