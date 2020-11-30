use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::{EthernetPacket, EtherTypes};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::Packet;

use std::env;

/*
0. pnet
    文档" https://crates.io/crates/pnet "
    libpnet 提供跨平台 API (用于底层网络连接)

1. 编译、运行
    a. 编译
        cargo build
    b. 输入网卡名运行
       # (进入" ./target/debug "目录执行" sudo ./可执行文件  网上名 ")
       root@ubuntu:.../target/debug # sudo ./use_pnet ens33

*/

fn main() {
    // 获取网卡
    let interface_name = env::args().nth(1).unwrap();
    // 网卡列表
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .filter(|iface: &NetworkInterface| iface.name == interface_name)
        .next()
        .expect("Error get interface");
    // 获取数据链路层的接收端、发送端
    let (_tx, mut rx) =
        match datalink::channel(&interface, Default::default()) {
            // 以太网
            Ok(Ethernet(tx, rx)) => (tx, rx),
            // 其它网络
            Ok(_) => panic!("Other"),
            Err(e) => panic!("error: {}", e),
        };
    // 遍历接收端(类比" for i in rx { //... } ")
    loop {
        match rx.next() {
            Ok(packet) => {
                let packet = EthernetPacket::new(packet).unwrap();
                // 解包
                handle_packet(&packet);
            }
            Err(e) => {
                println!("Some error: {}", e);
            }
        }
    }
}
// 处理包
fn handle_packet(ethernet: &EthernetPacket) {
    match ethernet.get_ethertype() {
        // Ipv4 以太网类型
        EtherTypes::Ipv4 => {
            let header = Ipv4Packet::new(ethernet.payload());
            if let Some(header) = header {
                match header.get_next_level_protocol() {
                    // 处理 tcp 协议(传输层解析)
                    IpNextHeaderProtocols::Tcp => {
                        let tcp = TcpPacket::new(header.payload());
                        if let Some(tcp) = tcp {
                            println!("Tcp packet {}: {} to {}: {}",
                                     header.get_source(),
                                     tcp.get_source(),
                                     header.get_destination(),
                                     tcp.get_destination());
                        }
                    }
                    // 忽略非 tcp 协议
                    _ => println!("ignore"),
                }
            }
        }
        // 忽略其余类型
        _ => println!("ignoring"),
    }
}