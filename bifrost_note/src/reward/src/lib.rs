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
use alloc::collections::btree_map::BTreeMap;
use frame_support::{Parameter, ensure, decl_module, decl_error, decl_storage, dispatch::DispatchResult};
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

#[derive(Encode, Decode, Default, Clone, Debug)]
pub struct RewardRecord<AccountId, Balance> {
	pub account_id: AccountId,
	pub record_amount: Balance,
}

pub const CAPACITY: usize = 512;
pub const LEN: usize = 256;

decl_storage! {
	trait Store for Module<T: Trait> as Reward {
		Reward get(fn vtoken_reward): map hasher(blake2_128_concat) TokenSymbol
			=> (BTreeMap<T::AccountId, T::Balance>, Vec<RewardRecord<T::AccountId, T::Balance>>);
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

impl<T: Trait> RewardTrait<T::Balance, T::AccountId> for Module<T> {
	type Error = Error<T>;
	
	fn record_integral(vtoken_symbol: TokenSymbol, convert_amount: T::Balance, referer: T::AccountId) -> DispatchResult {
		// Traverse (if map doesn't contains vtoken_symbol, the system will be initial)
		Reward::<T>::mutate(vtoken_symbol, |tup| {
			if tup.0.contains_key(&referer) {
				if let Some(x) = tup.0.get_mut(&referer) {
					*x = x.saturating_add(convert_amount);
				}
			} else {
				tup.0.insert(referer.clone(), convert_amount);
			}
			match tup.0.len() {
				// Referer amount = 256
				LEN => {
					// Initial capacity = 512
					tup.1 = Vec::<RewardRecord<T::AccountId, T::Balance>>::with_capacity(CAPACITY);
					for (key, value) in tup.0.iter() {
						// Append new account
						append_referer::<T>(&mut tup.1, key.clone(), value.clone());
					}
					// Vec sort
					tup.1.sort_by(|a, b| b.record_amount.cmp(&a.record_amount));
				}
				// Referer amount > 256
				x  if x > LEN => {
					let mut flag = true;
					for item in tup.1.iter_mut() {
						if item.account_id.eq(&referer) {
							// Update the referer's record_amount
							item.record_amount = item.record_amount.saturating_add(convert_amount);
							flag = false;
							break;
						}
					}
					if flag {
						append_referer::<T>(&mut tup.1, referer, convert_amount);
					}
					// Vec sort
					tup.1.sort_by(|a, b| b.record_amount.cmp(&a.record_amount));
				}
				_ => ()
			}
		});
		
		Ok(())
	}
	
	fn dispatch_reward(vtoken_symbol: TokenSymbol, staking_profit: T::Balance) -> DispatchResult {
		// Obtain tup
		let tup = Module::<T>::vtoken_reward(vtoken_symbol);
		ensure!(!tup.0.is_empty(), Error::<T>::RefererNotExist);
		// The total statistics
		let mut sum = T::Balance::from(0u32);
		match tup.0.len() {
			len if len <= LEN => {
				sum = tup.0.values().fold(T::Balance::from(0u32), |acc, x| acc.saturating_add(*x))
			}
			len if len > LEN => {
				sum = tup.1[..LEN].iter().fold(T::Balance::from(0u32), |acc, x| acc.saturating_add(x.record_amount))
			}
			_ => ()
		}
		// Dispatch reward
		ensure!(sum.ne(&T::Balance::from(0u32)), Error::<T>::RefererNotExist);
		match tup.0.len() {
			len if len <= LEN => {
				for (key, value) in tup.0.iter() {
					let reward = value.saturating_mul(staking_profit) / sum;
					// Check dispatch reward
					if reward.ne(&T::Balance::from(0u32)) {
						T::AssetTrait::asset_issue(vtoken_symbol, key, reward);
					}
				}
			}
			len if len > LEN => {
				for referer in tup.1[0..LEN].iter() {
					let reward = referer.record_amount.saturating_mul(staking_profit) / sum;
					// Check dispatch reward
					if reward.ne(&T::Balance::from(0u32)) {
						T::AssetTrait::asset_issue(vtoken_symbol, &referer.account_id, reward);
					}
				}
			}
			_ => ()
		}
		// Clear BTreeMap and Vec
		Reward::<T>::mutate(vtoken_symbol, |tup| {
			tup.0.clear();
			tup.1.clear();
		});
		
		Ok(())
	}
	
	fn query_integral(vtoken_symbol: TokenSymbol, referer: T::AccountId) -> Result<T::Balance, Self::Error> {
		// Get tup
		let tup = Module::<T>::vtoken_reward(vtoken_symbol);
		ensure!(!tup.0.is_empty(), Error::<T>::RefererNotExist);
		ensure!(tup.0.contains_key(&referer), Error::<T>::RefererNotExist);
		match tup.0.get(&referer) {
			Some(integral) => Ok(*integral),
			None => Err(Error::<T>::RefererNotExist)
		}
	}
}

fn append_referer<T: Trait>(vec: &mut Vec<RewardRecord<T::AccountId, T::Balance>>, referer: T::AccountId, convert_amount: T::Balance) {
	// Create new account
	let new_referer = RewardRecord::<T::AccountId, T::Balance> {
		account_id: referer,
		record_amount: convert_amount,
	};
	// Append to vec
	vec.push(new_referer);
}