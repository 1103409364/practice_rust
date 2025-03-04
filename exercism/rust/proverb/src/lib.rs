pub fn build_proverb(list: &[&str]) -> String {
    // todo!("build a proverb from this list of items: {list:?}")
    list.iter()
        .enumerate()
        .map(|(i, l)| {
            if i == list.len() - 1 {
                return format!("And all for the want of a {}.", list[0]);
            }
            format!("For want of a {} the {} was lost.", l, list[i + 1])
        })
        .collect::<Vec<String>>()
        .join("\n")
}
