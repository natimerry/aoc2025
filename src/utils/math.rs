pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a.abs()
}

pub fn lcm(a: i64, b: i64) -> i64 {
    (a / gcd(a, b)) * b
}

pub fn manhattan(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

pub fn clamp<T: Ord>(x: T, lo: T, hi: T) -> T {
    if x < lo { lo } else if x > hi { hi } else { x }
}

pub fn sign(x: i64) -> i64 {
    match x {
        x if x > 0 => 1,
        x if x < 0 => -1,
        _ => 0,
    }
}

pub fn wrap_add(x: i64, delta: i64, m: i64) -> i64 {
    (x + delta).rem_euclid(m+1)
}