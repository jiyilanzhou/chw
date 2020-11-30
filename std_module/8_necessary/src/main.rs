/*
0. 反射




*/

use std::any::{Any, TypeId};

#[derive(Debug)]
enum E { H, He, Li }

#[derive(Debug)]
struct S { x: u8, y: u8, z: u16 }

fn main() {
    let v1 = 0xc0ffee_u32;
    let mut a: &Any; // trait object
    a = &v1;
    assert!(a.is::<u32>());
    // TypeId { t: 12849923012446332737 }
    // Some(12648430)
    println!("{:?}", TypeId::of::<u32>());
    println!("{:?}", a.downcast_ref::<u32>());

    let v2 = E::He;
    a = &v2;
    assert!(a.is::<E>());
    // TypeId { t: 14196257485762751284 }
    // Some(He)
    println!("{:?}", TypeId::of::<E>());
    println!("{:?}", a.downcast_ref::<E>());

    let v3 = S { x: 0xde, y: 0xad, z: 0xbeef };
    a = &v3;
    assert!(a.is::<S>());
    // TypeId { t: 18433843556634687139 }
    // Some(S { x: 222, y: 173, z: 48879 })    println!("{:?}", TypeId::of::<S>());
    println!("{:?}", a.downcast_ref::<S>());

    let v4 = "rust";
    a = &v4;
    assert!(a.is::<&str>());
    // TypeId { t: 9147559743429524724 }
    // Some("rust")
    println!("{:?}", TypeId::of::<&str>());
    println!("{:?}", a.downcast_ref::<&str>());
}