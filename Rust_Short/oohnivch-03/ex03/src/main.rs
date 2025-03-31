use std::fmt::Debug;

trait FortyTwo {
    fn forty_two() -> Self;
}

impl FortyTwo for i32 {
    fn forty_two() -> Self {
        42
    }
}

impl FortyTwo for u32 {
    fn forty_two() -> Self {
        42
    }
}

impl FortyTwo for String {
    fn forty_two() -> Self {
        42.to_string()
    }
}

fn print_forty_two<T: Debug + FortyTwo>() {
    let value = T::forty_two();
    println!("{:?}", value);
}

fn main() {
    print_forty_two::<i32>();
    print_forty_two::<u32>();
    print_forty_two::<String>();
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_forty_two_i32() {
        let value = i32::forty_two();
        assert_eq!(value, 42);
    }

    #[test]
    fn test_forty_two_u32() {
        let value = u32::forty_two();
        assert_eq!(value, 42);
    }

    #[test]
    fn test_forty_two_string() {
        let value = String::forty_two();
        assert_eq!(value, "42");
    }
}
