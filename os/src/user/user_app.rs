use super::*;

// pub fn hello_world() {
//     printlnu!("hello_world");
//     sys_exit(0);
// }

// pub fn count_sum() {
//     let array = [1, 2, 3, 4, 5];
//     let mut sum = 0;
//     for i in array.iter() {
//         sum += i;
//     }
//     printlnu!("count_sum");
//     printlnu!("sum = {}", sum);
//     sys_exit(0);
// }

pub fn power_3() {
    const LEN: usize = 100;
    let p = 3u64;
    let m = 998244353u64;
    let iter: usize = 200000;
    let mut s = [0u64; LEN];
    let mut cur = 0usize;
    s[cur] = 1;
    for i in 1..=iter {
        let next = if cur + 1 == LEN { 0 } else { cur + 1 };
        s[next] = s[cur] * p % m;
        cur = next;
        if i % 10000 == 0 {
            printlnu!("power_3 [{}/{}]", i, iter);
        }
    }
    printlnu!("{}^{} = {}", p, iter, s[cur]);
    printlnu!("Test power_3 OK!");
    sys_exit(0);
}

pub fn power_5() {
    const LEN: usize = 100;
    let p = 5u64;
    let m = 998244353u64;
    let iter: usize = 140000;
    let mut s = [0u64; LEN];
    let mut cur = 0usize;
    s[cur] = 1;
    for i in 1..=iter {
        let next = if cur + 1 == LEN { 0 } else { cur + 1 };
        s[next] = s[cur] * p % m;
        cur = next;
        if i % 10000 == 0 {
            printlnu!("power_5 [{}/{}]", i, iter);
        }
    }
    printlnu!("{}^{} = {}", p, iter, s[cur]);
    printlnu!("Test power_5 OK!");
    sys_exit(0);
}

pub fn power_7() {
    const LEN: usize = 100;
    let p = 7u64;
    let m = 998244353u64;
    let iter: usize = 160000;
    let mut s = [0u64; LEN];
    let mut cur = 0usize;
    s[cur] = 1;
    for i in 1..=iter {
        let next = if cur + 1 == LEN { 0 } else { cur + 1 };
        s[next] = s[cur] * p % m;
        cur = next;
        if i % 10000 == 0 {
            printlnu!("power_7 [{}/{}]", i, iter);
        }
    }
    printlnu!("{}^{} = {}", p, iter, s[cur]);
    printlnu!("Test power_7 OK!");
    sys_exit(0);
}

pub fn sleep() {
    let current_timer = sys_get_time();
    let wait_for = current_timer + 10000000;
    while sys_get_time() < wait_for {
        sys_yield();
    }
    printlnu!("Test sleep OK!");
    sys_exit(0);
}

