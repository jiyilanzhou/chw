// bifrost['bi:frɔst]n.([北欧神话中]连接天宫和大地的)彩虹桥
// telemetry[təˈlemətri]n. 遥测(技术)
// stake[steɪk]n.赌注,桩,股份(奖金)    // PoS 股权证明
/*
0. Bifrost

1. brml : Bifrost Runtime Module Library (比拟" SRML ")

2. 关联类型
    /// Instantiate all Full RPC extensions.
    pub fn create_full<C, P, SC>(
        deps: FullDeps<C, P, SC>,
    ) -> jsonrpc_core::IoHandler<sc_rpc_api::Metadata> where
        C: ProvideRuntimeApi<Block>,
        C: HeaderBackend<Block> + HeaderMetadata<Block, Error=BlockChainError> + 'static,
        C: Send + Sync + 'static,
        C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
    {}
    // C 为泛型：从约束可知" C::Api "源于" C "的约束类型

3. EOS(Enterprise Operation System)智能合约开发
      智能合约(Smart contract)

 */

use chrono::prelude::*;

fn main() {
	let bl = leap_year_judge();
	println!("{}",bl);
	
	
}

pub fn is_leap_year_interval() -> bool {
	// Function define
	fn is_leap_year(year: u32) -> bool {
		year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
	}
	// Check judge
	let local: DateTime<Utc> = Utc::now();
	let mut year = 100u32;
	if let Ok(x) = local.format("%Y").to_string().parse::<u32>() {
		year = x;
	};
	is_leap_year(year - 1) || is_leap_year(year)
}

