type Seconds = f32;
type Minutes = f32;

fn seconds_to_minutes(seconds: Seconds) -> Minutes {
    seconds / 60.0
}

fn main() {
    let s: Seconds = 120.0;
    let m: Minutes = seconds_to_minutes(s);

    println!("{s} seconds is {m} minutes");
}

#[cfg(test)]
mod testing {
    use crate::seconds_to_minutes;
    #[test]
    fn less_then_a_minute() {
        assert_eq!(0.75, seconds_to_minutes(45.0));
        assert_eq!(0.50, seconds_to_minutes(30.0));
        assert_eq!(0.25, seconds_to_minutes(15.0));
    }
    #[test]
    fn zero() {
        assert_eq!(0.0, seconds_to_minutes(0.0));
    }
    #[test]
    fn many_minutes() {
        assert_eq!(60.0, seconds_to_minutes(3600.0));
        assert_eq!(600.0, seconds_to_minutes(36000.0));
        assert_eq!(60.5, seconds_to_minutes(3630.0));
    }
    #[test]
    fn nice() {
        assert_eq!(69.0, seconds_to_minutes(4140.0));
    }
    #[test]
    fn forty_two() {
        assert_eq!(42.0, seconds_to_minutes(2520.0));
    }
    #[test]
    fn blaze_it() {
        assert_eq!(420.0, seconds_to_minutes(25200.0));
    }
    #[test]
    fn negative() {
        assert_eq!(-60.0, seconds_to_minutes(-3600.0));
        assert_eq!(-600.0, seconds_to_minutes(-36000.0));
        assert_eq!(-60.5, seconds_to_minutes(-3630.0));
    }
}
