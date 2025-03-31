use std::cell::Cell;

#[allow(dead_code)]
fn swap_u32(a: &Cell<u32>, b: &Cell<u32>) {
    a.swap(b)
}

#[allow(dead_code)]
fn swap_string(a: &Cell<String>, b: &Cell<String>) {
    a.swap(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_u32() {
        let a = Cell::new(2);
        let b = Cell::new(4);
        swap_u32(&a, &b);
        assert_eq!(a.get(), 4);
        assert_eq!(b.get(), 2);
    }
    #[test]
    fn test_swap_string() {
        let a = Cell::new("ABC".into());
        let b = Cell::new("DEF".into());

        swap_string(&a, &b);

        assert_eq!(a.into_inner(), "DEF");
        assert_eq!(b.into_inner(), "ABC");
    }
}
