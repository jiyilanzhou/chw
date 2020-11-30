
/* 先在" .../18learn_crate2/Cargo.toml "的" [dependencies] "下
   引入" mylib = {path = "./mylib"} "配置
*/

/* // 引入作用域
     use mylib::factory::produce_refrigerator;    // Rust 提倡到使用的上一层级
     use mylib::factory::produce_refrigerator::produce_re;    // 更为精确的层级
     use mylib::factory::produce_washing_machine;
     use mylib::factory::produce_washing_machine as A; // 多模块内函数同名时使用别名
 */
use mylib::factory::*;  // " * "代表上一级目录下的所有模块

fn main() {
    // 绝对路径
    mylib::factory::produce_refrigerator::produce_re();
    // 使用 use
    produce_refrigerator::produce_re();
    // 使用更为精确的层级
    // produce_re();

    produce_washing_machine::produce_re();
    //A::produce_re(); // 多模块内函数同名时使用别名(使用指定模块内的函数)

}