use std::fmt;

struct John;

impl fmt::Display for John {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match f.precision() {
            Some(0) => f.write_str("Don't try to silence me!"),
            _ => f.pad("Hey! I'm John."),
        }
    }
}

impl fmt::Debug for John {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(if f.alternate() {
            "John, the man himself. He's handsome AND formidable."
        } else {
            "John, the man himself."
        })
    }
}

impl fmt::Binary for John {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad("Bip Boop?")
    }
}

fn main() {
    let john = John;

    println!("1. {john}");
    println!("2. |{john:<30}|");
    println!("3. |{john:>30}|");
    println!("4. {john:.6}");
    println!("5. {john:.0}");
    println!("6. {john:?}");
    println!("7. {john:#?}");
    println!("8. {john:b}");
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_display() {
        let john = John;
        assert_eq!(format!("{}", john), "Hey! I'm John.");
        assert_eq!(format!("{:.0}", john), "Don't try to silence me!");
    }

    #[test]
    fn test_debug() {
        let john = John;
        assert_eq!(format!("{:?}", john), "John, the man himself.");
        assert_eq!(
            format!("{:#?}", john),
            "John, the man himself. He's handsome AND formidable."
        );
    }

    #[test]
    fn test_binary() {
        let john = John;
        assert_eq!(format!("{:b}", john), "Bip Boop?");
    }
}
