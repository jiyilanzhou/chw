
// 弱引用实现"树型"结构
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {       // 定义树型结构中的 Node 结点
    value: i32,
    // 子结点不一定拥有父结点(但可查询父结点)故用" Weak "
    parent: RefCell<Weak<Node>>,    // 父结点
    // 父结点拥有子结点故用" Rc "
    children: RefCell<Vec<Rc<Node>>>,   // 子结点
}

fn main() {
    // 叶子结点
    let leaf = Rc::new( Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf parent = {:#?}", leaf.parent.borrow().upgrade());// Console:"None"
    println!("---------------------------------------------------");

    // 枝结点
    let branch = Rc::new( Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // 叶子结点的父结点指向"枝结点"
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:#?}", leaf.parent.borrow().upgrade());

}
