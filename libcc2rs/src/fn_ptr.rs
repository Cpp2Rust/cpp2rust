// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::any::{Any, TypeId};
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;

use crate::rc::{AnyPtr, ErasedPtr};
use crate::reinterpret::ByteRepr;

pub trait FnAddr {
    fn fn_addr(&self) -> usize;
}

macro_rules! impl_fn_addr {
    () => {
        impl_fn_addr!(@gen A B C D E F G H I J);
    };
    (@gen $($a:ident)*) => {
        impl<R $(, $a)*> FnAddr for fn($($a,)*) -> R {
            #[inline]
            fn fn_addr(&self) -> usize { *self as *const () as usize }
        }
        impl_fn_addr!(@peel $($a)*);
    };
    (@peel) => {};
    (@peel $head:ident $($tail:ident)*) => {
        impl_fn_addr!(@gen $($tail)*);
    };
}
impl_fn_addr!();

#[derive(Clone)]
pub(crate) struct FnState {
    addr: usize,
    cast_history: Vec<Option<Rc<dyn Any>>>,
}

pub struct FnPtr<T> {
    state: Option<Rc<FnState>>,
    // FnPtr does not use T, hence wrap in PhantomData
    _marker: PhantomData<T>,
}

impl<T> FnPtr<T> {
    #[inline]
    pub fn null() -> Self {
        FnPtr {
            state: None,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn is_null(&self) -> bool {
        self.state.is_none()
    }
}

impl<T: FnAddr + 'static> FnPtr<T> {
    pub fn new(f: T) -> Self {
        FnPtr {
            state: Some(Rc::new(FnState {
                addr: f.fn_addr(),
                cast_history: vec![Some(Rc::new(f))],
            })),
            _marker: PhantomData,
        }
    }
}

impl<T: 'static> FnPtr<T> {
    pub fn cast<U: 'static>(&self, adapter: Option<U>) -> FnPtr<U> {
        let state = self.state.as_ref().expect("ub: null fn pointer cast");

        for (i, entry) in state.cast_history.iter().enumerate() {
            if let Some(ref rc) = entry {
                if (*rc).as_ref().type_id() == TypeId::of::<U>() {
                    return FnPtr {
                        state: Some(Rc::new(FnState {
                            addr: state.addr,
                            cast_history: state.cast_history[..=i].to_vec(),
                        })),
                        _marker: PhantomData,
                    };
                }
            }
        }

        let mut new_stack = state.cast_history.clone();
        new_stack.push(adapter.map(|a| Rc::new(a) as Rc<dyn Any>));

        FnPtr {
            state: Some(Rc::new(FnState {
                addr: state.addr,
                cast_history: new_stack,
            })),
            _marker: PhantomData,
        }
    }
}

impl<T: 'static> Deref for FnPtr<T> {
    type Target = T;
    fn deref(&self) -> &T {
        let state = self.state.as_ref().expect("ub: null fn pointer call");
        let entry = state
            .cast_history
            .last()
            .expect("empty fn pointer cast_history");
        match entry {
            Some(rc) => rc
                .downcast_ref::<T>()
                .expect("ub: fn pointer type mismatch"),
            None => panic!("ub: calling through incompatible fn pointer type"),
        }
    }
}

impl<T> Clone for FnPtr<T> {
    fn clone(&self) -> Self {
        FnPtr {
            state: self.state.clone(),
            _marker: PhantomData,
        }
    }
}

impl<T> Default for FnPtr<T> {
    fn default() -> Self {
        Self::null()
    }
}

impl<T> PartialEq for FnPtr<T> {
    fn eq(&self, other: &Self) -> bool {
        match (&self.state, &other.state) {
            (None, None) => true,
            (Some(a), Some(b)) => a.addr == b.addr,
            _ => false,
        }
    }
}

impl<T> Eq for FnPtr<T> {}

impl<T: 'static> ByteRepr for FnPtr<T> {}

impl<T: 'static> ErasedPtr for FnPtr<T> {
    fn pointee_type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }
    fn memcpy(&self, _src: &dyn ErasedPtr, _len: usize) {
        panic!("memcpy not supported on fn pointer");
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn equals(&self, other: &dyn ErasedPtr) -> Option<bool> {
        if self.pointee_type_id() != other.pointee_type_id() {
            return None;
        }
        other.as_any().downcast_ref::<FnPtr<T>>().map(|o| self == o)
    }
}

impl<T: 'static> FnPtr<T> {
    pub fn to_any(&self) -> AnyPtr {
        AnyPtr {
            ptr: Rc::new(self.clone()),
        }
    }
}

impl AnyPtr {
    pub fn cast_fn<T: 'static>(&self) -> Option<FnPtr<T>> {
        self.ptr.as_any().downcast_ref::<FnPtr<T>>().cloned()
    }
}
