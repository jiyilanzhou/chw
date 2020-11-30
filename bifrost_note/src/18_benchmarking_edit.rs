
/*
0. benchmarking 文件编辑
    a. 编写基准测试
        benchmarks! {
            _ {
                   // 提供 benchmark 所需的公共变量如
                   let b in 1..100 => (); // 提供 1 到 100 的变量 b 但不对其进行其它操作
               }
            func {
                   // 对变量所有值进行循环
                   let b in ...;
                   let caller = account("caller",0,0)

            }: _(RawOrigin::Signed(caller),b.into())  // 调用函数(名称一致可使用" _ "简化)
            verify {
                   let value = Something::get();
                   // 验证存入的值是否是所预期的
                   assert_eq!(value,b.into());
            }

        }

    b. 辅助测试模块
        #[cfg(test)]
        mod tests {
            use super::*;
            use crate::tests_composite::{ExtBuilder, Test};
            use frame_support::assert_ok;

            #[test]
            fn transfer() {
                ExtBuilder::default().build().execute_with(|| {
                    assert_ok!(test_benchmark_transfer::<Test>());
                });
            }
        }

1.


 */


#![cfg(feature = "runtime-benchmarks")]
use super::*;
use frame_system::RawOrigin;
use frame_benchmarking::{benchmarks, account, whitelisted_caller};
use sp_runtime::traits::Bounded;
use crate::Module as Balances;

const SEED: u32 = 0;
// existential deposit multiplier
const ED_MULTIPLIER: u32 = 10;

benchmarks! {
	_ { }

	// Benchmark `transfer` extrinsic with the worst possible conditions:
	// * Transfer will kill the sender account.
	// * Transfer will create the recipient account.
	transfer {
		let existential_deposit = T::ExistentialDeposit::get();
		let caller = whitelisted_caller();

		// Give some multiple of the existential deposit + creation fee + transfer fee
		let balance = existential_deposit.saturating_mul(ED_MULTIPLIER.into());
		let _ = <Balances<T> as Currency<_>>::make_free_balance_be(&caller, balance);

		// Transfer `e - 1` existential deposits + 1 unit, which guarantees to create one account, and reap this user.
		let recipient: T::AccountId = account("recipient", 0, SEED);
		let recipient_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(recipient.clone());
		let transfer_amount = existential_deposit.saturating_mul((ED_MULTIPLIER - 1).into()) + 1.into();
	}: transfer(RawOrigin::Signed(caller.clone()), recipient_lookup, transfer_amount)
	verify {
		assert_eq!(Balances::<T>::free_balance(&caller), Zero::zero());
		assert_eq!(Balances::<T>::free_balance(&recipient), transfer_amount);
	}

	// Benchmark `transfer` with the best possible condition:
	// * Both accounts exist and will continue to exist.
	#[extra]
	transfer_best_case {
		let caller = whitelisted_caller();
		let recipient: T::AccountId = account("recipient", 0, SEED);
		let recipient_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(recipient.clone());

		// Give the sender account max funds for transfer (their account will never reasonably be killed).
		let _ = <Balances<T> as Currency<_>>::make_free_balance_be(&caller, T::Balance::max_value());

		// Give the recipient account existential deposit (thus their account already exists).
		let existential_deposit = T::ExistentialDeposit::get();
		let _ = <Balances<T> as Currency<_>>::make_free_balance_be(&recipient, existential_deposit);
		let transfer_amount = existential_deposit.saturating_mul(ED_MULTIPLIER.into());
	}: transfer(RawOrigin::Signed(caller.clone()), recipient_lookup, transfer_amount)
	verify {
		assert!(!Balances::<T>::free_balance(&caller).is_zero());
		assert!(!Balances::<T>::free_balance(&recipient).is_zero());
	}

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests_composite::{ExtBuilder, Test};
    use frame_support::assert_ok;

    #[test]
    fn transfer() {
        ExtBuilder::default().build().execute_with(|| {
            assert_ok!(test_benchmark_transfer::<Test>());
        });
    }
}
