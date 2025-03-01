pub fn is_leap_year(year: u64) -> bool {
    // todo!("true if {year} is a leap year")
    match (year % 4 == 0, year % 100 != 0) {
        (true, true) => true,
        _ => match year % 400 == 0 {
            true => true,
            _ => false,
        },
    }
}
