#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;
use compro::mint::*;
use std::str::FromStr;

#[test]
fn test_fromstr() {
    let s = "12";
    assert_eq!(Mint::<usize>::from_str(s).unwrap().0, 12);
    let s = "O";
    Mint::<usize>::from_str(s).unwrap_err();
}

#[quickcheck]
fn pow(x: usize, y: u32) -> bool {
    match x.checked_pow(y) {
        None => true,
        Some(z) => z % MOD == Mint(x).pow(y as usize).0,
    }
}

#[quickcheck]
fn inv(x: usize) {
    if x == 0 {
        return;
    }
    assert_eq!((Mint(x).inv() * x).0, 1, "x: {}", x);
}

#[quickcheck]
fn add_usize(x: usize, y: usize) -> bool {
    match x.checked_add(y) {
        None => true,
        Some(z) => z % MOD == (Mint(x) + y).0,
    }
}
#[quickcheck]
fn add(x: usize, y: usize) -> bool {
    match x.checked_add(y) {
        None => true,
        Some(z) => z % MOD == (Mint(x) + Mint(y)).0,
    }
}
#[quickcheck]
fn add_assign_usize(x: usize, y: usize) -> bool {
    let mut mx = Mint(x);
    match x.checked_add(y) {
        None => true,
        Some(z) => {
            mx += y;
            z % MOD == mx.0
        }
    }
}

#[quickcheck]
fn add_assign(x: usize, y: usize) -> bool {
    let mut mx = Mint(x);
    let my = Mint(y);
    match x.checked_add(y) {
        None => true,
        Some(z) => {
            mx += my;
            z % MOD == mx.0
        }
    }
}

#[quickcheck]
fn sub_usize(x: usize, y: usize) -> bool {
    let z = ((x as i64) - (y as i64)) % (MOD as i64);
    let z = if z < 0 { z + (MOD as i64) } else { z };
    z as usize == (Mint(x) - y).0
}

#[quickcheck]
fn sub(x: usize, y: usize) -> bool {
    let z = ((x as i64) - (y as i64)) % (MOD as i64);
    let z = if z < 0 { z + (MOD as i64) } else { z };
    z as usize == (Mint(x) - Mint(y)).0
}

#[quickcheck]
fn sub_assign_usize(x: usize, y: usize) -> bool {
    let z = ((x as i64) - (y as i64)) % (MOD as i64);
    let z = if z < 0 { z + (MOD as i64) } else { z };
    let mut mx = Mint(x);
    mx -= y;
    z as usize == mx.0
}

#[quickcheck]
fn sub_assign(x: usize, y: usize) -> bool {
    let z = ((x as i64) - (y as i64)) % (MOD as i64);
    let z = if z < 0 { z + (MOD as i64) } else { z };
    let mut mx = Mint(x);
    mx -= Mint(y);
    z as usize == mx.0
}

#[quickcheck]
fn mul_usize(x: usize, y: usize) -> bool {
    match x.checked_mul(y) {
        None => true,
        Some(z) => z % MOD == (Mint(x) * y).0,
    }
}

#[quickcheck]
fn mul(x: usize, y: usize) -> bool {
    match x.checked_mul(y) {
        None => true,
        Some(z) => z % MOD == (Mint(x) * Mint(y)).0,
    }
}

#[quickcheck]
fn div_usize(x: usize, y: usize) -> bool {
    if y == 0 {
        return true;
    }
    let mz = Mint(x) / y;
    (mz * y).0 == x % MOD
}

#[quickcheck]
fn div(x: usize, y: usize) -> bool {
    if y == 0 {
        return true;
    }
    let mz = Mint(x) / Mint(y);
    (mz * y).0 == x % MOD
}

#[quickcheck]
fn div_assign_usize(x: usize, y: usize) -> bool {
    if y == 0 {
        return true;
    }
    let mut mx = Mint(x);
    mx /= y;
    (mx * y).0 == x % MOD
}

#[quickcheck]
fn div_assign(x: usize, y: usize) -> bool {
    if y == 0 {
        return true;
    }
    let mut mx = Mint(x);
    mx /= Mint(y);
    (mx * y).0 == x % MOD
}
