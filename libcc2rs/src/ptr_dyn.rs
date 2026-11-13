// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::cell::{Ref, RefCell, RefMut};
use std::rc::{Rc, Weak};

use crate::rc::{Ptr, PtrKind};

pub struct StrongPtrDyn<T: ?Sized> {
    rc: Rc<RefCell<T>>,
}

impl<T: ?Sized> StrongPtrDyn<T> {
    pub fn deref(&self) -> Ref<'_, T> {
        self.rc.borrow()
    }

    pub fn deref_mut(&self) -> RefMut<'_, T> {
        self.rc.borrow_mut()
    }
}

// A pointer to a single object through one of its traits, e.g., `dyn Shape`
// for a pointer to a base class.
#[derive(Debug)]
pub struct PtrDyn<T: ?Sized>(Option<Weak<RefCell<T>>>);

impl<T: ?Sized> Default for PtrDyn<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T: ?Sized> Clone for PtrDyn<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: ?Sized> PtrDyn<T> {
    pub fn upgrade(&self) -> StrongPtrDyn<T> {
        let weak = self.0.as_ref().expect("ub: dereference of null pointer");
        StrongPtrDyn {
            rc: weak.upgrade().expect("ub: dangling pointer"),
        }
    }
}

impl<T> Ptr<T> {
    #[inline]
    pub fn to_dyn<U: ?Sized>(&self, coerce: fn(Weak<RefCell<T>>) -> Weak<RefCell<U>>) -> PtrDyn<U> {
        match &self.kind {
            PtrKind::Null => PtrDyn(None),
            PtrKind::StackSingle(weak) | PtrKind::HeapSingle(weak) => {
                assert_eq!(self.offset, 0, "ub: invalid offset");
                PtrDyn(Some(coerce(weak.clone())))
            }
            _ => panic!("ub: invalid upcast"),
        }
    }
}
