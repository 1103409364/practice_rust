pub fn square(s: u32) -> u64 {
    // todo!("grains of rice on square {s}");
    // match s {
    //     // n if n > 64 || n == 0 => panic!("square out of range"),
    //     0 | 65.. => panic!("square out of range"),
    //     _ => (1..=s).fold(1u64, |acc, cur| if cur == 1 { acc } else { 2u64 * acc }),
    // }
    // 状态转移方程 f(n) = f(n-1) * 2 通项公式：f(n) = f(0) * 2^n 如果 f(0) = 1，那么通项公式简化为：f(n) = 2^n
    2u64.pow(s as u32 - 1)
}

pub fn total() -> u64 {
    // todo!();
    (1..=64).fold(0u64, |acc, cur| acc + square(cur))
}
