
// asset[ˈæset]n.资产     // treasury[ˈtreʒəri]n.国库
// aura[ˈɔːrə]n.光环      // pallet[ˈpælət]n.画板,托盘
// primitive[ˈprɪmətɪv]adj&n.原始(的),原语
/*
0. Substrate 入门

1. " substrate\frame "模块
    其下的组件都是在 Runtime 层面上(可选择自组合[亦可自定义实现])。比如
    " assets ：与资产相关"、" treasury : 国库 "等模块

2. Substrate 一键升级永不分叉
   斯原理是将整个 Runtime 逻辑编译为两个版本：Wasm runtime (from chain)
   及 Native runtime (from client)，客户端版本(即本地)执行时预先判断其与
   链上版本是否一致，不一致则拉取更新后执行或直接执行链上二进制文件(屏蔽因
   客户端未及时更新导致的错误)

3. The Substrate 总体层次
    a. 图示
        ----- TECHNICAL FREEDOM ---->--->    // 从左到右(技术更加自由)
                           Substrate Core
                Substrate SRML              // 集中在业务需求较强的一层
        Substrate Node
        <------ DEVELOPMENT EASE ----<---    // 从右到左(开发更加容易)
    b. Substrate Core : 较为底层(如"共识、交易队列、网络传输层")，其对应
       框架" substrate\client "目录。比如" substrate\client\consensus "
       层目前提供了几种共识方案(如"aura、babe、common"等[参见框架源码])
    // 注：团队可根据自身水平选择相应层次的开发(尤其推荐 Substate Core [
           因官网已提供了众多的可选择性且技术更加自由])

*/
/*
4. Substrate 架构(源码层面)
    a. bin 目录(与可执行的二进制文件相关)
       (0). node ：官网全面功能的实现(暂使用 babe [支持链上随机数]共识)
       (1). node-template : 最小化结构(暂使用 aura [轮流出块]共识)
            上手练习：参见" substrate\frame\example\src\lib.rs "(对应
            官网" https://gibhub.com/paritytech/substrate/blob/master/frame/example/src/lib.rs ")。
            编辑完毕只需在"substrate\bin\node-template\runtime\src\lib.rs"
            下引入模块(如" mod example ")且追加至"construct_runtime!(...)"
        (2). untils 提供工具
    b. client ：与客户端相关内容(如网络、结点、监控及存储等)
    c. frame ：整个 runtime 组件
       模块命名规范: 如其下的"membership、metadata 及 system "模块命名皆以
       " frame-xxx "开头命名(源于其下"Cargo.toml")，因这些模块是整个 frame
       内非常基础的模块(比如需要自定义模块也是依赖这些组件)。而其余的部分模块
       则以" pallet-xxx "开头( pallet：画板、托盘)。故 Substrate 的架构需要
       从框架分层及模块命名(模块命名与所在目录名不尽相同[甚至大多数不同])入手。
       从其模块命名可大致判断其偏于 Core(底层)、SRML(逻辑业务层)还是 Node 层
    d. primitives : 原语
       (0). 一些基础定义内置于 privitives 而在其它地方实现。
       (1). primitives 内与 client 内些许模块命名相同
    e. utils : 工具库
    // 注: 除 bin 模块(其下" node-template "最适合练手)其余皆为" 库 "模块
*/

