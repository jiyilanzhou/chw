
// quote[kwəʊt]v&n.引用
/* 创建" xxx_derive lib "：
   a. 在" hello_macro "下(并以其为前缀名[推荐])创建相应 lib
      如"  hello_macro_derive " 库
   b. 于" hello_macro_derive/src/lib "编辑" derive "

*/

// extern : 使用外部 crate
extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    /* 解析为 DeriveInput 结构体：
          a. " syn "中的" parse_derive_input "函数获取一个 Tokenstream
             并返回一个表示解析出 Rust 代码的 DeriveInput 结构体(对应代码
             "syn::parse(input).unwrap()")
          b. DeriveInput 结构体相关内容大致如下
                DeriveInput {
                    // --snip--
                    ident: Ident {
                        ident: "Pancakes",
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
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, in my macro, my name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}

/*
6、类属性宏
    a. 类属性宏允许创建新的属性(与自定义派生宏相似但不同于为 derive 属性生成代码）
    b. 示例：可创建名为 route 的属性用于注解 web 应用程序框架的函数
        [route(GET,"/")]
        fn index(){ // ... }
        // " #[route] "属性将由框架本身定义为一个过程宏。其宏定义函数签名表象如：
           # [proc_macro_attribute]
            pub fn route(attr:Tokenstream,item:Tokenstream)->Tokenstream{
                // ...
            }
    // 说明：类属性宏其它工作方式和自定义 derive 宏工作方式一致

7、类函数宏
   a. 类函数宏定义像"函数调用的宏",类似于 macro rules！但比函数更灵活。
   b. 示例:" sql! "宏使用方式为：
         let sql = sql! ( SELECT * FROM posts WHERE id=1 )
      // 则可将其定义如下:
         #[proc_macro]
         pub fn sql(input:Tokenstream)->Tokenstream {
             //...
         }

8、宏的资料推荐
   https://danielkeep.github.io/tlborm/book/mbe-macro-rules.html

*/