
/*
0. no method named `saturating_mul` found
	a. 编译错误信息
		error[E0599]: no method named `saturating_mul` found for associated type
														`<T as Trait>::Balance` in the current scope
		      let reward = referer.record_amount.saturating_mul(staking_profit) / sum;
		                                         ^^^^^ method not found in `<T as Trait>::Balance`
		    = help: items from traits can only be used if the trait is in scope
		help: the following trait is implemented but not in scope; perhaps add a `use` for it:
		    |
		    | use sp_runtime::traits::Saturating;		// 留心编译器给出的" help "(预解决方案)

	b. 分析：外部 trait 即" T::Balance "未找到相应的" saturating_mul "方法，原因即是为外部模块未自定义实现相应的
			方法(但可能 继承或实现父 trait 的" saturating_mul "方法[使用则必须导入相应的" 父 Trait "])
	c. 解决方案 1 ：为 T::Balance 自定义实现" saturating_mul "方法(此种方案弃用[因 Balance 属于外部模块])
	   解决方案 2 ：为 T::Balance 导入其" 父 Trait "实现(注意错误信息中系统给出的" 预解决方案 ")如在文件中导入:
	   			  " use sp_runtime::traits::Saturating;"


 */