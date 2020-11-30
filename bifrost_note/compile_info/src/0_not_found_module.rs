

/*
0. bifrost pallet / lib.rs 编译错误
   decl_module! 编辑导致未找到相应的模块的错误
    a. 错误信息
          error[E0412]: cannot find type `Module` in this scope
           |     pub enum Error for Module<T: Trait> {
           |                        ^^^^^^ not found in this scope
    b. 解析
       Module 模块内编辑错误导致未生成相应的 Module 模块(绝大多数情况)或没有相应的 Module 模块
    c. 解决方案
       逐次注释掉 Module 模块内代码片段 -> 寻找错误
    // 注：decl_module!{} 导致"未查找到相应模块"的常见错误编辑
        decl_module! {
            pub struct Module<T: Trait> for enum Call where origin: T::Origin {
                // 常见错误编辑 1 ： 功能函数上方未曾使用" #[weight = ... ] "如
                fn sum_reward(origin,){}
                // 常见错误编辑 2 ： 首参数缺少 origin （Substruate 功能函数必然有调用者）
                #[weight = ... ]
                fn dispatch_reward(){}
                // 常见错误编辑 3 ： 参数间未使用逗号分隔
                #[weight = T::WeightInfo::sum_reward()]
                fn sum_reward(
                    origin,
                    vtoken_symbol: TokenSymbol,
                    #[compact] convert_amount: T::Balance   // 非末参数未使用逗号" , "分隔导致错误
                    referer: T::AccountId,                  // 末参数逗号" , "可省略
                ) {}
            }
        }

1. Vec 如何包裹 map
	decl_module! {
		pub struct Module<T: Trait> for enum Call where origin: T::Origin {
	        type Error = Error<T>;
	        fn deposit_event() = default;
	        #[weight = T::WeightInfo::sum_reward()]
	        fn sum_reward(
	            origin,
	            vtoken_symbol: TokenSymbol,
	            #[compact] convert_amount: T::Balance,
	            referer: T::AccountId,
	        ) {
	            // Verify convertor
	            ensure_root(origin)?;
	            ensure!(vtoken_symbol != TokenSymbol::aUSD, Error::<T>::NotSupportaUSD);
	            ensure!(T::AssetTrait::token_exists(vtoken_symbol), Error::<T>::TokenNotExist);
	            // Record table
	            let mut record_vec =  Vec::<BTreeMap<T::AccountId, T::Balance>>::new();
	            // match
	            match vtoken_symbol{
	                TokenSymbol::DOT =>{
	                    record_vec = DotReward::<T>::get();
	                },
	                TokenEOS=>{
	                     record_vec = EosReward::<T>::get();
	                },
	                _=>()
	            }
	            for b_treemap in record_vec.iter_mut(){
	                if b_treemap.contains_key(&referer){
	                    if let Some(x) = b_treemap.get_mut(&referer) {
	                        *x += convert_amount;
	                    }
	                    break;
	                }else{
	                    let mut new_b_treemap = BTreeMap::new();
	                    new_b_treemap.insert(&referer, convert_amount);
	                    // TODO 如何将 map 装入 vec
	                    record_vec.push(new_b_treemap.clone());
	                }
	            }
	        }
		}
	}

 */

/*
2. " cannot find macro `vec` "
a. 问题描述
	新开发的 brml-reward 模块，模块内编译测试均没有问题。当集成到整个 bifrost 项目时 cargo check 报错。

b. 错误信息：
	  error: cannot find macro `vec` in this scope
      --> /home/bifrost/.cargo/git/checkouts/substrate-../primitives/npos-elections/src/phragmms.rs
		 let mut winners = vec![];
	 // 分析：下载的依赖包中未查找到相应的" vec "宏，则说明问题源于"导入的依赖包"(依赖包版本、属性等)
	 // 解决方向：查看相应模块下" Cargo.toml "（即" bifrost/brml/reward/Cargo.toml "）的依赖项

c. 解决方案：
	(0). 发现问题(检查" bifrost/brml/reward/Cargo.toml "发现 )
		[dependencies]
		# -- snip --
		sp-runtime = { version = "2.0.0", default-features = false }
		sp-core = { version = "2.0.0"}
		# 检查发现" sp-core "依赖未标识" default-features = false "，即使用 default-features 属性导致错误
	(1). 解决
		# 在" sp-core "依赖中添加" default-features = false "即：
		[dependencies]
		# -- snip --
		sp-runtime = { version = "2.0.0", default-features = false }
		sp-core = { version = "2.0.0", default-features = false }

 */

/*
3. the trait bound `<T as Trait>::Balance: From<i32>` is not satisfied
	a. 问题描述
		开发的 brml-reward 模块正常测试编译，但集成到 bifrost 时编译报错
	
	b. 错误信息
	  error[E0277]: the trait bound `<T as Trait>::Balance: From<i32>` is not satisfied
	                 record_vec[..256].iter().fold(0.into(), |acc, x| acc + x.record_amount)
	                                                 ^^^^ the trait `From<i32>` is not implemented
	                                                                    for `<T as Trait>::Balance`
	     = note:required because of the requirements on the impl of `Into<<T as Trait>::Balance>`
	                                                                                       for `i32`
	     help: consider further restricting the associated type
	         fn dispatch_reward(vtoken_symbol: TokenSymbol, staking_profit: T::Balance)
	                                                        where <T as Trait>::Balance: From<i32> {
	                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
	
	c. 原因分析
		虽然在 reward crate 内编译测试通过，但因为 crate 模块内的测试时指定具体的类型故可能正常编译且通过测试，但是
		集成到" bifrost "时整体编译未必能正确推断出相应类型故需指定（有待深入理解:[自]理论上模块内编译通过集成后也应该
		能正常编译）
	
	d. 解决方案：
		(0). 方案一：在具体的数据实例上使用约束如下
			 record_vec.iter().fold(T::Balance::from(0u32), |acc, x| acc + x.record_amount)
		(1). 方案二：根据错误提示在函数上使用约束如下
			 fn dispatch_reward(vtoken_symbol: TokenSymbol, staking_profit: T::Balance)
				 where <T as Trait>::Balance: From<u32>
			 {
				 // --snip--
				 record_vec[..256].iter().fold(0.into(), |acc, x| acc + x.record_amount)
				 // --snip--
			 }

 */
