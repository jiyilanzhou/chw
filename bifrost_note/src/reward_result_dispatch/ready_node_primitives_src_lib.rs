
use frame_support::dispatch::DispatchResult;

/*pub trait RewardTrait<Balance, AccountId> {
	type Error;
	fn record_reward(vtoken_symbol: TokenSymbol, convert_amount: Balance, referer: AccountId) -> Result<(), Self::Error>;
	fn dispatch_reward(vtoken_symbol: TokenSymbol, staking_profit: Balance) -> Result<(), Self::Error>;
}

impl<A, B> RewardTrait<A, B> for () {
	type Error = core::convert::Infallible;
	fn record_reward(_: TokenSymbol, _: A, _: B) -> Result<(), Self::Error> { Ok(Default::default()) }
	fn dispatch_reward(_: TokenSymbol, _: A) -> Result<(), Self::Error> { Ok(Default::default()) }
}*/

pub trait RewardTrait<Balance, AccountId> {
	type Error;
	fn record_reward(vtoken_symbol: TokenSymbol, convert_amount: Balance, referer: AccountId) -> DispatchResult;
	fn dispatch_reward(vtoken_symbol: TokenSymbol, staking_profit: Balance) -> DispatchResult;
}

impl<A, B> RewardTrait<A, B> for () {
	type Error = core::convert::Infallible;
	fn record_reward(_: TokenSymbol, _: A, _: B) -> DispatchResult { Ok(Default::default()) }
	fn dispatch_reward(_: TokenSymbol, _: A) -> DispatchResult { Ok(Default::default()) }
}