#![forbid(unsafe_op_in_unsafe_fn)]
use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use std::clone::Clone;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::{drop_in_place, NonNull};

pub struct Carton<T> {
    ptr: NonNull<T>,         // Pointer to the heap-allocated T
    _marker: PhantomData<T>, // Used for variance and drop checking
}

impl<T> Carton<T> {
    /// Allocates a new Carton containing the given value.
    pub fn new(value: T) -> Self {
        let layout = Layout::new::<T>();

        // SAFETY:
        // - `alloc` returns a pointer to uninitialized memory of size `layout`.
        // - If allocation fails, `handle_alloc_error` will abort the process,
        //   so we are guaranteed to have valid memory after this.
        // - The returned pointer must not be null (checked by `handle_alloc_error`).
        let ptr = unsafe { alloc(layout) } as *mut T;
        if ptr.is_null() {
            handle_alloc_error(layout);
        }

        // SAFETY:
        // - `ptr` points to a valid, uninitialized memory block suitable for a `T`.
        // - We initialize the memory with `value`, making it valid for future reads.
        unsafe {
            ptr.write(value);
        }

        Self {
            ptr: unsafe { NonNull::new_unchecked(ptr) }, // SAFETY: `ptr` is not null.
            _marker: PhantomData,
        }
    }

    /// Consumes the Carton and `returns` the inner `value`.
    /// Similar to `ptr.read()` but with **self-destruction** after the ***extraction***
    pub fn into_inner(self) -> T {
        // SAFETY:
        // - We are consuming `self`, meaning no other references to the inner value exist.
        // - `self.ptr` points to a valid `T` that has not been dropped yet.
        unsafe { self.ptr.as_ptr().read() }
    }
}

impl<T> Drop for Carton<T> {
    fn drop(&mut self) {
        // SAFETY:
        // - `self.ptr` is valid and points to an initialized `T`.
        // - We drop the inner value to prevent memory leaks.
        unsafe {
            drop_in_place(self.ptr.as_ptr());
        }

        // SAFETY:
        // - After dropping the inner value, we deallocate the memory block.
        // - `self.ptr` points to a valid memory block allocated with the same layout.
        let layout = Layout::new::<T>();
        unsafe {
            dealloc(self.ptr.as_ptr() as *mut u8, layout);
        }
    }
}

impl<T> Deref for Carton<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY:
        // - `self.ptr` is valid and points to an initialized `T`.
        // - We are returning a shared reference, which ensures no mutation occurs.
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> DerefMut for Carton<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY:
        // - `self.ptr` is valid and points to an initialized `T`.
        // - We are returning a mutable reference, ensuring exclusive access.
        unsafe { self.ptr.as_mut() }
    }
}

impl<T: Clone> Clone for Carton<T> {
    fn clone(&self) -> Self {
        // Clone the inner value and allocate a new Carton for it.
        Carton::new((**self).clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Carton;

    #[derive(Clone, Debug, PartialEq)]
    struct Point {
        x: u32,
        y: u32,
    }

    #[test]
    fn test_carton() {
        let point_in_carton = Carton::new(Point { x: 1, y: 2 });

        assert_eq!(point_in_carton.x, 1);
        assert_eq!(point_in_carton.y, 2);

        let mut another_point = point_in_carton.clone();
        another_point.x = 2;
        another_point.y = 3;

        assert_eq!(another_point.x, 2);
        assert_eq!(another_point.y, 3);
        assert_eq!(point_in_carton.x, 1);
        assert_eq!(point_in_carton.y, 2);
    }

    #[test]
    fn test_new_and_deref() {
        let carton = Carton::new(42);
        assert_eq!(*carton, 42); // Deref test
    }

    #[test]
    fn test_into_inner() {
        let carton = Carton::new("Hello, world!");
        let value = carton.into_inner();
        assert_eq!(value, "Hello, world!"); // Ensure the inner value is correct
    }

    #[test]
    fn test_clone() {
        let original = Carton::new(3.14);
        let cloned = original.clone();
        assert_eq!(*original, *cloned); // Check values are equal
        assert_ne!(&original as *const _, &cloned as *const _); // Ensure they are different instances
    }

    #[test]
    fn test_deref_mut() {
        let mut carton = Carton::new(10);
        *carton.deref_mut() = 20; // Modify through deref_mut
        assert_eq!(*carton, 20); // Check if the modification was successful
    }
}
