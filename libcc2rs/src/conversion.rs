// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::rc::Ptr;

pub trait CppFrom<S>: Sized {
    fn cpp_from(src: Ptr<S>) -> Self;

    fn cpp_from_move(src: Ptr<S>) -> Self {
        Self::cpp_from(src)
    }
}

pub trait CppFromUnsafe<S>: Sized {
    /// # Safety
    /// `src` must point to a live value.
    unsafe fn cpp_from(src: *const S) -> Self;

    /// # Safety
    /// `src` must point to a live value.
    unsafe fn cpp_from_move(src: *mut S) -> Self {
        unsafe { Self::cpp_from(src) }
    }
}

macro_rules! impl_arithmetic_from {
    ($from:ty => $($to:ty),*) => {
        $(
            impl CppFrom<$from> for $to {
                fn cpp_from(src: Ptr<$from>) -> $to {
                    src.read() as $to
                }
            }

            impl CppFromUnsafe<$from> for $to {
                unsafe fn cpp_from(src: *const $from) -> $to {
                    unsafe { *src as $to }
                }
            }
        )*
    };
}

macro_rules! impl_arithmetic_conversions {
    ($($ty:ty),*) => {
        $(
            impl_arithmetic_from!($ty => i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64);

            impl CppFrom<$ty> for bool {
                fn cpp_from(src: Ptr<$ty>) -> bool {
                    src.read() != 0 as $ty
                }
            }

            impl CppFromUnsafe<$ty> for bool {
                unsafe fn cpp_from(src: *const $ty) -> bool {
                    unsafe { *src != 0 as $ty }
                }
            }

            impl CppFrom<bool> for $ty {
                fn cpp_from(src: Ptr<bool>) -> $ty {
                    src.read() as u8 as $ty
                }
            }

            impl CppFromUnsafe<bool> for $ty {
                unsafe fn cpp_from(src: *const bool) -> $ty {
                    unsafe { *src as u8 as $ty }
                }
            }
        )*
    };
}

impl_arithmetic_conversions!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64);

impl CppFrom<bool> for bool {
    fn cpp_from(src: Ptr<bool>) -> bool {
        src.read()
    }
}

impl CppFromUnsafe<bool> for bool {
    unsafe fn cpp_from(src: *const bool) -> bool {
        unsafe { *src }
    }
}
