// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::cell::{Ref, RefCell, RefMut};
use std::rc::{Rc, Weak};

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

#[derive(Default, Debug)]
enum PtrKindDyn<T: ?Sized> {
    #[default]
    Null, // TODO: is this useful?
    StackSingle(Weak<RefCell<T>>),
}

impl<T: ?Sized> Clone for PtrKindDyn<T> {
    fn clone(&self) -> Self {
        match &self {
            PtrKindDyn::Null => PtrKindDyn::Null,
            PtrKindDyn::StackSingle(weak) => PtrKindDyn::StackSingle(weak.clone()),
        }
    }
}

#[derive(Debug, Default)]
pub struct PtrDyn<T: ?Sized> {
    offset: usize,
    kind: PtrKindDyn<T>,
}

impl<T: ?Sized> PtrDyn<T> {
    pub fn upgrade(&self) -> StrongPtrDyn<T> {
        match &self.kind {
            PtrKindDyn::Null => panic!("ub: dereference of null pointer"),
            PtrKindDyn::StackSingle(weak) => {
                assert_eq!(self.offset, 0, "ub: invalid offset");
                StrongPtrDyn {
                    rc: weak.upgrade().expect("ub: dangling pointer"),
                }
            }
        }
    }
}

impl<T: ?Sized> Clone for PtrDyn<T> {
    fn clone(&self) -> Self {
        Self {
            offset: self.offset,
            kind: self.kind.clone(),
        }
    }
}

pub trait AsPointerDyn<T: ?Sized> {
    fn as_pointer_dyn(&self) -> PtrDyn<T>;
}

impl<T: ?Sized> AsPointerDyn<T> for Rc<RefCell<T>> {
    #[inline]
    fn as_pointer_dyn(&self) -> PtrDyn<T> {
        PtrDyn {
            offset: 0,
            kind: PtrKindDyn::StackSingle(Rc::downgrade(self)),
        }
    }
}
