// fn hamming_distance(source: u64, target: u64) -> u32 {
//     (source ^ target).count_ones()
// }

// fn main() {
//     let source = 1;
//     let target = 2;
//     let distance = hamming_distance(source, target);
//     println!("the hamming distance is {distance}");
// }

// hamming_distance.rs
fn hamming_distance_str(source: &str, target: &str) -> u32 {
    let mut count = 0;
    let mut source = source.chars();
    let mut target = target.chars();
    println!("{:?} - {:?}", source, target);
    // 两字符串逐字符比较可能出现如下四种情况
    loop {
        match (source.next(), target.next()) {
            (Some(cs), Some(ct)) if cs != ct => count += 1,
            (Some(_), None) | (None, Some(_)) => panic!("Must have the same length"),
            (None, None) => break,
            _ => continue,
        }
    }

    count as u32
}

fn main() {
    let source = "abce";
    let target = "edcf";
    let distance = hamming_distance_str(source, target);
    println!("the hamming distance is {distance}");
}
