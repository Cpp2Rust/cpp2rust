// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

use crate::{ArgRepr, AsPointer, ByteRepr, FnPtrArg, Ptr, Value, record_from_repr};

impl ByteRepr for std::cmp::Ordering {}

impl FnPtrArg for std::cmp::Ordering {
    #[inline]
    fn to_repr(self) -> ArgRepr {
        ArgRepr::Record(Box::new(self))
    }
    fn from_repr(r: ArgRepr) -> Self {
        record_from_repr(r)
    }
}

pub static STRONG_ORDERING_LESS: Ordering = Ordering::Less;
pub static STRONG_ORDERING_EQUAL: Ordering = Ordering::Equal;
pub static STRONG_ORDERING_EQUIVALENT: Ordering = Ordering::Equal;
pub static STRONG_ORDERING_GREATER: Ordering = Ordering::Greater;

thread_local! {
    static STRONG_ORDERING_LESS_VALUE: Value<Ordering> = Rc::new(RefCell::new(Ordering::Less));
    static STRONG_ORDERING_EQUAL_VALUE: Value<Ordering> = Rc::new(RefCell::new(Ordering::Equal));
    static STRONG_ORDERING_EQUIVALENT_VALUE: Value<Ordering> = Rc::new(RefCell::new(Ordering::Equal));
    static STRONG_ORDERING_GREATER_VALUE: Value<Ordering> = Rc::new(RefCell::new(Ordering::Greater));
}

pub fn strong_ordering_less() -> Ptr<Ordering> {
    STRONG_ORDERING_LESS_VALUE.with(AsPointer::as_pointer)
}

pub fn strong_ordering_equal() -> Ptr<Ordering> {
    STRONG_ORDERING_EQUAL_VALUE.with(AsPointer::as_pointer)
}

pub fn strong_ordering_equivalent() -> Ptr<Ordering> {
    STRONG_ORDERING_EQUIVALENT_VALUE.with(AsPointer::as_pointer)
}

pub fn strong_ordering_greater() -> Ptr<Ordering> {
    STRONG_ORDERING_GREATER_VALUE.with(AsPointer::as_pointer)
}
