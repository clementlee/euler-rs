fn get_days(year: u32, month: u32) -> u32 {
    match month {
        4 | 6 | 9 | 11 => 30,
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        2 => {
            if year % 4 == 0 && (year % 100 != 0) {
                29
            } else if year % 400 == 0 {
                29
            } else {
                28
            }
        }
        _n => panic!("bad"),
    }
}

pub fn solution() -> u32 {
    let mut count = 0;
    let mut day = 1;
    for year in 1900..=2000 {
        for month in 1..=12 {
            if day % 7 == 0 && year >= 1901 {
                // why this 1901 condition ugh.
                count += 1
            }
            day = (day + get_days(year, month)) % 7;
        }
    }

    count
}
