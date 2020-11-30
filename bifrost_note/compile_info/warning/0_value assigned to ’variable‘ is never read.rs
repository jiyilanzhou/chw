
// I have the following simple code:
fn main() {
    let mut foo = String::new();
    foo = "Hi!".to_string();

    println!("{}", foo);
}

/*
warning: value assigned to `foo` is never read
 --> src\main.rs:2:6
  |
2 |     let mut foo = String::new();
  |         ^^^^^^^
  |
  = note: #[warn(unused_assignments)] on by default

 */

/*
This warning seems to be wrong to me. The assignment foo has been used in the println! macro. So I should not get this error. When I remove the mutable assignment as follows, I get no warning as expected:

fn main() {
	let foo = "Hi!".to_string();
	println!("{}", foo);
}

 */
