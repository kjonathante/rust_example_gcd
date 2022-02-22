use std::env;
fn main() {
    for arg in env::args().skip(1) {
        println!("{}", arg);
    }
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(12, 8), 4);
}

#[test]
fn test_gcd2() -> Result<(), String> {
    let x = gcd(2, 4);
    if x == 2 {
        Ok(())
    } else {
        Err(String::from("actual not equal expected"))
    }
}