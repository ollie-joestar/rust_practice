#[allow(dead_code)]
fn deduplicate(list: &mut Vec<i32>) {
    if list.is_empty() {
        return ;
    }
    for i in 0..list.len() - 1 {
        for j in 1..list.len() {
            while i + j < list.len() && list[i] == list[i + j] {
                list.remove(i + j);
            }
        }
    }
}

#[cfg(test)]
mod testing {
    use crate::deduplicate;
    #[test]
    fn test_dedup_small() {
        let mut v = vec![1, 2, 2, 3, 2, 4, 3];
        deduplicate(&mut v);
        assert_eq!(v, [1, 2, 3, 4]);
    }
    #[test]
    fn test_dedup_big() {
        let mut list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4];
        deduplicate(&mut list);
        assert_eq!(list, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
    #[test]
    fn test_repeat() {
        let mut list = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        deduplicate(&mut list);
        assert_eq!(list, vec![1]);
    }
    #[test]
    fn test_deduplicated() {
        let mut list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        deduplicate(&mut list);
        assert_eq!(list, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
    #[test]
    fn test_palindrome() {
        let mut list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        deduplicate(&mut list);
        assert_eq!(list, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
    #[test]
    fn test_empty() {
        let mut list = vec![];
        deduplicate(&mut list);
        assert_eq!(list, vec![]);
    }
}
