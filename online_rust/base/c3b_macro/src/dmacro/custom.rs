/*
0. 声明宏

 */
/* 将此宏导出以供外部(如 main 函数)调用
   没有 #[macro_export] 的宏则外部不可见
 */
#[macro_export]
macro_rules! cat {
    ()=>(
            println!(" cat! ");
        )
}

#[warn(dead_code)]
pub fn func(){}