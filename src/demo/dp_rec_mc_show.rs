// dp_rc_mc_show.rs
// 使用 cashes_used 收 集 使 用 过 的 各 面 额 纸 币
fn dp_rec_mc_show(
    cashes: &[u32],
    amount: u32,
    min_cashes: &mut [u32],
    cashes_used: &mut [u32],
) -> u32 {
    for denm in 1..=amount {
        let mut min_cashe_num = denm;
        let mut used_cashe = 1; // 最小 面 额是 1 元
        for c in cashes.iter().filter(|&c| *c <= denm).collect::<Vec<&u32>>() {
            let index = (denm - c) as usize;
            let cashe_num = 1 + min_cashes[index];
            if cashe_num < min_cashe_num {
                min_cashe_num = cashe_num;
                used_cashe = *c;
            }
        }

        // 更 新 各 金 额 对 应 的 最 小 纸 币 数
        min_cashes[denm as usize] = min_cashe_num;
        // 记录使用过的纸币，只要记录当前金额使用的最后一个纸币即可。再用减去当前金额后的余额去找另一个最优解，通过这个最优解就可以找到下一个使用过的纸币。不断重复这个过程
        cashes_used[denm as usize] = used_cashe;
    }

    min_cashes[amount as usize]
}

// 打 印 输 出 各 面 额 纸 币
fn print_cashes(cashes_used: &[u32], mut amount: u32) {
    while amount > 0 {
        let curr = cashes_used[amount as usize];
        println!("￥{curr}");
        amount -= curr;
    }
}

fn main() {
    let amount = 81u32;
    let cashes = [1, 5, 10, 20, 50];
    let mut min_cashes: [u32; 82] = [0; 82];
    let mut cashes_used: [u32; 82] = [0; 82];
    let cs_num = dp_rec_mc_show(&cashes, amount, &mut min_cashes, &mut cashes_used);
    println!("Refund for ￥{amount} need {cs_num} cashes:");
    print_cashes(&cashes_used, amount);
}
