
// mock[mɒk]n.模拟,模仿
/*
0. Bifrost 项目新增 pallet 开发流程

1. 在" bifrost/brml "目录下拷贝任一 pallet 且重命名为目标 pallet
   （此例以拷贝" assets "且重命名为 poe pallet）其" src "目录如下
      (0). lib.rs : 功能文件
      (1). mock.rs ：测试依赖文件
      (2). tests.rs ：测试文件(源于拷贝故可先注释其内容[避免编译错误])

2. 修改" brml/poe/Cargo.toml "文件为:
    // --snip--
    name = "brml-poe"
    // --snip-- (其余依赖后续按需修改)

3. 将编辑好的 poe 模块加入 Runtime （模仿 assets pallet）
    a. 在" ..\runtime\Cargo.toml "内添加 poe 模块并追加至 [features] 下的 std 即
        [dependencies]
        // --snip--
        brml-poe = { path = "../../../brml/poe", default-features = false }
        // --snip--
        std = [
            // --snip--
            "brml-poe/std",
            // --snip--
        ]

    b. 在 runtime 的功能文件即" ..\runtime\src\lib.rs"内加载 poe 模块
        (0). 追加 poe 的 Runtime 实现
              //pub use poe;
              impl poe::Trait for Runtime {
                  type Event = Event;
              }
        (1). 将自定义 pallet 追加到" construct_runtime! "宏
             construct_runtime!(
                 pub enum Runtime where
                     Block = Block,
                     NodeBlock = opaque::Block,
                     UncheckedExtrinsic = UncheckedExtrinsic
                 {
                     //...
                     // 追加自定义 pallet
                     PoeModule: poe::{Module, Call, Storage, Event<T>},
                 }
             );
    c. 最后将 poe 加入" bifrost/Cargo.toml "成员并编译项目
        [workspace]
        members = [
            // --snip--
            "brml/assets",
            "brml/poe",
            // --snip--
        ]
        // --snip--

4. 编写测试依赖文件 mock.rs



 */