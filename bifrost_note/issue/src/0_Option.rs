/*
0. Option

1. 在标准库（std）中做 Option<T>枚举类型，用于处理可能 “不存在” 的情况。
    Some(T)：找到一个属于 T 类型的元素
    None：找不到相应元素
    // 选项可以通过 match 显式处理或使用 unwrap 隐式地处理(隐式处理要么返回 Some 内部元素要么 panic )
    // 使用 expect 方法相对于 unwrap 优势即为可自定义 panic 信息




 */