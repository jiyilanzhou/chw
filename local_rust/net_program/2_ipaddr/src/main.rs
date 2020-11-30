use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/* std::net::IpAddr 源码
        pub enum IpAddr {
            V4(Ipv4Addr),
            V6(Ipv6Addr),
        }
 */

fn main() {
    let v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));

    assert_eq!("127.0.0.1".parse(), Ok(v4));
    assert_eq!("::1".parse(), Ok(v6));
    // 判断是否为回环地址
    assert_eq!(v4.is_loopback(), true);
    assert_eq!(v6.is_loopback(), true);
    assert_eq!(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)).is_loopback(), false);
    // 判断是否为广播地址
    assert_eq!(v4.is_multicast(), false);
    assert_eq!(v6.is_multicast(), false);
    assert_eq!(IpAddr::V4(Ipv4Addr::new(224, 254, 0, 0)).is_multicast(), true);

    assert_eq!(v4.is_ipv4(), true);
    assert_eq!(v6.is_ipv6(), true);

    assert_eq!(v4.is_ipv6(), false);
    assert_eq!(v6.is_ipv4(), false);
}
