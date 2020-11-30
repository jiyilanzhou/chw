
// abbreviate[əˈbriːvieɪt]v.缩写,简化
// abbreviation[əˌbriːviˈeɪʃn]n.缩写,简称
// " abbr / abbrev "为 abbreviation 缩写
/*
std::net

std::fs
std::fs::File

std::io

std::result::Result

// ops:" Overloadable operators "
use std::ops::Deref     // 智能指针(解引用)

std::mem::drop          // 在作用域结束前释放资源

// Box 无需导入
// cell[sel]n.细胞、单元格、储存格
std::cell::RefCell;
std::rc::{Rc, Weak};  // 引用计数

*/

/* abbr 缩写,简称
0. Rng
   random number generator  随机数生成器

*/

/* 知识点 knowledge
0. 查阅依赖文档
   " cargo doc --open "可在本地构建一份有关所有依赖的文档并自动在在浏览器
   中打开以供查阅

1. 引用与借用
    a. & 代表的就是"引用"语义，允许在不获取所有权的前提下使用值。同理函数
       签名中 & 用来表明参数类型是引用([自]类比 Golang 中的指针" * ")
    b. 通过引用传递参数的方式也被称为"借用(borrowing)"


*/