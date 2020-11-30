
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // 调用 send 时会发生 move 故不可再用 val
        //println!("val = {}", val);
    });

    let re = rx.recv().unwrap();
    println!("got: {}", re);

}
