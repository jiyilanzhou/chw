
/*
2、过程宏
   a. 过程宏接收输入的 Rust 代码并于其上进行操作，然后产生输出代码。
      区别于声明宏(声明宏匹配对应模式后以另一部分代码替换当前代码)
   b. 定义过程宏的函数接受一个 Tokenstream 作为输入并产生一个 Tokenstream
      作为输出(亦是宏的核心:宏所处理的源代码组成了输入 Tokenstream 同时宏
      生成的代码是输出 Tokenstream )如:
          use proc_macro
          #[some_attribute]
          pub fn some_name（input: TokenStream）-> TokenStream {
              // ...
          }
    c. " #[meta] "外部属性；" #![meta] "内部属性

 */

#[derive(HelloMacro)]   // 过程宏(" derive 宏 ")
struct Main;

// 定义 trait
pub trait HelloMacro {
    fn hello_macro();
}

// extern : 声明使用外部 crate
extern crate proc_macro;
use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    /* 解析为 DeriveInput 结构体：
          a. " syn "中的" parse_derive_input "函数获取一个 Tokenstream
             并返回一个表示解析出 Rust 代码的 DeriveInput 结构体(对应代码
             " syn::parse(input).unwrap(); ")
          b. DeriveInput 结构体相关内容大致如下
                DeriveInput {
                    // --snip--
                    ident: Ident {
                        ident: "StructName",
                        span: #0 bytes(95..103)
                    },
                    data: Struct(
                        DataStruct {
                            struct_token: Struct,
                            fields: Unit,
                            semi_token: Some(
                                Semi
                            )
                        }
                    )
                }
    */
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        // " #name "的含义 : 将输入的 #name 替换为变量 name 中的值[???]
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, in my macro, my name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}

/* derive 宏：
   在结构体上标注则编译器自动为其实现相应 trait 如
        #[derive(Debug)]
        struct SN{}
    // 编译器自动为结构体生成实现" Debug trait "的代码

*/
fn main() {
    Main::hello_macro();
}
