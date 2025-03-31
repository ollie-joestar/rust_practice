#![allow(dead_code)]
fn min<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b { b } else { a }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = min(2, 4);
        assert_eq!(result, 2);
    }
    #[test]
    fn test_i32() {
        assert_eq!(min(12i32, -14i32), -14);
    }
    #[test]
    fn test_f32() {
        assert_eq!(min(12f32, 14f32), 12f32);
    }
    #[test]
    fn test_str() {
        assert_eq!(min("abc", "def"), "abc");
    }
    #[test]
    fn test_string() {
        assert_eq!(min(String::from("abc"), String::from("def")), "abc");
    }
}
