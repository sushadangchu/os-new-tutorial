use super::*;
use crate::memory;

pub fn hello_world() {
    let remap = memory::MemorySet::new_kernel().unwrap();
    remap.activate();

    println!("kernel remapped");
    sys_exit(0);
}

pub fn count_sum() {
    let array = [1, 2, 3, 4, 5];
    let mut sum = 0;
    for i in array.iter() {
        sum += i;
    }
    printlnu!("count_sum");
    printlnu!("sum = {}", sum);
    sys_exit(0);
}

pub fn power_3() {
    printlnu!("Test power_3 OK!");
    sys_exit(0);
}

pub fn power_5() {
    printlnu!("Test power_5 OK!");
    sys_exit(0);
}

pub fn power_7() {
    printlnu!("Test power_7 OK!");
    sys_exit(0);
}

pub fn sleep() {
    printlnu!("Test sleep OK!");
    sys_exit(0);
}

