use super::*;

pub fn hello_world() {
    printlnu!("hello_world");
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

pub fn write_a() {
    let width: usize = 10;
    let height: usize = 5;
    for i in 0..height {
        for _ in 0..width { printu!("A"); }
        printlnu!(" [{}/{}]", i + 1, height);
        //sys_yield();
    }
    printlnu!("Test write_a OK!");
    sys_exit(0);
}

pub fn write_b() {
    let width: usize = 10;
    let height: usize = 5;
    for i in 0..height {
        for _ in 0..width { printu!("B"); }
        printlnu!(" [{}/{}]", i + 1, height);
        //sys_yield();
    }
    printlnu!("Test write_b OK!");
    sys_exit(0);
}

pub fn write_c() {
    let width: usize = 10;
    let height: usize = 3;
    for i in 0..height {
        for _ in 0..width { printu!("C"); }
        printlnu!(" [{}/{}]", i + 1, height);
        //sys_yield();
    }
    printlnu!("Test write_c OK!");
    sys_exit(0);
}