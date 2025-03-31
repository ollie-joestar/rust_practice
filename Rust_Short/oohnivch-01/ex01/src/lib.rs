#[allow(dead_code)]

fn min<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
    if *a < *b {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod testing {
    #[test]
    fn test_min() {
        assert_eq!(&3, crate::min(&3, &5));
        assert_eq!(&3, crate::min(&5, &3));
        assert_eq!(&-3, crate::min(&-3, &5));
        assert_eq!(&-5, crate::min(&-3, &-5));
    }
}
