/*




 */

/*
0. 十六进行转为 u8 切片

 */
use hex;
use sp_core::crypto::{Ss58Codec, AccountId32, PublicError, Ss58AddressFormat};

fn main() {
    // 0. Encoding a String
    let hex_string = hex::encode("Hello world!");
    println!("{}", hex_string); // Prints "48656c6c6f20776f726c6421"
    // 1. Decoding a String
    if let Ok(decoded_string) = hex::decode("48656c6c6f20776f726c6421"){
        // Todo Vec<u8> 如何转为 string
        //println!("{:?}", decoded_string); // Prints "Hello world!"
    }

    // 2.
    let input = "090A0B0C";
    let decoded = hex::decode(input);
    println!("{:?}", decoded);

    // 3.
    let mut bytes = [0u8; 12];
    assert_eq!(hex::decode_to_slice("48656c6c6f20776f726c6421", &mut bytes as &mut [u8]), Ok(()));
    assert_eq!(&bytes, b"Hello world!");
}



