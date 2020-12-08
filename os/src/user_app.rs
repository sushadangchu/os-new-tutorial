use crate::syscall::exit;

pub fn hello_world() {
    println!("hello world!");
    exit();
}

pub fn count_sum(array: [usize;5]) {
    let mut sum = 0;
    for i in array.iter() {
        sum += i;
    }
    println!("sum = {}", sum);
    exit();
}