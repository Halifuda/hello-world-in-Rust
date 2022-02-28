use std::env;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");

    let mut nums = Vec::new();
    for arg in env::args().skip(1) {
        nums.push(u64::from_str(&arg).expect("Parsing Error"));
    }
    if nums.len() < 2 {
        eprintln!("Use by : main num0 num1 ...");
        std::process::exit(1);
    } 
    let mut res = nums[0];
    for num in &nums[1..] {
        res = gcd_loop(res, *num);
    }
    println!("gcd({:?}) -> {}", nums, res);
}

fn gcd_loop(mut n : u64, mut m : u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m %= n;
    }
    n
}

#[test]
fn test_gcd_loop() {
    assert_eq!(gcd_loop(13, 15), 1);
    assert_eq!(gcd_loop(3 * 5 * 7, 7 * 11 * 13), 7);
}