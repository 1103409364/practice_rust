// dp_rec_mc.rs
fn dp_rec_mc(cashes: &[u32], amount: u32, min_cashes: &mut [u32]) -> u32 {
    // 动态 收 集从 1 到 amount 的 最 小 找 零 币 值 数 量
    // 然 后 从 小 到 大 凑 出 找 零 纸 币 数 量
    for denm in 1..=amount {
        let mut min_cashe_num = denm;
        for c in cashes.iter().filter(|&c| *c <= denm).collect::<Vec<&u32>>() {
            let index = (denm - c) as usize;

            let cashe_num = 1 + min_cashes[index];
            if cashe_num < min_cashe_num {
                min_cashe_num = cashe_num;
            }
        }
        min_cashes[denm as usize] = min_cashe_num;
    }

    // 因 为 收 集 了 各 个 值 的 最 小 找 零 纸 币 数 ， 所 以 直 接 返 回
    min_cashes[amount as usize]
}

fn main() {
    let amount = 81u32;
    let cashes = [1, 5, 10, 20, 50];
    let mut min_cashes: [u32; 82] = [0; 82]; // 声明
    let cash_num = dp_rec_mc(&cashes, amount, &mut min_cashes);
    println!("Refund for ￥{amount} need {cash_num} cashes");
}
