// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

pub trait PrefixInc {
    fn prefix_inc(&mut self) -> &mut Self;
}

pub trait PostfixInc {
    fn postfix_inc(&mut self) -> Self;
}

pub trait PrefixDec {
    fn prefix_dec(&mut self) -> &mut Self;
}

pub trait PostfixDec {
    fn postfix_dec(&mut self) -> Self;
}

// The postfix operators return the old value; the prefix ones the new one.
macro_rules! inc_dec_impl {
    ($inc:expr, $dec:expr; $($type:ty),*) => {
        $(impl PrefixInc for $type {
            #[inline]
            fn prefix_inc(&mut self) -> &mut Self {
                *self = $inc(*self);
                self
            }
        }

        impl PostfixInc for $type {
            #[inline]
            fn postfix_inc(&mut self) -> Self {
                let copy = *self;
                self.prefix_inc();
                copy
            }
        }

        impl PrefixDec for $type {
            #[inline]
            fn prefix_dec(&mut self) -> &mut Self {
                *self = $dec(*self);
                self
            }
        }

        impl PostfixDec for $type {
            #[inline]
            fn postfix_dec(&mut self) -> Self {
                let copy = *self;
                self.prefix_dec();
                copy
            }
        })*
    };
}

inc_dec_impl!(
    |x: Self| x.wrapping_add(1), |x: Self| x.wrapping_sub(1);
    i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize
);
inc_dec_impl!(|x: Self| x + 1.0, |x: Self| x - 1.0; f32, f64);

pub trait UnsafePrefixInc {
    /// # Safety
    /// This function increments a pointer and returns the new value.
    /// The caller must ensure the pointer is valid and doesn't overflow.
    unsafe fn prefix_inc(&mut self) -> &mut Self;
}

pub trait UnsafePostfixInc {
    /// # Safety
    /// This function increments a pointer and returns the old value.
    /// The caller must ensure the pointer is valid and doesn't overflow.
    unsafe fn postfix_inc(&mut self) -> Self;
}

pub trait UnsafePrefixDec {
    /// # Safety
    /// This function decrements a pointer and returns the new value.
    /// The caller must ensure the pointer is valid and doesn't underflow.
    unsafe fn prefix_dec(&mut self) -> &mut Self;
}

pub trait UnsafePostfixDec {
    /// # Safety
    /// This function decrements a pointer and returns the old value.
    /// The caller must ensure the pointer is valid and doesn't underflow.
    unsafe fn postfix_dec(&mut self) -> Self;
}

macro_rules! unsafe_inc_dec_impl {
    ($($ptr:ty),*) => {
        $(impl<T> UnsafePrefixInc for $ptr {
            #[inline]
            unsafe fn prefix_inc(&mut self) -> &mut Self {
                unsafe { *self = self.offset(1) }
                self
            }
        }

        impl<T> UnsafePostfixInc for $ptr {
            #[inline]
            unsafe fn postfix_inc(&mut self) -> Self {
                let copy = *self;
                unsafe { self.prefix_inc() };
                copy
            }
        }

        impl<T> UnsafePrefixDec for $ptr {
            #[inline]
            unsafe fn prefix_dec(&mut self) -> &mut Self {
                unsafe { *self = self.offset(-1) }
                self
            }
        }

        impl<T> UnsafePostfixDec for $ptr {
            #[inline]
            unsafe fn postfix_dec(&mut self) -> Self {
                let copy = *self;
                unsafe { self.prefix_dec() };
                copy
            }
        })*
    };
}

unsafe_inc_dec_impl!(*const T, *mut T);

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
        assert_eq!(*x.prefix_inc(), 11);
        assert_eq!(x, 11);
    }

    #[test]
    fn test_postfix_dec() {
        let mut x = 10;
        assert_eq!(x.postfix_dec(), 10);
        assert_eq!(x, 9);
    }

    #[test]
    fn test_prefix_dec() {
        let mut x = 10;
        assert_eq!(*x.prefix_dec(), 9);
        assert_eq!(x, 9);
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
            assert_eq!(*z.prefix_inc(), y.offset(1));
            assert_eq!(z, y.offset(1));
        }
    }

    #[test]
    fn test_unsafe_postfix_dec() {
        let x = [10, 20];
        let y = &x[1] as *const i32;
        let mut z = &x[1] as *const i32;
        unsafe {
            assert_eq!(z.postfix_dec(), y);
            assert_eq!(z, y.offset(-1));
        }
    }

    #[test]
    fn test_unsafe_prefix_dec() {
        let x = [10, 20];
        let y = &x[1] as *const i32;
        let mut z = &x[1] as *const i32;
        unsafe {
            assert_eq!(*z.prefix_dec(), y.offset(-1));
            assert_eq!(z, y.offset(-1));
        }
    }
}
