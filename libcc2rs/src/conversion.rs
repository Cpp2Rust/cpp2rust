// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::rc::Ptr;
use crate::reinterpret::ByteRepr;

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

impl<T: Clone + ByteRepr> CppFrom<T> for T {
    fn cpp_from(src: Ptr<T>) -> T {
        src.read()
    }
}

impl<T: Clone> CppFromUnsafe<T> for T {
    unsafe fn cpp_from(src: *const T) -> T {
        unsafe { (*src).clone() }
    }
}

#[macro_export]
macro_rules! cpp_type {
    ($cpp:literal, $rust:ty) => {
        $rust
    };
}
