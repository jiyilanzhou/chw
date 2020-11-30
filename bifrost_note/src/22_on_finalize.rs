
/*
0. " decl_module! "下的 on_finalize
	a. 非测试模块(即" #[test] ")不可使用 assert_ok!
		非测试模块使用 assert_ok! 即使本模块内断点调试和测试均通过，则集成到整个项目时 build 也会报错即：
		( frame_support::assert_ok 虽然可用于处理 DispatchResult 类型但仅用于" #[test] " )
	b. " decl_module! "下的 on_finalize 可用 ensure! 或 assert!(源于" use core::assert; ")
		[ ensure! 或 assert! 均用于处理 bool 类型, 但 ensure! 返回 Result 类型] )

1. 示例
	decl_module! {
		pub struct Module<T: Trait> for enum Call where origin: T::Origin {
			fn on_finalize(current_block_number: T::BlockNumber) {
				// -- snip --
				// Self::issue_bnc();       // 不处理有警告
				// frame_support::assert_ok 虽然可用于处理 DispatchResult 类型但仅用于" #[test] "
				assert_ok!(Self::issue_bnc());
				// core::assert 用于处理 bool 类型
				assert!(Self::issue_bnc());
				// -- snip --
			}
		}
	}

2. 在 mock.rs 文件中
	// 以下代码可有可无(主要是测试文件中即"tests.rs "中必须添加" use frame_support::traits::OnFinalize;")
	// // pub type VtokenMint = crate::Module<Test>;
	// pub type System = frame_system::Module<Test>;
	//
	// // simulate block production
	// pub(crate) fn run_to_block(n: u64) {
	// 	while System::block_number() < n {
	// 		System::on_finalize(System::block_number());
	// 		System::set_block_number(System::block_number() + 1);
	// 		System::on_initialize(System::block_number());
	// 		// VtokenMint::on_finalize(System::block_number());
	// 		<crate::Module<Test>>::on_finalize(System::block_number());
	// 		// VtokenMint::on_initialize(System::block_number());
	// 	}
	// }




 */