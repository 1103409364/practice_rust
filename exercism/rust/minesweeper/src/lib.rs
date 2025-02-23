pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // todo!( "\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{minefield:#?}\n" );
    let v_source: Vec<Vec<String>> = minefield
        .iter()
        .map(|x| x.chars().map(|c| c.to_string()).collect::<Vec<String>>())
        .collect();

    let mut res = v_source.clone();

    v_source.iter().enumerate().for_each(|(i, v)| {
        v.iter().enumerate().for_each(|(j, c)| {
            if c == " " {
                let sum = sum_mines_j(&v_source, i as i32, j as i32);
                if sum > 0 {
                    res[i][j] = sum.to_string()
                }
            }
        });
    });

    res.iter().map(|v| v.join("")).collect::<Vec<String>>()
}
// 统计雷的数量
fn sum_mines_j(v: &Vec<Vec<String>>, i: i32, j: i32) -> i32 {
    // j 的上下左右和对角线
    [
        (i - 1, j - 1),
        (i - 1, j),
        (i - 1, j + 1),
        (i, j - 1),
        (i, j + 1),
        (i + 1, j - 1),
        (i + 1, j),
        (i + 1, j + 1),
    ]
    .iter()
    .fold(0, |acc, (x, y)| {
        if *x < 0 || *y < 0 || *x >= v.len() as i32 || *y >= v[0].len() as i32 {
            return acc;
        }
        if v[*x as usize][*y as usize] == "*" {
            acc + 1
        } else {
            acc
        }
    })
}
