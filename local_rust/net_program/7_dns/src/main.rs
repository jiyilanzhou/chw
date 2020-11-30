use std::env;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;
use trust_dns::rr::record_type::RecordType;

/*
0. trust-dns
    文档" https://crates.io/crates/trust-dns "

1. trust-dns-resolver
    文档" https://crates.io/crates/trust-dns-resolver "

// 注： " trust-dns "及" trust-dns-resolver "皆可于 crates.io 中搜索" dns "获取

*/

// 运行 如" cargo run baidu.com "
fn main() {
    // 从命令行读取参数
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Please provide a name to query!");
        std::process::exit(1);
    }
    let query = format!("{}.", args[1]);
    println!("\nuse default config:");
    let resolver =
        // ResolverConfig、ResolverOpts 均使用默认配置
        Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    let response = resolver.lookup_ip(query.as_str());
    for ans in response.iter() {
        println!("{:?}", ans);
    }

    println!("\nuse system config:");
    // 从系统默认配置文件创建解析器
    let resolver =
        Resolver::from_system_conf().unwrap();
    let response = resolver.lookup_ip(query.as_str());
    for ans in response.iter() {
        println!("{:?}", ans);
    }

    println!("\nuse ns:");
    let ns = resolver.lookup(query.as_str(), RecordType::NS);
    for ans in ns.iter() {
        println!("{:?}", ans);
    }

}
