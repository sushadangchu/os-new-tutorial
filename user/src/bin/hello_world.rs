#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{
    fork,
    wait,
    exec,
    yield_,
};

#[no_mangle]
pub fn main() -> i32 {
    if fork() == 0 {
        println!("child");
        exec("yield");
    } else {
        println!("parent");
    }
    println!("Hello world from user mode program!");
    0
}