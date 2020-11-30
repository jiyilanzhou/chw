
// 使用"#[macro_export]"导出宏(以便外部可用)
#[macro_export]
macro_rules! my_vec {       // 声明宏(模仿" vec！ "宏)
    // 匹配"0"个或"多个"表达式(类似"match")
    ($($x: expr), *) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}