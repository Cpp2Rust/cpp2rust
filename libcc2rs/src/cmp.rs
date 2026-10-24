// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// The comparisons that the C++ standard library does with operator< and
// operator==, in the refcount model.

use std::cmp::Ordering;

use crate::rc::Ptr;
use crate::reinterpret::ByteRepr;

// operator<. Records implement it with their comparison operators.
pub trait CppLess: Sized {
    fn less(lhs: &Ptr<Self>, rhs: &Ptr<Self>) -> bool;
}

// operator==. Records implement it with their comparison operators.
pub trait CppEq<Rhs = Self>: Sized {
    fn equal(lhs: &Ptr<Self>, rhs: &Ptr<Rhs>) -> bool;
}

impl<T: PartialOrd + ByteRepr> CppLess for T {
    fn less(lhs: &Ptr<Self>, rhs: &Ptr<Self>) -> bool {
        lhs.with(|a| rhs.with(|b| a < b))
    }
}

impl<T: PartialEq<U> + ByteRepr, U: ByteRepr> CppEq<U> for T {
    fn equal(lhs: &Ptr<Self>, rhs: &Ptr<U>) -> bool {
        lhs.with(|a| rhs.with(|b| a == b))
    }
}

// The order of two values that std::sort and std::map derive from operator<.
pub fn cpp_cmp<T: CppLess>(lhs: &Ptr<T>, rhs: &Ptr<T>) -> Ordering {
    if T::less(lhs, rhs) {
        Ordering::Less
    } else if T::less(rhs, lhs) {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}
