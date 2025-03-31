#![allow(dead_code)]

const fn color_name(color: & [u8; 3]) -> &'static str {
    match color {
        [0, 0, 0] => "pure black",
        [255, 255, 255] => "pure white",
        [255, 0, 0] => "pure red",
        [0, 255, 0] => "pure green",
        [0, 0, 255] => "pure blue",
        [128, 128, 128] => "perfect grey",
        [0..31, 0..31, 0..31] => "almost black",
        [129..=255, 0..=127, 0..=127] => "redish",
        [0..=127, 0..=127, 129..=255] => "blueish",
        [0..=127, 129..=255, 0..=127] => "greenish",
        _ => "unknown",
    }
}

#[cfg(test)]
mod testing {
    #[test]
    fn  test_color_name() {
        assert_eq!("pure black", crate::color_name(&[0, 0, 0]));
        assert_eq!("pure white", crate::color_name(&[255, 255, 255]));
        assert_eq!("pure red", crate::color_name(&[255, 0, 0]));
        assert_eq!("pure green", crate::color_name(&[0, 255, 0]));
        assert_eq!("pure blue", crate::color_name(&[0, 0, 255]));
        assert_eq!("perfect grey", crate::color_name(&[128, 128, 128]));
        assert_eq!("almost black", crate::color_name(&[5, 5, 5]));
        assert_eq!("redish", crate::color_name(&[150, 5, 5]));
        assert_eq!("greenish", crate::color_name(&[5, 150, 15]));
        assert_eq!("blueish", crate::color_name(&[5, 5, 150]));
        assert_eq!("unknown", crate::color_name(&[128, 5, 150]));
    }
    #[test]
    fn test_lifetimes() {
        let name_of_the_best_color;

        {
            let the_best_color = [42, 42, 42];
            name_of_the_best_color = crate::color_name(&the_best_color);
        }

        assert_eq!(name_of_the_best_color, "unknown");
    }
}
