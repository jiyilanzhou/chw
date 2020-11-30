
/*
0. DSL ：Domain Specific Language

1. 宏根据模式进行匹配

*/

// 声明宏
macro_rules! calculate {
    (
        // 强制使用标识符
        eval  $e:expr
    ) => {
        let val:usize = $e;
        println!("{} = {}",stringify!($e), val);
    };
    (
        eval $e:expr,$(eval $es:expr),+
    ) => {
        calculate!{eval $e} // replace{}
        calculate!{ $(eval $es),+ }  // replace{}
    };
}

fn main() {
    // 若宏内声明" val => 表达式＂故展开调用时须与宏声明一致
    calculate!(eval 3);     // Console:" 3 = 3  "
    calculate!(eval 3*6);    // Console:" 3 * 6 = 18 "
    // calculate!(eval => 6*8); // 标识符不匹配则报错
    // calculate!(val 6*8); // 标识符不匹配则报错

    calculate!(
        eval 6*8,
        eval 3
    );
    /* console:
          6 * 8 = 48
          3 = 3
     */
}