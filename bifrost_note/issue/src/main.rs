/*
定点数(fixed-point number)：
	就是小数点位置固定的数，也就是说，小数点后面的位数是固定的，比如要记录一笔账目，这些账目的数字都不会超过100，就可以
	使用2位小数位定点数来记录，比如99.99，2.30，75.28；如果要更精确，可使用3位小数位的定点数来记录，如7.668，38.235


 */

use sp_arithmetic::{FixedU128,FixedPointOperand, FixedPointNumber, Permill};
use std::ops::Div;


fn main() {
	// let (a, b) = (FixedU128::from_inner(3u128), FixedU128::from_inner(6u128));
	//
	// //let c = a.saturating_div_int(b);
	// let c = FixedPointNumber::saturating_div_int(a,b);
	// println!("{}",c);
	
	let a = Permill::from_perthousand(42);
	let b = Permill::from_perthousand(33);
	let c = a.div(b);
	let d = 3;
	
	println!("{}",68.0/3 as f32);
	"43".parse::<f64>().is_ok();
	if let Ok(reward) = "42".parse::<f64>()  {
		
		println!("{}",reward);
	};
	
	
}