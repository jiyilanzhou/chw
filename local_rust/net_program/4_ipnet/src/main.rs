
/*
0. 无类别域间路由：CIDR(Classless Inter-Domain Routing)
    第三方 ipnet 包提供与 ip 相关的一系列操作
 */
use ipnet::{IpNet, Ipv4Net, Ipv6Net};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    /* 编译报错：因返回的 Result 类型不兼容(故不可使用问号表达式)
       error[E0277]: `?` couldn't convert the error to `std::io::Error`
         fn main() -> std::io::Result<()> {
                      ------------------- expected `std::io::Error` because of this
             let _v4 = Ipv4Net::new(Ipv4Addr::new(10, 1, 1, 0), 24)?;
                                                                   ^ the trait `std::convert
                       ::From<ipnet::PrefixLenError>` is not implemented for `std::io::Error`
          = note: the question mark operation (`?`) implicitly performs a conversion on the
                                                            error value using the `From` trait
          = help: the following implementations were found:
                    <std::io::Error as std::convert::From<std::ffi::NulError>>
                    <std::io::Error as std::convert::From<std::io::ErrorKind>>
                    <std::io::Error as std::convert::From<std::io::IntoInnerError<W>>>
          = note: required by `std::convert::From::from`
       // 要求"std::io::Result<()>"却返回"Result<Ipv4Net, PrefixLenError>"故才用 unwrap 提取

     */
    let _v4 = Ipv4Net::new(Ipv4Addr::new(10, 1, 1, 0), 24)?;
    // let _v4 = Ipv4Net::new(Ipv4Addr::new(10, 1, 1, 0), 24).unwrap();
    let _v6 = Ipv6Net::new(Ipv6Addr::new(0xfd, 0, 0, 0, 0, 0, 0, 0), 24).unwrap();

    // 从字符串创建 ipnet
    let _v4 = Ipv4Net::from_str("10.1.1.0/24").unwrap();
    let _v6 = Ipv6Net::from_str("fd00::/24").unwrap();

    let v4: Ipv4Net = "10.1.1.0/24".parse().unwrap();
    let _v6: Ipv6Net = "fd00::/24".parse().unwrap();

    let _net = IpNet::from(v4);

    let _net = IpNet::from_str("10.1.1.0/24").unwrap();
    let net: IpNet = "10.1.1.0/24".parse().unwrap();

    println!("{}, hostmask = {}", net, net.hostmask());
    println!("{}, netmask = {}", net, net.netmask());

    // 前缀
    assert_eq!(
        "192.168.12.34/16".parse::<IpNet>().unwrap().trunc(),
        "192.168.0.0/16".parse().unwrap()
    );

    Ok(())
}
