// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::any::TypeId;

use crate::Record;
use crate::rc::Ptr;
use crate::reinterpret::ByteRepr;

const NULL: isize = -1;

fn member_of<C: Record, T: 'static>(object: &C, offset: usize) -> &T {
    object
        .locate(offset, TypeId::of::<T>())
        .and_then(|member| member.downcast_ref::<T>())
        .unwrap_or_else(|| panic!("ub: invalid member pointer"))
}

fn member_of_mut<C: Record, T: 'static>(object: &mut C, offset: usize) -> &mut T {
    object
        .locate_mut(offset, TypeId::of::<T>())
        .and_then(|member| member.downcast_mut::<T>())
        .unwrap_or_else(|| panic!("ub: invalid member pointer"))
}

impl<C: Record> Ptr<C> {
    #[inline]
    pub fn member<T: ByteRepr>(&self, member: isize) -> Ptr<T> {
        if member == NULL {
            panic!("ub: null member pointer");
        }
        let offset = member as usize;
        self.field_ptr_at(
            move |object| member_of(object, offset),
            move |object| member_of_mut(object, offset),
            offset,
        )
    }
}
