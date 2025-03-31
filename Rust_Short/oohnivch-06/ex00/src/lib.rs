#![forbid(unsafe_op_in_unsafe_fn)]

use std::ptr::{read, write};

/// Copies proper C-style string (null-terminated) at `src` into `dst`
///
/// # Safety
///
/// - `src` must be:
///     Valid readable *null-terminated* string.
///     Accessible and not be deallocated during operation.
///
/// - `dst` must be:
///     Valid writable memory with sufficient space (`dst` has at least `strlen(src) + 1` allocated bytes).
///
/// - `src` and `dst` must not overlap
///
/// - This function does not perform any bounds checking, and the
///   caller must ensure that both pointers are valid and point to
///   accessible memory.
#[allow(dead_code)]
pub unsafe fn ft_strcpy(dst: *mut u8, src: *const u8) {
    let mut src_ptr = src;
    let mut dst_ptr = dst;

    // SAFETY:
    // - `src` must point to a valid null-terminated string.
    // - `dst` must point to valid writable memory with enough space.
    // - We copy until we encounter the null terminator, ensuring we write it.
    // - The caller guarantees that `src` and `dst` do not overlap.
    unsafe {
        while read(src_ptr) != 0 {
            write(dst_ptr, read(src_ptr));
            src_ptr = src_ptr.add(1);
            dst_ptr = dst_ptr.add(1);
        }
        write(dst_ptr, b'\0');
    }
}

#[allow(dead_code)]
/// Calculates the length of a C-style string (null-terminated).
///
/// # Safety
///
/// The pointer `s` must:
/// - Be valid for reads of type `u8`.
/// - Point to a properly null-terminated string.
/// - Not be null (i.e., not point to address `0x0`).
///
/// If any of these conditions are violated, the function may invoke
/// **undefined behavior** due to unchecked pointer arithmetic and dereferencing.
///
/// # Examples
/// ```
/// use crate::ex00::*;
///
/// fn example() {
///     let s = b"Hello, World!\0";  // A byte string with a null terminator.
///     let len = unsafe { ft_strlen(s.as_ptr()) };
///     assert_eq!(len, 13);
/// }
/// ```
pub unsafe fn ft_strlen(s: *const u8) -> usize {
    let mut len = 0;
    let mut ptr = s;
    // SAFETY:
    // - 's' must be a valid, non-null, and aligned pointer to a null-terminated string.
    // - 's' must be readable up to the first null byte.
    // - 'ptr.add(1)' in that case is safe
    // - Dereferencing with `read(ptr)` is safe since `ptr` is valid and aligned
    //   for `u8` reads, and we never access beyond the null byte.
    unsafe {
        while read(ptr) != 0 {
            len += 1;
            ptr = ptr.add(1);
        }
    }

    len
}

#[allow(dead_code)]
pub fn ft_swap<T>(a: &mut T, b: &mut T) {
    // SAFETY: Both `a` and `b` must be valid readable mutable references
    // and shouldn't be same
    unsafe {
        let temp = read(a);
        write(a, read(b));
        write(b, temp);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap() {
        let mut a = String::from("Hello, World!");
        let mut b = String::from("Goodby, World!");
        ft_swap(&mut a, &mut b);
        assert_eq!(a, "Goodby, World!");
        assert_eq!(b, "Hello, World!");
    }
    #[test]
    fn test_str_len() {
        let s = b"Hello, World!\0";
        // SAFETY:
        //  /* ... */
        let len = unsafe { ft_strlen(s.as_ptr()) };
        assert_eq!(len, 13);
    }
    #[test]
    fn test_strcpy() {
        let s = b"Hello, World!\0";
        let mut dst = [0u8; 14];
        // SAFETY:
        //  /* ... */
        unsafe { ft_strcpy(dst.as_mut_ptr(), s.as_ptr()) };
        assert_eq!(&dst, b"Hello, World!\0");
    }
}
