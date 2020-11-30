
/*
0. Develop Issue
   提交代码前务必执行" cargo build --release "及" cargo test "通过后方能提交

1. 搜索实现
    command + shift + F （如正则搜索" impl.*AssetTrait "）

2. 查看 Trait 定义的 method 实现
    option + command + B

3. let 声明语句位置表达式即使是" {} "也必须添加分号" ; "
   因为" {} "返回的是其内最末表达式的值而 let 声明是不可反驳模式
   故必须在" {} "后添加分号" ; "声明即
       let variable = { ... };      // 末尾分号";"不可省

4. "解引用"是否可用取决于是否已实现 deref 及是否是引用

5. vec<struct> 按照某个字段排序
   注意 vec 下函数名的意义如" is_sort_by(...) -> bool  "就不是用于排序的,源于：
    a. " is "前缀一般用于判断
    b. 看其返回值为" bool "一般亦是用于判断
    c. 使用" sort_by "或" sort_by_xx "
*/

/*
6. substrate 中的整数转换为浮点
	a. 依赖
		sp-arithmetic = "2.0.0"
	b. 文档
		https://docs.rs/sp-arithmetic/2.0.0/sp_arithmetic/
		

 */
fn main() {

}
