use std::mem::transmute;
use std::slice::from_raw_parts;

type GoldNugget = u16;
type Gold = [GoldNugget];
type Iron = u32;
type Mercure = u64;

/// The Philosopher's Stone is a mythical object that can transmute metals into gold.
struct PhilosopherStone;

#[allow(dead_code)]
impl PhilosopherStone {
    /// Converts an `Iron` (u32) into two `GoldNuggets` (u16).
    ///
    /// This function uses `transmute` to reinterpret the bits of a `u32` as two `u16` values.
    /// This is safe because the size of `u32` (4 bytes) matches exactly with the size of `[u16; 2]` (4 bytes).
    fn transmute_iron(self, iron: Iron) -> [GoldNugget; 2] {
        // SAFETY: `Iron` (u32) is exactly 4 bytes, the same as `[u16; 2]`.
        // Reinterpreting `Iron` in this way is valid, as it preserves the size.
        unsafe { transmute::<Iron, [GoldNugget; 2]>(iron) }
    }
    /// Converts a `Mercure` (u64) into four `GoldNuggets` (u16).
    ///
    /// This function uses `transmute` to reinterpret the bits of a `u64` as four `u16` values.
    /// This is safe because the size of `u64` (8 bytes) matches exactly with the size of `[u16; 4]` (8 bytes).
    fn transmute_mercure(self, mercure: Mercure) -> [GoldNugget; 4] {
        // SAFETY: `Mercure` (u64) is exactly 8 bytes, the same as `[u16; 4]`.
        // This is a valid use of `transmute` since sizes align perfectly.
        unsafe { transmute::<Mercure, [GoldNugget; 4]>(mercure) }
    }

    /// Transmutes a reference to a metal into a slice of `GoldNugget`s.
    ///
    /// This function assumes that the `metal` reference points to a valid object whose size is divisible
    /// by the size of `GoldNugget`. The caller must ensure that the input type `M` implements the `Metal` trait,
    /// and that the underlying memory layout matches expectations.
    fn transmute_metal<M: Metal>(self, metal: &M) -> &Gold {
        // SAFETY:
        // 1. `metal` is cast to a raw pointer and then to a `*const GoldNugget`. This is valid
        //    because `M` implements the `Metal` trait, ensuring it's safe to reinterpret.
        // 2. The size of `metal` is divided by the size of `GoldNugget`, ensuring the slice length is correct.
        unsafe {
            from_raw_parts(
                metal as *const M as *const GoldNugget,
                size_of_val(metal) / size_of::<GoldNugget>(),
            )
        }
    }
}

/// A marker trait for types that can be transmuted into gold.
///
/// # Safety
/// Implementors of this trait must ensure that the memory layout of the type is suitable for
/// reinterpreting it as a slice of `GoldNugget`s. This means that:
/// - The size of the type must be divisible by the size of `GoldNugget` (u16).
/// - The data must remain valid and aligned when cast to a `GoldNugget` slice.
unsafe trait Metal {}

// # Safety
// Implementing `Metal` for `Iron` (u32) is safe because:
// - `Iron` has a size of 4 bytes, which is divisible by the size of `GoldNugget` (2 bytes).
// - Casting `Iron` to a `[u16; 2]` maintains proper alignment and validity.
unsafe impl Metal for Iron {}

// # Safety
// Implementing `Metal` for `Mercure` (u64) is safe because:
// - `Mercure` has a size of 8 bytes, which is divisible by the size of `GoldNugget` (2 bytes).
// - Casting `Mercure` to a `[u16; 4]` maintains proper alignment and validity.
unsafe impl Metal for Mercure {}

// # Safety
// Implementing `Metal` for `GoldNugget` (u16) is safe because:
// - `GoldNugget` has a size of 2 bytes, which is divisible by the size of `GoldNugget` (2 bytes).
// ***Duh...***
// - Casting `GoldNugget` to a `[u16; 1]` maintains proper alignment and validity.
unsafe impl Metal for GoldNugget {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iron() {
        let iron = 0x12345678;
        assert_eq!(PhilosopherStone.transmute_iron(iron), [0x5678, 0x1234]);
    }
    #[test]
    fn test_mercure() {
        let mercure = 0x0123456789ABCDEF;
        assert_eq!(
            PhilosopherStone.transmute_mercure(mercure),
            [0xCDEF, 0x89AB, 0x4567, 0x0123],
        );
    }
    #[test]
    fn test_metal_gold() {
        let nugget: GoldNugget = 0x1234;
        let gold = PhilosopherStone.transmute_metal(&nugget);
        assert_eq!(gold, &[0x1234]);
    }
    #[test]
    fn test_metal_iron() {
        let iron: Iron = 0x12345678;
        let gold = PhilosopherStone.transmute_metal(&iron);
        assert_eq!(gold, &[0x5678, 0x1234]);
    }
    #[test]
    fn test_metal_mercure() {
        let mercure: Mercure = 0x0123456789ABCDEF;
        assert_eq!(
            PhilosopherStone.transmute_metal(&mercure),
            &[0xCDEF, 0x89AB, 0x4567, 0x0123],
        );
    }
}