/* "  substrate\frame\sudo  "模块：
5. 如何开始定义一个 Runtime (以" substrate\frame\sudo [有超级权限]"为例 )
   a. sudo 模块被赋于了超级权限，比如可指定一个超级账户执行链上代码升级(实际
      情况下链上代码升级并非通过超级权限操纵，而是通过整个链上治理来达成),此
      超级权限仅用于开发者练手且仅用于测试网络
   b. " substrate\frame\sudo\src\lib.rs "内代码(其内宏定义的扩展性组件代码
      并非严格遵循 Rust 语法[即不属于 Rust 代码],其更像 Rust 的某种子集)层次
      清晰(几乎 Runtime 都是根据这个层次展开的)
      (0). pub trait Trait: frame_system::Trait {...}
      (1). decl_module! {...} 模块
           定义外界与链上交互的方法(即是可调用的链上方法)
      (2). decl_event!(...)
           定义事件
      (3). decl_storage! {...}
           定义链上存储位置
      (4). decl_error! {...}
           定义运行时会遇上的错误处理

6. decl_module! {...} 模块
   a. 模块内源码(源于" substrate\frame\sudo\src\lib.rs ")
        decl_module! {
            pub struct Module<T: Trait> for enum Call where origin: T::Origin {
                type Error = Error<T>;
                fn deposit_event() = default;
                #[weight = (call.get_dispatch_info().weight + 10_000, call.get_dispatch_info().class)]
                fn sudo(origin, call: Box<<T as Trait>::Call>) -> DispatchResultWithPostInfo {
                    /* 当" frame\system\src\lib.rs "当签名即" RawOrigin::Signed(AccountId) "后
                       发送至链上则调用" ensure_signed(origin) "进行校验从而获取 sender 。即发送
                       签名后的消息至链上
                    */
                    let sender = ensure_signed(origin)?;
                    /* 如链上升级的具体实现：
                       可先在" decl_storage! {...} "模块存储一个 key 即 T::AccountId (地址)，当
                       需要升级链上逻辑时可先调用" ensure_signed(origin) "验签，验签通过后再与存储
                       模块即" decl_storage! {...} "内存储的 key 比较，若不等则报 Error
                    */
                    ensure!(sender == Self::key(), Error::<T>::RequireSudo);
                    /* 使用 Root 超级权限操作即把 function 重新包装为使得 origin 变为 RawOrigin，
                       从而调用" substrate\frame\system\src\lib.rs "文件" decl_module! {...} "
                       模块内的" fn set_code(origin, code: Vec<u8>){...} "方法实现链上升级
                    */
                    let res = call.dispatch_bypass_filter(frame_system::RawOrigin::Root.into());
                    Self::deposit_event(RawEvent::Sudid(res.map(|_| ()).map_err(|e| e.error)));
                    Ok(Pays::No.into())
                }
                #[weight = (*_weight, call.get_dispatch_info().class)]
                fn sudo_unchecked_weight(origin, call: Box<<T as Trait>::Call>, _weight: Weight) -> DispatchResultWithPostInfo {
                    let sender = ensure_signed(origin)?;
                    ensure!(sender == Self::key(), Error::<T>::RequireSudo);
                    let res = call.dispatch_bypass_filter(frame_system::RawOrigin::Root.into());
                    Self::deposit_event(RawEvent::Sudid(res.map(|_| ()).map_err(|e| e.error)));
                    Ok(Pays::No.into())
                }
                #[weight = 0]
                fn set_key(origin, new: <T::Lookup as StaticLookup>::Source) -> DispatchResultWithPostInfo {
                    let sender = ensure_signed(origin)?;
                    ensure!(sender == Self::key(), Error::<T>::RequireSudo);
                    let new = T::Lookup::lookup(new)?;
                    Self::deposit_event(RawEvent::KeyChanged(Self::key()));
                    // Sudo user does not pay a fee.
                    Ok(Pays::No.into())
                }
                #[weight = (call.get_dispatch_info().weight + 10_000, call.get_dispatch_info().class)]
                fn sudo_as(origin,
                    who: <T::Lookup as StaticLookup>::Source,
                    call: Box<<T as Trait>::Call>
                ) -> DispatchResultWithPostInfo {
                    let sender = ensure_signed(origin)?;
                    ensure!(sender == Self::key(), Error::<T>::RequireSudo);
                    let who = T::Lookup::lookup(who)?;
                    let res = match call.dispatch_bypass_filter(frame_system::RawOrigin::Signed(who).into()) {
                        Ok(_) => true,
                        Err(e) => {
                            sp_runtime::print(e);
                            false
                        }
                    };
                    Self::deposit_event(RawEvent::SudoAsDone(res));
                    Ok(Pays::No.into())
                }
            }
        }
   b. 模块内方法首参数皆为" origin "(源于 RawOrigin )其源码：
        // 源于"substrate\frame\system\src\lib.rs "
        /// Origin for the System module.
        #[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode)]
        pub enum RawOrigin<AccountId> {
            /// The system itself ordained this dispatch to happen: this is the highest privilege level.
            // Root 其功能类似" substrate\frame\sudo "模块，即拥有超级权限如升级链上代码
            Root,
            /// It is signed by some public key and we provide the `AccountId`.
            /* 当签名即" RawOrigin::Signed(AccountId) "后发送至链上,其可被
               " substrate\frame\sudo\src\lib.rs "下" decl_module! {...} "
               模块内的" ensure_signed(origin) "调用进行校验从而获取 sender
            */
            Signed(AccountId),      // 签名
            /// It is signed by nobody, can be either:
            /// * included and agreed upon by the validators anyway,
            /// * or unsigned transaction validated by a module.
            /* None 表示没有签名。目前在 Substrate 主要用于由 Validator 发送的消息，其发送的消息
              无需经过 validator 签名。如" substrate\frame\timestamp\src\lib.rs "文件部分源码：
              fn set(origin, #[compact] now: T::Moment) { // 设置时间戳(仅 validator 可调用)
                  ensure_none(origin)?; //
                  ...
              }
              // 当其为 validator 的情况下即使不提供签名" ensure_none(origin) "亦可发起交易。
                 因为类似"时间戳"之类被区块链认为是底层的常识无需再提供签名及认证。
            */
            None,
        }

7. decl_storage! {...} 模块：
    a. 源码：
        decl_storage! {
            trait Store for Module<T: Trait> as Sudo {
                /// The `AccountId` of the sudo key.
                Key get(fn key) config(): T::AccountId; // 宏展开后 Key 为结构体
            }
        }
    b. 可于官网文档" https://substrate.dev/rustdocs/v2.0.0-rc5/sc_service/index.html "搜索
       (在"All crates"中搜索如"decl_storage!")查看宏(图"0_substrate官网查阅宏使用.png")。并
       参阅" http://substrate.dev/docs/en/ "->左侧栏点选"Rutime"选项下的"Runtime Storage(
       可参照其内文档说明模仿编辑)"。

 */