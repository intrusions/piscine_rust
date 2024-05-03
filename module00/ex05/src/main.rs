fn is_leap_year(year: u32) -> bool {
    if year == 0 {
        panic!("invalid year");
    }

    return (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0);
}

fn days_in_february(year: u32) -> u32 {
    if is_leap_year(year) {
        return 29;
    }  
    return 28;
}

fn num_days_in_month(year: u32, month: u32) -> u32 {
    if !(month >= 1 && month <= 12) {
        panic!("invalid month");
    }

    match month {
        4 | 6 | 9 | 11                  => 30,
        1 | 3 | 5 | 7 | 8 | 10 | 12     => 31,
        2                               => days_in_february(year),
        _                               => panic!("invalid month"),
    }
}

fn month_u32_to_str(month: u32) -> String {
    return match month {
        1 => "January".to_string(),
        2 => "February".to_string(),
        3 => "March".to_string(),
        4 => "April".to_string(),
        5 => "May".to_string(),
        6 => "June".to_string(),
        7 => "July".to_string(),
        8 => "August".to_string(),
        9 => "September".to_string(),
        10 => "October".to_string(),
        11 => "November".to_string(),
        12 => "December".to_string(),
        _  => panic!("invalid month")
    }
}

fn main() {
    let mut total_days = 0;
    
    for year in 1..=6 {
        
        for month in 1..=12 {
            let num_days = num_days_in_month(year, month);
            
            for day in 1..=num_days {
                total_days += 1;
            
                if day == 13 && total_days % 7 == 5 {
                    println!("Friday, {} 13, {}", month_u32_to_str(month), year);
                }
            }
        }
    }
}
