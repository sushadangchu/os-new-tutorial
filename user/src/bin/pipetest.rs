#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;

use alloc::string::String;
use user_lib::{
    fork,
    pipe,
    close,
    write,
    read,
};

#[no_mangle]
pub fn main() -> i32 {
    let (mut read_fd, mut write_fd) = pipe();
    if fork() == 0 {
        //close(write_fd);
        println!("child");
        let mut buffer = [0u8; 64];
        let len = read(read_fd, &mut buffer);
        let mut res = String::new();
        for ch in buffer.iter() {
            res.push(*ch as char);
        }
        println!("{}", res);
    } else {
        println!("parent");
        //close(read_fd);
        write(write_fd, b"hello_world");
    }
    0
}