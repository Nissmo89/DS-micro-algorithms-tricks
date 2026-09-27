fn mul(a: [[u64; 2]; 2], b: [[u64; 2]; 2]) -> [[u64; 2]; 2] {
    [
        [a[0][0] * b[0][0] + a[0][1] * b[1][0], a[0][0] * b[0][1] + a[0][1] * b[1][1]],
        [a[1][0] * b[0][0] + a[1][1] * b[1][0], a[1][0] * b[0][1] + a[1][1] * b[1][1]],
    ]
}

fn fib(n: u64) -> u64 {
    let mut res = [[1u64, 0u64], [0u64, 1u64]]; // identity
    let mut base = [[1u64, 1u64], [1u64, 0u64]];
    let mut exp = n;
    while exp > 0 {
        if exp & 1 == 1 {
            res = mul(res, base);
        }
        base = mul(base, base);
        exp >>= 1;
    }
    res[0][1]
}

fn main() {
    for i in 0..10 {
        println!("fib({}) = {}", i, fib(i));
    }
}
