pub fn ext_gcd(a: i32, b: i32) -> (i32, i32) {
    if b == 0 {
        return (1, 0);
    }
    let (x, y) = ext_gcd(b, a % b);
    (y, x - a / b * y)
}
