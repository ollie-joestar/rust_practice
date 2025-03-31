use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
struct Time {
    hours: u32,
    minutes: u32,
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.hours, self.minutes) {
            (1, 1) => write!(f, "{} hour, {} minute", self.hours, self.minutes),
            (_, 1) => write!(f, "{} hours, {} minute", self.hours, self.minutes),
            (1, _) => write!(f, "{} hour, {} minutes", self.hours, self.minutes),
            (_, _) => write!(f, "{} hours, {} minutes", self.hours, self.minutes),
        }
    }
}

impl FromStr for Time {
    type Err = TimeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.contains(':') {
            return Err(TimeParseError::MissingColon);
        }
        let bytes = s.as_bytes();
        let mut colon = 0;
        for (i, &byte) in bytes.iter().enumerate() {
            if byte == b':' {
                colon = i;
            }
            if !byte.is_ascii_digit() && i != colon {
                return Err(TimeParseError::InvalidNumber);
            }
        }
        let hr_s = &s[0..colon].trim();
        let m_s = &s[colon + 1..].trim();
        if hr_s.len() != 2 || m_s.len() != 2 {
            return Err(TimeParseError::InvalidLength);
        }
        let hours = hr_s
            .parse::<u32>()
            .map_err(|_| TimeParseError::InvalidNumber)?;
        let minutes = m_s
            .parse::<u32>()
            .map_err(|_| TimeParseError::InvalidNumber)?;
        if hours >= 24 || minutes >= 60 {
            return Err(TimeParseError::InvalidNumber);
        }
        Ok(Time { hours, minutes })
    }
}

#[derive(Debug)]
enum TimeParseError {
    MissingColon,
    InvalidLength,
    InvalidNumber,
}

impl fmt::Display for TimeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeParseError::MissingColon => write!(f, "missing ':'"),
            TimeParseError::InvalidLength => write!(f, "invalid length"),
            TimeParseError::InvalidNumber => write!(f, "invalid number"),
        }
    }
}

fn main() {
    let a: Time = "12:20".parse().unwrap();
    let b: Time = "15:14".parse().unwrap();

    println!("{a}");
    println!("{b}");

    let err1: TimeParseError = "12.20".parse::<Time>().unwrap_err();
    let err2: TimeParseError = "12:2".parse::<Time>().unwrap_err();
    let err3: TimeParseError = "12:2a".parse::<Time>().unwrap_err();
    println!("error: {err1}");
    println!("error: {err2}");
    println!("error: {err3}");
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_display() {
        let time1 = Time {
            hours: 1,
            minutes: 1,
        };
        let time2 = Time {
            hours: 2,
            minutes: 1,
        };
        let time3 = Time {
            hours: 1,
            minutes: 20,
        };
        let time4 = Time {
            hours: 2,
            minutes: 20,
        };

        assert_eq!(format!("{}", time1), "1 hour, 1 minute");
        assert_eq!(format!("{}", time2), "2 hours, 1 minute");
        assert_eq!(format!("{}", time3), "1 hour, 20 minutes");
        assert_eq!(format!("{}", time4), "2 hours, 20 minutes");
    }

    #[test]
    fn test_valid_parse() {
        let time: Time = "12:34".parse().unwrap();
        assert_eq!(time.hours, 12);
        assert_eq!(time.minutes, 34);

        let time: Time = "00:00".parse().unwrap();
        assert_eq!(time.hours, 0);
        assert_eq!(time.minutes, 0);

        let time: Time = "23:59".parse().unwrap();
        assert_eq!(time.hours, 23);
        assert_eq!(time.minutes, 59);
    }

    #[test]
    fn test_invalid_parse_missing_colon() {
        let err: Result<Time, TimeParseError> = "1234".parse();
        assert!(matches!(err, Err(TimeParseError::MissingColon)));
    }

    #[test]
    fn test_invalid_parse_invalid_number() {
        let err: Result<Time, TimeParseError> = "12:ab".parse();
        assert!(matches!(err, Err(TimeParseError::InvalidNumber)));

        let err: Result<Time, TimeParseError> = "25:00".parse();
        assert!(matches!(err, Err(TimeParseError::InvalidNumber)));

        let err: Result<Time, TimeParseError> = "12:60".parse();
        assert!(matches!(err, Err(TimeParseError::InvalidNumber)));
    }

    #[test]
    fn test_invalid_parse_invalid_length() {
        let err: Result<Time, TimeParseError> = "1:00".parse();
        assert!(matches!(err, Err(TimeParseError::InvalidLength)));

        let err: Result<Time, TimeParseError> = "12:5".parse();
        assert!(matches!(err, Err(TimeParseError::InvalidLength)));
    }
}
