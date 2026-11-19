// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::{ArgRepr, ByteRepr, FnPtrArg, record_from_repr};

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
