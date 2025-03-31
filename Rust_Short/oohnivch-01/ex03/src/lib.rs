#[allow(dead_code)]
fn all_used(hay: & [u32], needle: &[u32]) -> bool {
    let mut check = true;
    for n in needle {
        check = hay.contains(n);
        if !check {
            break;
        }
    }
    check
}

#[allow(dead_code)]
fn largest_group<'a>(haystack: &'a [u32], needle: &[u32]) -> &'a [u32] {
    if needle.is_empty() || haystack.is_empty() {
        return &[];
    }
    let mut result:&'a [u32] = &[];
    let mut nice = true;
    for start in 0..haystack.len() {
        if needle.contains(&haystack[start]) {
            for end in start..haystack.len() {
                let temp = &haystack[start..=end];
                for n in temp {
                    nice = needle.contains(n);
                    if !nice {
                        break;
                    }
                }
                if temp.len() > result.len() && nice && all_used(temp, needle) {
                    result = temp;
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod testing {
    #[test]
    fn test_largest_group() {
        assert_eq!(crate::largest_group(&[1, 3, 4, 3, 5, 5, 4], &[5]), &[5, 5]);
        assert_eq!(crate::largest_group(&[1, 3, 4, 3, 5, 5, 4], &[5, 3]), &[3, 5, 5]);
        assert_eq!(crate::largest_group(&[1, 3, 4, 3, 5, 5, 4], &[]), &[]);
        assert_eq!(crate::largest_group(&[1, 3, 4, 3, 5, 5, 4], &[4, 1]), &[]);
        assert_eq!(crate::largest_group(&[1, 3, 4, 5], &[4, 1, 4, 3, 1, 4]), &[1, 3, 4]);
    }
    #[test]
    fn rustinette_tests() {
        assert_eq!(crate::largest_group(&[1, 5, 5, 4, 3, 5, 5, 5, 4], &[5]), &[5, 5, 5]);
        assert_eq!(crate::largest_group(&[5, 5, 5], &[5, 5, 5]), &[5, 5, 5]);
        assert_eq!(crate::largest_group(&[], &[]), &[]);
        assert_eq!(crate::largest_group(&[5], &[5]), &[5]);
        assert_eq!(crate::largest_group(&[1, 3, 4, 3, 5, 4], &[5, 5]), &[5]);
    }
    #[test]
    fn test_lifetimes() {
        let haystack = [1, 2, 3, 2, 1];
        let result;

        {
            let needle = [2, 3];
            result = crate::largest_group(&haystack, &needle);
        }

        assert_eq!(result, &[2, 3, 2]);
    }
}
