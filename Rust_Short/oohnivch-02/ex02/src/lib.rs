#[derive(Debug, PartialEq)]
enum PizzaStatus {
    Ordered,
    Cooking,
    Cooked,
    Delivering,
    Delivered,
}

#[allow(dead_code)]
impl PizzaStatus {
    fn from_delivery_time(ordered_days_ago: u32) -> Self {
        match ordered_days_ago {
            0..2 => PizzaStatus::Ordered,
            2..7 => PizzaStatus::Cooking,
            7..10 => PizzaStatus::Cooked,
            10..17 => PizzaStatus::Delivering,
            17.. => PizzaStatus::Delivered,
        }
    }
    fn get_delivery_time_in_days(&self) -> u32 {
        match self {
            PizzaStatus::Ordered => 17,
            PizzaStatus::Cooking => 15,
            PizzaStatus::Cooked => 10,
            PizzaStatus::Delivering => 7,
            PizzaStatus::Delivered => 0,
        }
    }
}

#[cfg(test)]
mod testing {
    use super::PizzaStatus;
    #[test]
    fn test_from_delivery_ordered() {
        assert_eq!(PizzaStatus::from_delivery_time(0), PizzaStatus::Ordered);
        assert_eq!(PizzaStatus::from_delivery_time(1), PizzaStatus::Ordered);
    }
    #[test]
    fn test_from_delivery_cooking() {
        assert_eq!(PizzaStatus::from_delivery_time(2), PizzaStatus::Cooking);
        assert_eq!(PizzaStatus::from_delivery_time(6), PizzaStatus::Cooking);
    }
    #[test]
    fn test_from_delivery_cooked() {
        assert_eq!(PizzaStatus::from_delivery_time(7), PizzaStatus::Cooked);
        assert_eq!(PizzaStatus::from_delivery_time(9), PizzaStatus::Cooked);
    }
    #[test]
    fn test_from_delivery_delivering() {
        assert_eq!(PizzaStatus::from_delivery_time(10), PizzaStatus::Delivering);
        assert_eq!(PizzaStatus::from_delivery_time(16), PizzaStatus::Delivering);
    }
    #[test]
    fn test_from_delivery_delivered() {
        assert_eq!(PizzaStatus::from_delivery_time(18), PizzaStatus::Delivered);
        assert_eq!(PizzaStatus::from_delivery_time(42), PizzaStatus::Delivered);
        assert_eq!(PizzaStatus::from_delivery_time(69), PizzaStatus::Delivered);
    }
    #[test]
    fn test_get_delivery_time_ordered() {
        assert_eq!(PizzaStatus::Ordered.get_delivery_time_in_days(), 17);
    }
    #[test]
    fn test_get_delivery_time_cooking() {
        assert_eq!(PizzaStatus::Cooking.get_delivery_time_in_days(), 15);
    }
    #[test]
    fn test_get_delivery_time_cooked() {
        assert_eq!(PizzaStatus::Cooked.get_delivery_time_in_days(), 10);
    }
    #[test]
    fn test_get_delivery_time_delivering() {
        assert_eq!(PizzaStatus::Delivering.get_delivery_time_in_days(), 7);
    }
    #[test]
    fn test_get_delivery_time_delivered() {
        assert_eq!(PizzaStatus::Delivered.get_delivery_time_in_days(), 0);
    }
}
