
/*
0. 向外暴露模块

1. 向外暴露模块内细节
 */
// 向外暴露模块
pub mod prod;
pub mod stock;

// (重导出)向外暴露模块内细节
pub use prod::*;
pub use stock::*;