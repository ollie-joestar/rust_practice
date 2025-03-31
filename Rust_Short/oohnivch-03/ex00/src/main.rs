fn choose<T>(values: &[T]) -> &T {
    if values.is_empty() {
        panic!("Empty arr");
    }
    &values[ftkit::random_number(0..values.len() as i32) as usize]
}

fn main() {
    let values = [1, 2, 3, 4, 5];
    let chosen_value = choose(&values);
    println!("Chosen value: {}", chosen_value);
}

#[cfg(test)]
mod tests {
    use crate::choose;

    #[test]
    #[should_panic(expected = "Empty arr")]
    fn test_choose_empty() {
        let values: [i32; 0] = [];
        choose(&values);
    }

    #[test]
    fn test_choose_single() {
        let values = [42];
        let result = choose(&values);
        assert_eq!(*result, 42);
    }

    #[test]
    fn test_choose_multiple() {
        let values = [1, 1, 1, 1, 1];
        let result = choose(&values);
        assert_eq!(*result, 1);
    }
}

