use ac_library_rs::modint::Mod1000000007;
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
    let combmod = CombMod::<Mod1000000007>::new(100000);
    assert_eq!(combmod.c(100, 50), 538992043.into());
    assert_eq!(combmod.c(10000, 3001), 981292794.into());
    assert_eq!(combmod.c(10000, 30001), 0.into());
    assert_eq!(combmod.p(10, 4), 5040.into());
    assert_eq!(combmod.p(30, 10), 349668811.into());
}
