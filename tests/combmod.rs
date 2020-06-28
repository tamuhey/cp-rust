use compro::combmod::*;

#[test]
fn test_gcd() {
    assert_eq!(gcd(1, 2), 1);
    assert_eq!(gcd(2, 4), 2);
    assert_eq!(gcd(8, 44), 4);
}

#[test]
fn test_prime_factors() {
    assert_eq!(prime_factors(10), vec![2, 5]);
    assert_eq!(prime_factors(12387), vec![3, 4129]);
    assert_eq!(prime_factors(16), vec![2, 2, 2, 2]);
}

#[test]
fn test_combmod() {
    let combmod = CombMod::new(100000);
    assert_eq!(combmod.c(100, 50).0, 538992043);
    assert_eq!(combmod.c(10000, 3001).0, 981292794);
    assert_eq!(combmod.c(10000, 30001).0, 0);
    assert_eq!(combmod.p(10, 4).0, 5040);
    assert_eq!(combmod.p(30, 10).0, 349668811);
}
