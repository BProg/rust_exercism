use nth_prime as np;
use std::time::{Instant, Duration};

#[test]
fn test_first_prime() {
    assert_eq!(np::nth(0), 2);
}

#[test]
#[ignore]
fn test_second_prime() {
    assert_eq!(np::nth(1), 3);
}

#[test]
#[ignore]
fn test_sixth_prime() {
    assert_eq!(np::nth(5), 13);
}

#[test]
#[ignore]
fn test_big_prime() {
    let now = Instant::now();
    assert_eq!(np::nth(10_000), 104_743);
    println!("{}", now.elapsed().as_millis());
}
