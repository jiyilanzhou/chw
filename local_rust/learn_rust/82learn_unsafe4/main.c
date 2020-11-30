
/* C 语言调用 Rust crate :
1. ( Rust 部分)创建 crate lib (如" foo ")
    a. 在"..\foo\src\lib.rs "编译库代码并使用" extern "导出
           #![crate_type = "staticlib"]
           #[no_mangle]
           // 使用 extern 导出
           pub extern fn foo() {
               println!("use rust");
           }
    b. 在" \foo\Cargo.toml "配置库
           [lib]
           name = "foo"
           crate-type = ["staticlib"]
    c. 进入" foo " 编译
        Administrator@CHW .../82learn_unsafe4/foo (master)
        $ cargo build
    d. 在" ...\82learn_unsafe4\foo\target\debug "下生成
        " libfoo.a "及" foo.lib "文件
        // 将生成的" libfoo.a "文件拷贝至" main.c "同目录)

2. ( C 语言部分)
    a. 使用 extern 导入
    b. 按需调用
    c. 编译" gcc -o main main.c libfoo.a -lpthread -ldl "
       (未能编译：待解决)

*/

// 使用 extern
extern void foo();

int main() {
	foo();
	return 0;
}
