/*
0. Substrate 使用的 Rust 宏
    a. 文档" https://doc.rust-lang.org/book/ch19-06-macros.html "
    b. Substrate 使用宏源于 DSL (Domain Specific Language 特定领域语言)
    c. 宏展开：cargo expand
       参阅：" https://github.com/dtolnay/cargo-expand "
       参阅：" https://github.com/kaichaosun/play-substrate/blob/master/pallets/template/expanded.rs "
    d. substrate 的其它相关宏：
       (0). decl_runtime_apis & imp_runtime_apis ： 定义 runtime api
            参阅" https://substrate.dev/recipes/3-entrees/runtime-api.html "
            参阅" https://substrate.dev/rustdocs/master/sp_api/macro.decl_runtime_apis.html "
            参阅" https://substrate.dev/rustdocs/master/sp_api/macro.impl_runtime_apis.html "
       (1). runtime_interface : 定义在 runtime 里可调用的 Host 提供的函数
            参阅" https://substrate.dev/rustdocs/v2.0.0-alpha.8/sp_runtime_interface/attr.runtime_interface.html "

1. Substrate Runtime
    a. 内置的模块亦称为 Pallet (调色板)如:
        assets / balances : 资产
        babe / grandpa : 共识
        collective / democracy : 治理
    b. Runtime 模块组成常用到的宏
        decl_storage 定义存储单元
        decl_module 包含可调用函数
        decl_event 事件
        decl_error 错误信息
        construct_runtime 添加模块到 Runtime

2. Pallet 组成常用宏
    a. decl_storage : 定义存储单元
       (0). 示例
              /// The pallet's configuration trait.
              pub trait Trait:  system::Trait {
                      /// The overarching event type.
                      type Event:  From<Event<Self>>  +  Into<<Self as system::Trait>::Event>;
              }
              // This pallet's storage items.
              decl_storage! {
                  // 存储别名：用于区分于其它模块
                  trait Store for Module<T: Trait> as TemplateModule {
                      // 通过 fn 关键字定义 something 可选函数
                      Something get(fn something):  Option<u32>;
                  }
              }
    b. decl_event ： 区块链是一个异步系统，runtime 通过触发事件通知交易执行结果
        (0). 示例
              decl_event!(
                  pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
                      SomethingStored(u32, AccountId),
                  }
              )
    c. decl_error
        (0). 示例
              decl_error! {
                  pub enum Error for Module<T: Trait> {
                        /// Value was None
                        NoneValue,
                        /// Value reached maximum and cannot be incremented further
                        StorageOverflow,
                  }
              }
        (1). 可调用函数里的错误类型 :
                不能添加数据
                通过 metadata 暴露给客户端
                错误发生时触发 system.ExtrinsicFailed 事件(包含对应错误信息)
    d. decl_module
        (0). 区块链的链上状态变化由交易触发，Substrate 不仅支持自定义的存储数据结构
             还支持自定义的交易(如转账、注册身份、投票等[亦称 extrinsic 外部交易])
        (1). decl_module 模块用于定义可调用函数，每个外部交易都会触发一个可调用函数
             并根据交易体信息也就是函数参数，更新链上状态
        (2). 示例
              decl_module! {
                  // T 泛型内并未定义 Origin 故其源于" system::Trait "
                  pub struct Module<T: Trait> for enum Call where origin:  T::Origin {
                      type Error = Error<T>;
                      fn deposit_event() = default;
                      // 定义可调用函数：origin 源于 T::Origin 故无需再指定类型约束
                      #[weight = 10_000]
                      pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
                          // -- snip --
                          // 将 u32 类型值 something 放入 Something 存储单元
                          Something::put(something);
                          // 触发事件
                          Self::deposit_event(RawEvent::SomethingStored(something, who));
                          Ok(())
                      }
                  }
                  #[weight = 10_000]
                  pub fn cause_error(origin) -> dispatch::DispatchResult {
                      // -- snip --
                      // 通过 get 函数获取存储单元内的值
                      match Something::get() {
                          None => Err(Error::<T>::NoneValue)?,
                          Some(old) => {
                              let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
                              Something::put(new);
                              Ok(())
                          },
                      }
                  }
              }
        (3). Runtime 模块除 deposit_event 外还存在的保留函数：
             on_initialize ：在每个区块的开头执行
             on_finalize ：在每个区块结束时执行
             offchain_worker ：链外执行(不占用链上资源)
             on_runtime_upgrade ：有 runtime 升级时才会执行(常用于迁移数据)

3. construct_runtime 加载模块
    a. 实现
        impl template::Trait for Runtime {
            type Event = Event;
        }
    b. 追加至 consturct_rutime! 宏
        construct_runtime!(
            pub enum Runtime where
                    Block = Block,
                    NodeBlock = opaque::Block,
                    UncheckedExtrinsic = UncheckedExtrinsic
            {
                    // -- snip --
                    // 引入模块须后置于依赖模块(即依赖模块前置)
                    TemplateModule:  template::{Module, Call, Storage, Event<T>},
            }
        );





*/

