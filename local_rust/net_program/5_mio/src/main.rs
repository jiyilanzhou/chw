
// poll[pəʊl]n.投票,轮询
/*
0. mio (文档" https://crates.io/crates/mio ")
    一个快速、非阻塞的 I/O 库,用于构建高性能 I/O 应用程序

 */
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};

const SERVER: Token = Token(0);
const CLIENT: Token = Token(1);

fn main() -> std::io::Result<()> {
    // 服务端
    let mut poll = Poll::new()?;
    // 监听的事件集合 128 个
    let mut events = Events::with_capacity(128);
    let addr = "127.0.0.1:8080".parse().unwrap();
    let mut server = TcpListener::bind(addr)?;
    // 注册到 poll
    poll.registry().register(&mut server, SERVER, Interest::READABLE)?;

    // 客户端
    let mut client = TcpStream::connect(addr)?;
    // Interest : 感兴趣的操作
    poll.registry().register(&mut client,
                             CLIENT,
                             Interest::READABLE | Interest::WRITABLE)?;

    // 轮询
    loop {
        poll.poll(&mut events, None)?;
        // 处理事件
        for event in events.iter() {
            match event.token() {
                SERVER => {
                    let connection = server.accept();
                    println!("SERVER recv a connection!");
                    drop(connection);
                }
                CLIENT => {
                    if event.is_writable() {
                        println!("CLIENT write");
                    }
                    if event.is_readable() {
                        println!("CLIENT read");
                    }
                    return Ok(())
                }
                _ => unreachable!(),
            }
        }
    }

}
