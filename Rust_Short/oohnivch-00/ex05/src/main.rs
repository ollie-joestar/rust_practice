
fn  is_leap_year(year: u32) -> bool {
    if year == 0 {
        panic!("Bad year number");
    }
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn  num_days_in_month(year: u32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => 28 + (is_leap_year(year) as u32),
        _ => panic!("Bad month"),
    }
}

fn  main() {
    let mut day:u32 = 1;
    for year in 1..2025 {
        for month in 1..13 {
            if year == 2024 && month > 9 {
            break;
            }
            if day % 7 == 0 {
                match month {
                    1 => println!("Friday, January 13, {}", year),
                    2 => println!("Friday, February 13, {}", year),
                    3 => println!("Friday, March 13, {}", year),
                    4 => println!("Friday, April 13, {}", year),
                    5 => println!("Friday, May 13, {}", year),
                    6 => println!("Friday, June 13, {}", year),
                    7 => println!("Friday, July 13, {}", year),
                    8 => println!("Friday, August 13, {}", year),
                    9 => println!("Friday, September 13, {}", year),
                    10 => println!("Friday, October 13, {}", year),
                    11 => println!("Friday, November 13, {}", year),
                    12 => println!("Friday, December 13, {}", year),
                    _ => panic!("Bad month"),
                }
            }
            day = (day + num_days_in_month(year, month)) % 7;
        }
    }
}

#[cfg(test)]
mod testing {
    #[test]
    fn test_num_days_in_month() {
        assert_eq!(31, crate::num_days_in_month(1, 1));
        assert_eq!(31, crate::num_days_in_month(4, 1));
        assert_eq!(28, crate::num_days_in_month(1, 2));
        assert_eq!(29, crate::num_days_in_month(4, 2));
        assert_eq!(31, crate::num_days_in_month(1, 3));
        assert_eq!(31, crate::num_days_in_month(4, 3));
        assert_eq!(30, crate::num_days_in_month(1, 4));
        assert_eq!(30, crate::num_days_in_month(4, 4));
        assert_eq!(31, crate::num_days_in_month(1, 5));
        assert_eq!(31, crate::num_days_in_month(4, 5));
        assert_eq!(30, crate::num_days_in_month(1, 6));
        assert_eq!(30, crate::num_days_in_month(4, 6));
        assert_eq!(31, crate::num_days_in_month(1, 7));
        assert_eq!(31, crate::num_days_in_month(4, 7));
        assert_eq!(31, crate::num_days_in_month(1, 8));
        assert_eq!(31, crate::num_days_in_month(4, 8));
        assert_eq!(30, crate::num_days_in_month(1, 9));
        assert_eq!(30, crate::num_days_in_month(4, 9));
        assert_eq!(31, crate::num_days_in_month(1, 10));
        assert_eq!(31, crate::num_days_in_month(4, 10));
        assert_eq!(30, crate::num_days_in_month(1, 11));
        assert_eq!(30, crate::num_days_in_month(4, 11));
        assert_eq!(31, crate::num_days_in_month(1, 12));
        assert_eq!(31, crate::num_days_in_month(4, 12));
    }
    #[test]
    fn test_is_leap_year() {
        assert_eq!(true, crate::is_leap_year(1600));
        assert_eq!(false, crate::is_leap_year(1500));
        assert_eq!(true, crate::is_leap_year(2004));
        assert_eq!(false, crate::is_leap_year(2003));
    }
    #[test]
    #[should_panic]
    fn test_panic_year() {
        crate::is_leap_year(0);
    }
    #[test]
    #[should_panic]
    fn test_panic_zero_month() {
        crate::num_days_in_month(1, 0);
    }
    #[test]
    #[should_panic]
    fn test_panic_big_month() {
        crate::num_days_in_month(1, 13);
    }
}
