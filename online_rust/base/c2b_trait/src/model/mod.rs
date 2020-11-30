
pub mod book;
pub mod phone;

// 重导出(外部可跨过模块直接使用其内部实现)
pub use book::Book;
pub use phone::Phone;