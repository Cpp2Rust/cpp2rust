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
    original: Rc<dyn Any>,
    current_cast: Option<Rc<dyn Any>>,
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
        let addr = f.fn_addr();
        let rc: Rc<dyn Any> = Rc::new(f);
        FnPtr {
            state: Some(Rc::new(FnState {
                addr,
                original: rc.clone(),
                current_cast: Some(rc),
            })),
            _marker: PhantomData,
        }
    }
}

impl<T: 'static> FnPtr<T> {
    pub fn cast<U: 'static>(&self, adapter: Option<U>) -> FnPtr<U> {
        let state = self.state.as_ref().expect("ub: null fn pointer cast");

        let current_cast = if state
            .current_cast
            .as_ref()
            .is_some_and(|rc| (**rc).type_id() == TypeId::of::<U>())
        {
            state.current_cast.clone()
        } else if (*state.original).type_id() == TypeId::of::<U>() {
            Some(state.original.clone())
        } else {
            adapter.map(|a| Rc::new(a) as Rc<dyn Any>)
        };

        FnPtr {
            state: Some(Rc::new(FnState {
                addr: state.addr,
                original: state.original.clone(),
                current_cast,
            })),
            _marker: PhantomData,
        }
    }
}

impl<T: 'static> Deref for FnPtr<T> {
    type Target = T;
    fn deref(&self) -> &T {
        let state = self.state.as_ref().expect("ub: null fn pointer call");
        match &state.current_cast {
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
    fn is_null(&self) -> bool {
        FnPtr::is_null(self)
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
