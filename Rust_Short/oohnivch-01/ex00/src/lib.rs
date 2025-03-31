#![allow(dead_code)]
fn add(a: &i32, b: i32) -> i32 {
    a + b
}

fn add_assign(a: &mut i32, b: i32) {
    *a += b
}

//fn main() {
//    let a:i32 = 5;
//    let b:i32 = 3;
//    let mut c: i32 = a;
//    println!("using add() on {a} and {b} = {}", add(&a, b));
//    print!("using add_assign on {c} and {b} = ");
//    add_assign(&mut c, b);
//    println!("{c}");
//}

#[cfg(test)]
mod testing {
    #[test]
    fn test_add() {
        assert_eq!(8, crate::add(&5, 3));
        assert_eq!(10, crate::add(&5, 5));
        assert_eq!(2147483647, crate::add(&2147483646, 1));
        assert_eq!(-2147483648, crate::add(&-2147483648, 0));
        assert_eq!(-1, crate::add(&-2147483648, 2147483647));
    }
    #[test]
    fn test_add_assign() {
        let mut a: i32 = 5;
        crate::add_assign(&mut a, 3);
        assert_eq!(8, a);
        crate::add_assign(&mut a, 3);
        assert_eq!(11, a);
        crate::add_assign(&mut a, 31);
        assert_eq!(42, a);
        crate::add_assign(&mut a, 27);
        assert_eq!(69, a);
    }
}
