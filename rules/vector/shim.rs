// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::{ArgRepr, ByteRepr, FnPtrArg, record_from_repr};

impl<T: ByteRepr> ByteRepr for Vec<T> {}

impl<T: 'static> FnPtrArg for Vec<T> {
    #[inline]
    fn to_repr(self) -> ArgRepr {
        ArgRepr::Record(Box::new(self))
    }
    fn from_repr(r: ArgRepr) -> Self {
        record_from_repr(r)
    }
}
