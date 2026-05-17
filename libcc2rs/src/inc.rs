// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::io::Cursor;

pub trait PostfixInc {
    fn postfix_inc(&mut self) -> Self;
}

impl<T> PostfixInc for Cursor<*mut T> {
    #[inline]
    fn postfix_inc(&mut self) -> Self {
        let clone = self.clone();
        self.set_position(self.position() + 1);
        clone
    }
}

macro_rules! postfix_nowrap_inc_impl {
    ($($type:ty),*) => {
        $(impl PostfixInc for $type {
            #[inline]
            fn postfix_inc(&mut self) -> Self {
                let copy = *self;
                *self += 1 as $type;
                copy
            }
        })*
    };
}

macro_rules! postfix_wrap_inc_impl {
    ($($type:ty),*) => {
        $(impl PostfixInc for $type {
            #[inline]
            fn postfix_inc(&mut self) -> Self {
                let copy = *self;
                *self = self.wrapping_add(1 as $type);
                copy
            }
        })*
    };
}

pub trait PrefixInc {
    fn prefix_inc(&mut self) -> Self;
}

macro_rules! prefix_nowrap_inc_impl {
    ($($type:ty),*) => {
        $(impl PrefixInc for $type {
            #[inline]
            fn prefix_inc(&mut self) -> Self {
                *self += 1 as $type;
                *self
            }
        })*
    };
}

macro_rules! prefix_wrap_inc_impl {
    ($($type:ty),*) => {
        $(impl PrefixInc for $type {
            #[inline]
            fn prefix_inc(&mut self) -> Self {
                *self = self.wrapping_add(1 as $type);
                *self
            }
        })*
    };
}

prefix_wrap_inc_impl!(i8, u8, i16, u16, i32, u32, i64, u64, isize, usize);
prefix_nowrap_inc_impl!(f32, f64);

postfix_wrap_inc_impl!(i8, u8, i16, u16, i32, u32, i64, u64, isize, usize);
postfix_nowrap_inc_impl!(f32, f64);

pub trait UnsafePostfixInc {
    /// # Safety
    /// This function increments a pointer and returns the old value.
    /// The caller must ensure the pointer is valid and doesn't overflow.
    unsafe fn postfix_inc(&mut self) -> Self;
}

impl<T> UnsafePostfixInc for *const T {
    #[inline]
    unsafe fn postfix_inc(&mut self) -> Self {
        let copy = *self;
        unsafe { *self = self.offset(1) }
        copy
    }
}

impl<T> UnsafePostfixInc for *mut T {
    #[inline]
    unsafe fn postfix_inc(&mut self) -> Self {
        let copy = *self;
        unsafe { *self = self.offset(1) }
        copy
    }
}

pub trait UnsafePrefixInc {
    /// # Safety
    /// This function increments a pointer and returns the new value.
    /// The caller must ensure the pointer is valid and doesn't overflow.
    unsafe fn prefix_inc(&mut self) -> Self;
}

impl<T> UnsafePrefixInc for *const T {
    #[inline]
    unsafe fn prefix_inc(&mut self) -> Self {
        unsafe { *self = self.offset(1) }
        *self
    }
}

impl<T> UnsafePrefixInc for *mut T {
    #[inline]
    unsafe fn prefix_inc(&mut self) -> Self {
        unsafe { *self = self.offset(1) }
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_postfix_inc() {
        let mut x = 10;
        assert_eq!(x.postfix_inc(), 10);
        assert_eq!(x, 11);
    }

    #[test]
    fn test_prefix_inc() {
        let mut x = 10;
        assert_eq!(x.prefix_inc(), 11);
        assert_eq!(x, 11);
    }

    #[test]
    fn test_unsafe_postfix_inc() {
        let x = 10;
        let y = &x as *const i32;
        let mut z = &x as *const i32;
        unsafe {
            assert_eq!(z.postfix_inc(), y);
            assert_eq!(z, y.offset(1));
        }
    }

    #[test]
    fn test_unsafe_prefix_inc() {
        let x = 10;
        let y = &x as *const i32;
        let mut z = &x as *const i32;
        unsafe {
            assert_eq!(z.prefix_inc(), y.offset(1));
            assert_eq!(z, y.offset(1));
        }
    }
}
