extern crate miniscript;

use std::collections::VecDeque;

use miniscript::prelude::*;

fn main() {
    let mut int = 4u32;
    let mut data = serialize(&int).unwrap();

    println!("Data size: {}", data.len());
    for byte in data.iter() {
        println!("Byte value: {:X}", byte);
    }
    println!("creaitng deque...");
    let mut deque = VecDeque::from(data);
    let mut vec: Vec<u8> = Vec::new();
    for _i in 0..3 {
        let opt = deque.pop_front();
        if opt.is_none() {
            println!("Error! Is none!");
        } else {
            let val = opt.unwrap();
            vec.push(val);
        }
    }
    //vec.reverse();

    int = deserialize(&vec).unwrap();
    if int == 4 {
        println!("Deserialize worked.");
    }
    int += 1;
    data = serialize(&int).unwrap();

    println!("Data size: {}", data.len());
    for byte in data.iter() {
        println!("Byte value: {:X}", byte);
    }
}
