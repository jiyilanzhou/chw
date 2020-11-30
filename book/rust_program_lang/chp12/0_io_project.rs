
/*
0. I/O 项目：编写一个命令行程序
    a. 开发一个能够和文件系统交互并处理命令行输入、输出的工具
    b. grep : globally search a regular expression and print
    c. grep 最简单的使用场景即是在特定文件中搜索指定的字符串

1. 读取参数值
   使用标准库提供的" std::env::args "函数：返回迭代器(iterator),再通过调用迭代器
   " collect "方法生成所有产出值的集合(如动态数组)

2. 读取文件
   标准库" std::fs::read_to_string(filename) "打开文件并并使用 " Result<String> "
   返回文件内容

3. 修复错误处理逻辑
    a. "unwrap_or_else(closure)"被定义于标准库 Result<T,E> 中
    b. 标准库" std::process::exit "会立即终止程序运行并将指定的错误码返回给调用者

4. 将错误信息打印到标准错误而不是标准输出
   大部分终端都提供两种输出：用于输出一般信息的标准输出(standard output，stdout)
   以及用于输出错误提示信息的标准错误(standard error，stderr)。这种区分可使用户
   将正常输出重定向到文件的同时仍然将错误提示信息打印到屏幕上

*/