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
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use frame_support::{Parameter, decl_module, decl_storage};
use node_primitives::{
	RewardTrait, AssetTrait, TokenSymbol,
};
use sp_runtime::traits::{AtLeast32Bit, Member, MaybeSerializeDeserialize};
use codec::{Encode, Decode};

mod mock;
mod tests;

pub trait Trait: frame_system::Trait {
	/// The units in which we record balances.
	type Balance: Member + Parameter + AtLeast32Bit + Default + Copy + MaybeSerializeDeserialize + From<Self::BlockNumber>;
	/// The arithmetic type of asset identifier.
	type AssetId: Member + Parameter + AtLeast32Bit + Default + Copy + MaybeSerializeDeserialize;
	/// The units in which we record costs.
	type Cost: Member + Parameter + AtLeast32Bit + Default + Copy + MaybeSerializeDeserialize;
	/// The units in which we record incomes.
	type Income: Member + Parameter + AtLeast32Bit + Default + Copy + MaybeSerializeDeserialize;
	/// Assets
	type AssetTrait: AssetTrait<Self::AssetId, Self::AccountId, Self::Balance, Self::Cost, Self::Income>;
}

#[derive(Encode, Decode, Default, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct RewardRecord<AccountId, Balance> {
	pub account_id: AccountId,
	pub record_amount: Balance,
}

decl_storage! {
	trait Store for Module<T: Trait> as Reward {
        Reward get(fn vtoken_reward): map hasher(blake2_128_concat) TokenSymbol
                           => Vec<RewardRecord<T::AccountId, T::Balance>> = Vec::with_capacity(256);
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {}
}

impl<T: Trait> RewardTrait<T::Balance, T::AccountId> for Module<T> {
	fn record_reward(vtoken_symbol: TokenSymbol, convert_amount: T::Balance, referer: T::AccountId) {
		/*  // 此处判断完全没有必要:但为了明确逻辑语义亦可添加
			// ensure vtoken is exist
			if !Reward::<T>::contains_key(vtoken_symbol) {
				Reward::<T>::insert(vtoken_symbol,
								Vec::<RewardRecord<T::AccountId, T::Balance>>::with_capacity(512));
			};
			// 分析:" Reward::<T>::function() "会调用" decl_storage! "宏存储的数据，若" Reward::<T> "对应
			   map 未包含 vtoken_symbol 则系统会自动初始化" vtoken_symbol,Vec[RewardRecord;512] "装入 map
			// 在" decl_storage! "内" Reward ... "行断点调试可展示效果
		 */
		// Traverse
		Reward::<T>::mutate(vtoken_symbol, |vec| {
			let mut flag = true;
			for item in vec.iter_mut() {
				if item.account_id.eq(&referer) {
					item.record_amount += convert_amount;
					flag = false;
					break;
				}
			}
			if flag {
				// Create new account
				let new_referer = RewardRecord::<T::AccountId, T::Balance> {
					account_id: referer,
					record_amount: convert_amount,
				};
				// Append to vec
				vec.push(new_referer);
			}
			// Sort vec     // partial_cmp 用于浮点数且系统内不允许出现 unwrap()
			// vec.sort_by(|a, b| b.record_amount.partial_cmp(&a.record_amount).unwrap());
			vec.sort_by(|a, b| b.record_amount.cmp(&a.record_amount));
		});
	}
	
	fn dispatch_reward(vtoken_symbol: TokenSymbol, staking_profit: T::Balance) {
		// Obtain vec
		let record_vec = Self::vtoken_reward(vtoken_symbol);
		if record_vec.is_empty() {
			return;
		}
		// The total statistics
		let sum: T::Balance = {
			if record_vec.len() > 256 {
				record_vec[..256].iter().fold(0.into(), |acc, x| acc + x.record_amount)
			} else {
				record_vec.iter().fold(0.into(), |acc, x| acc + x.record_amount)
			}
		};
		// Dispatch reward
		let mut length = record_vec.len();
		if length > 256 {
			length = 256
		}
		for i in 0..length {
			let account_id = &record_vec[i].account_id;
			// let reward = record_vec[i].record_amount * staking_profit / sum;
			let reward = referer.record_amount.saturating_mul(staking_profit) / sum;
			T::AssetTrait::asset_issue(vtoken_symbol, account_id, reward);
		}
		// Clear vec
		Reward::<T>::mutate(vtoken_symbol, |vec| {
			vec.clear();
		});
	}
}

