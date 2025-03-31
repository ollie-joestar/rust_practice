#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
struct MyType {
}

fn main() {
    let instance = MyType::default();

    let other_instance = instance.clone();

    println!("the default value of MyType is {instance:?}");
    println!("the clone of `instance` is {other_instance:#?}");
    assert_eq!(
        instance,
        other_instance,
        "the clone isn't the same :/"
    );
    assert!(
        instance == other_instance,
        "why would the clone be less or greater than the original?",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        let instance = MyType::default();
        assert_eq!(instance, MyType::default());
    }
    #[test]
    fn test_clone() {
        let instance = MyType::default();
        let cloned_instance = instance.clone();
        assert_eq!(instance, cloned_instance);
    }
    #[test]
    fn test_equal() {
        let instance = MyType::default();
        let other_instance = instance.clone();
        assert_eq!(instance, other_instance);
        assert!(instance == other_instance);
    }
}

