// TODO:优化，缓存已计算过的结果，状态转移方程 f(n) = f(n-1) * 2
pub fn square(s: u32) -> u64 {
    // todo!("grains of rice on square {s}");
    match s {
        // n if n > 64 || n == 0 => panic!("square out of range"),
        0 | 65.. => panic!("square out of range"),
        _ => (1..=s).fold(1u64, |acc, cur| if cur == 1 { acc } else { 2u64 * acc }),
    }
}

pub fn total() -> u64 {
    // todo!();
    (1..=64).fold(0u64, |acc, cur| acc + square(cur))
}
