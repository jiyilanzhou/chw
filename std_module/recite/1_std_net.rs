
/*
" std::net "包

Struct std::net::TcpListener
0. Declaration
   a. pub struct TcpListener(net_imp::TcpListener);
   b. // 注:" use crate::sys_common::net as net_imp; "
   c. // 则 net_imp::TcpListener 为
         pub struct TcpListener {
            inner: Socket,
         }

1. Methods：impl TcpListener
   a. pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<TcpListener> {
        super::each_addr(addr, net_imp::TcpListener::bind).map(TcpListener)
      }
      // 注:" pub type Result<T> = result::Result<T, Error>; "

   b. pub fn incoming(&self) -> Incoming<'_> {
          Incoming { listener: self }
      }
      // 注:" Struct std::net::Incoming "
              pub struct Incoming<'a> {
                  listener: &'a TcpListener,
              }
      // An iterator that infinitely accepts connections on a TcpListener.
      // This struct is created by the incoming method on TcpListener.
         See its documentation for more


*/

/*
Struct std::net::TcpStream
0. Declaration
    a. pub struct TcpStream(net_imp::TcpStream);
    b. // 注:" use crate::sys_common::net as net_imp; "
    c. // 则 net_imp::TcpStream 为
        pub struct TcpStream {
            inner: Socket,
        }

1. Methods：impl TcpStream
    a. pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<TcpStream> {
          super::each_addr(addr, net_imp::TcpStream::connect).map(TcpStream)
       }


*/