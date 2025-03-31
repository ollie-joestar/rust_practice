#![allow(dead_code)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    fn zero() -> Self {
        let (x, y) = (0.0, 0.0);
        Self { x, y }
    }
    fn distance(&self, other: &Self) -> f32 {
        (((self.x - other.x) * (self.x - other.x)) + ((self.y - other.y) * (self.y - other.y))).sqrt()
    }
    fn translate(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }
}

#[cfg(test)]
mod testing {
    use super::Point;
    #[test]
    fn test_point_new() {
        let point = Point::new(3.0, 4.0);
        assert_eq!(point.x, 3.0);
        assert_eq!(point.y, 4.0);
    }
    #[test]
    fn test_point_new_zero() {
        let point = Point::new(0.0, 0.0);
        assert_eq!(point.x, 0.0);
        assert_eq!(point.y, 0.0);
    }
    #[test]
    fn test_point_zero() {
        let point = Point::zero();
        assert_eq!(point.x, 0.0);
        assert_eq!(point.y, 0.0);
    }
    #[test]
    fn test_point_distance() {
        let point1 = Point::new(1.0, 1.0);
        let point2 = Point::new(4.0, 5.0);
        let distance = point1.distance(&point2);
        assert!((distance - 5.0).abs() < f32::EPSILON);
    }
    #[test]
    fn test_point_distance_zero() {
        let point1 = Point::new(1.0, 1.0);
        let point2 = Point::new(1.0, 1.0);
        let distance = point1.distance(&point2);
        assert_eq!(distance, 0.0);
    }
    #[test]
    fn test_point_translate() {
        let mut point = Point::new(2.0, 3.0);
        point.translate(1.0, 1.0);
        assert_eq!(point.x, 3.0);
        assert_eq!(point.y, 4.0);
    }
    #[test]
    fn test_point_translate_negative() {
        let mut point = Point::new(5.0, 5.0);
        point.translate(-3.0, -2.0);
        assert_eq!(point.x, 2.0);
        assert_eq!(point.y, 3.0);
    }
}
