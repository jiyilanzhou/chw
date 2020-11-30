
// yank[jæŋk]v&n.猛拉
/* 文档注释: " /// "及" //! "注释
    命令" cargo doc "：
    命令" cargo doc --open ": 生成并打开文档
    命令" cargo test ": 自动对"文档注释"中用例进行测试
    "文档注释"一般用于" panic! 场景的说明、返回 Result 类型的描述"及" unsafe 代码"

*/

//! My Crate
//! 
//! 'my_crate'  makes performing certain calcuations more convenient
//!
/// Add one to the number given 
/// 
/// #Example
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, mylib::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

/*
crate 的发布与撤回（此部分没有实际操作过，只讲步骤）
（1）创建 Crates.io 账号:通过 Github 账户注册，并通过 cargo login**** 来登陆
（2）发布前需要在 Cargo.tom 中增加描述
    name = package name
    version
    license="MIT"   # Linux 基金会的 Software Package Data Exchange（SPDX）
                    # 列出了可使用的标识符
    authors =["chw"]
    description = "some thing descript the package"
    # 运行 cargo publish 来发布。
（3）撤回至指定版本      // yank[jæŋk]v&n.猛拉
    cargo yank --vers 0.1.0

*/