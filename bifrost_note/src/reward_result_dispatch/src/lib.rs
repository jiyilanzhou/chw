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
use frame_support::{Parameter, decl_module, decl_event, decl_error, decl_storage, dispatch::DispatchResult, dispatch::DispatchError};
use node_primitives::{RewardTrait, AssetTrait, TokenSymbol};
use sp_runtime::traits::{AtLeast32Bit, Member, Saturating, MaybeSerializeDeserialize};
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
			=> Vec<RewardRecord<T::AccountId, T::Balance>> = Vec::with_capacity(512);
	}
}

decl_event! {
	pub enum Event {
		QueryRefererAmountSuccess,
	}
}

decl_error! {
	pub enum Error for Module<T: Trait> {
		/// No included referer
		RefererNotExist
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {}
}

/*
impl<T: Trait> RewardTrait<T::Balance, T::AccountId> for Module<T> {
	type Error = Error<T>;
	
	fn record_reward(vtoken_symbol: TokenSymbol, convert_amount: T::Balance, referer: T::AccountId) -> Result<(), Self::Error> {
		// Traverse (if map doesn't contains vtoken_symbol, the system will be initial)
		Reward::<T>::mutate(vtoken_symbol, |vec| {
			let mut flag = true;
			for item in vec.iter_mut() {
				if item.account_id.eq(&referer) {
					// Update the referer's record_amount
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
			// Sort vec
			vec.sort_by(|a, b| b.record_amount.cmp(&a.record_amount));
		});
		Ok(())
	}
	
	fn dispatch_reward(vtoken_symbol: TokenSymbol, staking_profit: T::Balance) -> Result<(), Self::Error> {
		// Obtain vec
		let record_vec = Self::vtoken_reward(vtoken_symbol);
		if record_vec.is_empty() {
			return Err(Error::<T>::TokenNotExist);
		}
		// The total statistics
		let sum: T::Balance = {
			if record_vec.len() > 256 {
				record_vec[..256].iter().fold(T::Balance::from(0u32), |acc, x| acc + x.record_amount)
			} else {
				record_vec.iter().fold(T::Balance::from(0u32), |acc, x| acc + x.record_amount)
			}
		};
		// Check sum
		if sum.eq(&T::Balance::from(0u32)) {
			return Err(Error::<T>::NoIncludedReferer);
		}
		// Dispatch reward
		let mut length = record_vec.len();
		if length > 256 {
			length = 256
		}
		for referer in record_vec[0..length].iter() {
			let reward = referer.record_amount.saturating_mul(staking_profit) / sum;
			if reward.ne(&T::Balance::from(0u32)) {
				T::AssetTrait::asset_issue(vtoken_symbol, &referer.account_id, reward);
			}
		}
		// Clear vec
		Reward::<T>::mutate(vtoken_symbol, |vec| {
			vec.clear();
		});
		Ok(())
	}
}
*/


impl<T: Trait> RewardTrait<T::Balance, T::AccountId> for Module<T> {
	type Error = Error<T>;
	
	fn record_reward(vtoken_symbol: TokenSymbol, convert_amount: T::Balance, referer: T::AccountId) -> DispatchResult {
		// Traverse (if map doesn't contains vtoken_symbol, the system will be initial)
		Reward::<T>::mutate(vtoken_symbol, |vec| {
			let mut flag = true;
			for item in vec.iter_mut() {
				if item.account_id.eq(&referer) {
					// Update the referer's record_amount
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
			// Sort vec
			vec.sort_by(|a, b| b.record_amount.cmp(&a.record_amount));
		});
		Ok(())
	}
	
	fn dispatch_reward(vtoken_symbol: TokenSymbol, staking_profit: T::Balance) -> DispatchResult {
		// Obtain vec
		let record_vec = Self::vtoken_reward(vtoken_symbol);
		if record_vec.is_empty() {
			// return Err(DispatchError::Module { index: 0, error: 0, message: Some("RefererNotExist") });
			// 不可自定义 index 故使用下述 ensure! 替代
			ensure!(!record_vec.is_empty(), Error::<T>::RefererNotExist);
		}
		// The total statistics
		let sum: T::Balance = {
			if record_vec.len() > 256 {
				record_vec[..256].iter().fold(T::Balance::from(0u32), |acc, x| acc + x.record_amount)
			} else {
				record_vec.iter().fold(T::Balance::from(0u32), |acc, x| acc + x.record_amount)
			}
		};
		// Dispatch reward
		let mut length = record_vec.len();
		if length > 256 {
			length = 256
		}
		for referer in record_vec[0..length].iter() {
			let reward = referer.record_amount.saturating_mul(staking_profit) / sum;
			if reward.ne(&T::Balance::from(0u32)) {
				T::AssetTrait::asset_issue(vtoken_symbol, &referer.account_id, reward);
			}
		}
		// Clear vec
		Reward::<T>::mutate(vtoken_symbol, |vec| {
			vec.clear();
		});
		Ok(())
	}
}
