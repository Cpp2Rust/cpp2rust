// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

pub trait PostfixDec {
    fn postfix_dec(&mut self) -> Self;
}

macro_rules! postfix_nowrap_dec_impl {
    ($($type:ty),*) => {
        $(impl PostfixDec for $type {
            #[inline]
            fn postfix_dec(&mut self) -> Self {
                let copy = *self;
                *self -= 1 as $type;
                copy
            }
        })*
    };
}

macro_rules! postfix_wrap_dec_impl {
    ($($type:ty),*) => {
        $(impl PostfixDec for $type {
            #[inline]
            fn postfix_dec(&mut self) -> Self {
                let copy = *self;
                *self = self.wrapping_sub(1 as $type);
                copy
            }
        })*
    };
}

pub trait PrefixDec {
    fn prefix_dec(&mut self) -> Self;
}

macro_rules! prefix_nowrap_dec_impl {
    ($($type:ty),*) => {
        $(impl PrefixDec for $type {
            #[inline]
            fn prefix_dec(&mut self) -> Self {
                *self -= 1 as $type;
                *self
            }
        })*
    };
}

macro_rules! prefix_wrap_dec_impl {
    ($($type:ty),*) => {
        $(impl PrefixDec for $type {
            #[inline]
            fn prefix_dec(&mut self) -> Self {
                *self = self.wrapping_sub(1 as $type);
                *self
            }
        })*
    };
}

postfix_wrap_dec_impl!(i8, u8, i16, u16, i32, u32, i64, u64, isize, usize);
postfix_nowrap_dec_impl!(f32, f64);

prefix_wrap_dec_impl!(i8, u8, i16, u16, i32, u32, i64, u64, isize, usize);
prefix_nowrap_dec_impl!(f32, f64);

pub trait UnsafePostfixDec {
    /// # Safety
    /// This function decrements a pointer and returns the old value.
    /// The caller must ensure the pointer is valid and doesn't underflow.
    unsafe fn postfix_dec(&mut self) -> Self;
}

impl<T> UnsafePostfixDec for *const T {
    #[inline]
    unsafe fn postfix_dec(&mut self) -> Self {
        let copy = *self;
        *self = self.offset(-1);
        copy
    }
}

impl<T> UnsafePostfixDec for *mut T {
    #[inline]
    unsafe fn postfix_dec(&mut self) -> Self {
        let copy = *self;
        *self = self.offset(-1);
        copy
    }
}

pub trait UnsafePrefixDec {
    /// # Safety
    /// This function decrements a pointer and returns the new value.
    /// The caller must ensure the pointer is valid and doesn't underflow.
    unsafe fn prefix_dec(&mut self) -> Self;
}

impl<T> UnsafePrefixDec for *const T {
    #[inline]
    unsafe fn prefix_dec(&mut self) -> Self {
        *self = self.offset(-1);
        *self
    }
}

impl<T> UnsafePrefixDec for *mut T {
    #[inline]
    unsafe fn prefix_dec(&mut self) -> Self {
        *self = self.offset(-1);
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_postfix_dec() {
        let mut x = 10;
        assert_eq!(x.postfix_dec(), 10);
        assert_eq!(x, 9);
    }

    #[test]
    fn test_prefix_dec() {
        let mut x = 10;
        assert_eq!(x.prefix_dec(), 9);
        assert_eq!(x, 9);
    }

    #[test]
    fn test_unsafe_postfix_dec() {
        let x = 10;
        let y = &x as *const i32;
        let mut z = &x as *const i32;
        unsafe {
            assert_eq!(z.postfix_dec(), y);
            assert_eq!(z, y.offset(-1));
        }
    }

    #[test]
    fn test_unsafe_prefix_dec() {
        let x = 10;
        let y = &x as *const i32;
        let mut z = &x as *const i32;
        unsafe {
            assert_eq!(z.prefix_dec(), y.offset(-1));
            assert_eq!(z, y.offset(-1));
        }
    }
}
