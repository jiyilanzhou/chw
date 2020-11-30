// Copyright 2019-2020 Liebi Technologies.
// This file is part of Bifrost.

// Bifrost is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Bifrost is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Bifrost.  If not, see <http://www.gnu.org/licenses/>.

//! Tests for the module.
#![cfg(test)]

use crate::*;
use crate::mock::*;
use node_primitives::{TokenSymbol, RewardTrait};

#[test]
/* @parameter ready :
		vtoken_symbol: TokenSymbol,
		convert_amount: T::Balance,
		referer: T::AccountId)
 */
fn record_reward_should_be_ok() {
	new_test_ext().execute_with(|| {
		println!("================== record_reward ==================");
		/* 调用其它模块：
			a. 在 crate 下导入相应模块依赖：如在"brml/reward/Cargo.toml"导入" assets "模块依赖
				[dev-dependencies]
				assets = { package = "brml-assets", path = "../assets" }
			b. 具体类型(如本 crate 下 mock.rs 文件内的 Test )必须实现相应模块(如"assets pallet")下的 Trait ：
				impl assets::Trait for Test {
					type Event = TestEvent;
					type Balance = u64;
					type AssetId = u32;
					type Price = u64;
					type Cost = u64;
					type Income = u64;
					type Convert = u64;
					type AssetRedeem = ();
					type FetchConvertPrice = ();
					type WeightInfo = ();
				}
			c. 在测试文件中导入本 crate 下所有依赖即：
				use crate::*;
				use mock::*;
		 */
		common();
	});
}

pub fn common(){
	// 方式1：使用" assets 模块下 decl_storage! 内存储结构体 NextAssetId<T> 的配置"
	let _vdot_id = assets::NextAssetId::<Test>::get();
	// 方式2：使用" assets::Module<T> "实现的函数
	let _vdot_id = assets::Module::<Test>::next_asset_id();
	// 方式3：因在 mock.rs 中定义" pub type Assets = assets::Module<Test>; "故可替换为
	let vdot_id = Assets::next_asset_id();
	let vtoken_symbol = TokenSymbol::from(vdot_id + 2);
	// 方式4：直接获取 vtoken_symbol
	assert_eq!(vtoken_symbol, TokenSymbol::vDOT);
	let convert_amount = 100 as u64;
	// let referer = AccountId::new([0 as u8;32]);
	let (referer_one, referer_two) = (168 as u64, 100 as u64);

	// System::set_block_number(1);
	// 调用模块方法方式1：(条件: 源于 mock.rs 文件内" pub type RewardType = crate::Module<Test>; "声明)
	// RewardType::record_reward(vtoken_symbol,convert_amount,referer);
	// 调用模块方法方式2:使用 turbofish
	//crate::Module::<Test>::record_reward(vtoken_symbol,convert_amount,referer);
	// 调用模块方法方式3:使用完全限定语法
	//<crate::Module<Test> as RewardTrait<Balance, AccountId>>::record_reward(vtoken_symbol, convert_amount, referer);
	<crate::Module<Test>>::record_reward(vtoken_symbol, convert_amount, referer_one);
	assert_eq!(1, crate::Module::<Test>::vtoken_reward(vtoken_symbol).len());

	// increate different referer
	<crate::Module<Test>>::record_reward(vtoken_symbol, convert_amount, referer_two);
	assert_eq!(2, crate::Reward::<Test>::get(vtoken_symbol).len());

	// append exist referer
	<crate::Module<Test>>::record_reward(vtoken_symbol, convert_amount, referer_one);
	assert_eq!(2, <crate::Reward<Test>>::get(vtoken_symbol).len());
	// println!("vDot ==> {:?}", <crate::Module<Test>>::vtoken_reward(vtoken_symbol));

	// different vtoken （another table）
	let vtoken_symbol = TokenSymbol::vEOS;
	<crate::Module<Test>>::record_reward(vtoken_symbol, convert_amount, referer_one);
	assert_eq!(1, <crate::Module<Test>>::vtoken_reward(vtoken_symbol).len());
	// println!("vEos ==> {:?}", crate::Module::<Test>::vtoken_reward(vtoken_symbol));

}

#[test]
/* @parameter ready :
		vtoken_symbol: TokenSymbol,
		staking_profit: T::Balance,
 */
fn dispatch_reward_is_be_ok() {
	new_test_ext().execute_with(||{
		// condition ready
		common();
		let (vtoken_symbol,
			referer_one,
			referer_two,
			staking_profit
		) = (TokenSymbol::vDOT, 168 as u64, 100 as u64, 60 as u64);

		// the first query asset
		let referer_one_assets = assets::Module::<Test>::account_assets((vtoken_symbol,referer_one));
		// println!(" referer first ==> {:?}",referer_one_assets);
		assert_eq!(0, referer_one_assets.balance);
		let referer_two_assets = assets::Module::<Test>::account_assets((vtoken_symbol, referer_two));
		assert_eq!(0, referer_two_assets.balance);
		// println!(" another referer ==> {:?}",referer_two_assets);

		// Dispatch reward:
		crate::Module::<Test>::dispatch_reward(vtoken_symbol,staking_profit);

		// The second query asset
		let referer_one_assets = assets::Module::<Test>::account_assets((vtoken_symbol,referer_one));
		// println!(" ==== after dispatch reward ==== \n {:?}",referer_one_assets);
		assert_ne!(0, referer_one_assets.balance);
		assert_eq!(40, referer_one_assets.balance);
		let referer_two_assets = assets::Module::<Test>::account_assets((vtoken_symbol, referer_two));
		// println!(" {:?}",referer_two_assets);
		debug_assert_ne!(0, referer_two_assets.balance);
		debug_assert_eq!(40, referer_two_assets.balance);

		// judge vtoken table whether be clear
		debug_assert!(<crate::Module<Test>>::vtoken_reward(vtoken_symbol).is_empty());
	});
}
