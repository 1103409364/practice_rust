pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    // Alice, Bob, Charlie, David, Eve, Fred, Ginny, Harriet, Ileana, Joseph, Kincaid, and Larry.
    const STUDENTS: [&str; 12] = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];

    let index = STUDENTS.iter().position(|&s| s == student).unwrap();

    diagram.split_whitespace().fold(vec![], |mut acc, c| {
        acc.extend(
            &c[index * 2..index * 2 + 2]
                .chars()
                .map(|c| match c.to_string().as_str() {
                    "G" => "grass",
                    "C" => "clover",
                    "R" => "radishes",
                    "V" => "violets",
                    _ => panic!("Invalid character in diagram"),
                })
                .collect::<Vec<&str>>(),
        );
        acc
    })
}
