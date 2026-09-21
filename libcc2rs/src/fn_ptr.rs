// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::any::{Any, TypeId};
use std::ops::Deref;
use std::rc::Rc;

use crate::rc::Ptr;
use crate::reinterpret::ByteRepr;
use crate::void::{AnyPtr, ErasedPtr};

pub trait FnAddr {
    fn fn_addr(&self) -> usize;
}

macro_rules! impl_fn_addr {
    () => {
        impl_fn_addr!(@gen A B C D E F G H I J K L M N O P);
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

pub struct FnPtr<T> {
    // Address of the function the pointer was created from. 0 for null, which
    // is never the address of a function.
    addr: usize,
    // The function as callable through T. None if the pointer was cast to T
    // without an adapter, when calling it is UB.
    current: Option<T>,
    // The function the pointer was created from, kept type-erased once the
    // pointer is cast to a different type so that it can be cast back. While
    // the pointer keeps its type this is None (and `current` is the original),
    // so creating and copying function pointers does not allocate.
    original: Option<Rc<dyn Any>>,
}

impl<T> FnPtr<T> {
    #[inline]
    pub fn null() -> Self {
        FnPtr {
            addr: 0,
            current: None,
            original: None,
        }
    }

    #[inline]
    pub fn is_null(&self) -> bool {
        self.addr == 0
    }
}

impl<T: FnAddr + 'static> FnPtr<T> {
    #[inline]
    pub fn new(f: T) -> Self {
        FnPtr {
            addr: f.fn_addr(),
            current: Some(f),
            original: None,
        }
    }
}

impl<T: FnAddr + Copy + 'static> FnPtr<T> {
    pub fn cast<U: FnAddr + Copy + 'static>(&self, adapter: Option<U>) -> FnPtr<U> {
        assert!(!self.is_null(), "ub: null fn pointer cast");

        // The current function, if it already has type U.
        let current_as_u = self.current.as_ref().and_then(|current| {
            let current: &dyn Any = current;
            current.downcast_ref::<U>().copied()
        });
        // The function this pointer was created from, if it has type U. While
        // `original` is unset, that is the current function.
        let original_as_u = match &self.original {
            Some(original) => original.downcast_ref::<U>().copied(),
            None => current_as_u,
        };

        let original = match &self.original {
            Some(original) => Some(original.clone()),
            // Casting to the same type keeps the pointer as is.
            None if TypeId::of::<T>() == TypeId::of::<U>() => None,
            None => self.current.map(|current| Rc::new(current) as Rc<dyn Any>),
        };

        FnPtr {
            addr: self.addr,
            current: current_as_u.or(original_as_u).or(adapter),
            original,
        }
    }
}

impl<T: 'static> Deref for FnPtr<T> {
    type Target = T;
    fn deref(&self) -> &T {
        if self.is_null() {
            panic!("ub: null fn pointer call");
        }
        self.current
            .as_ref()
            .expect("ub: calling through incompatible fn pointer type")
    }
}

impl<T: Copy> Clone for FnPtr<T> {
    fn clone(&self) -> Self {
        FnPtr {
            addr: self.addr,
            current: self.current,
            original: self.original.clone(),
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
        self.addr == other.addr
    }
}

impl<T> Eq for FnPtr<T> {}

impl<T: 'static> ByteRepr for FnPtr<T> {}

impl<T: Copy + 'static> ErasedPtr for FnPtr<T> {
    fn as_bytes(&self) -> Ptr<u8> {
        panic!("byte view not supported on fn pointer");
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn equals(&self, other: &dyn ErasedPtr) -> bool {
        other.as_any().downcast_ref::<FnPtr<T>>() == Some(self)
    }
    fn is_null(&self) -> bool {
        FnPtr::is_null(self)
    }
}

impl<T: Copy + 'static> FnPtr<T> {
    pub fn to_any(&self) -> AnyPtr {
        AnyPtr {
            ptr: Rc::new(self.clone()),
        }
    }
}

impl AnyPtr {
    pub fn cast_fn<T: Copy + 'static>(&self) -> Option<FnPtr<T>> {
        self.ptr.as_any().downcast_ref::<FnPtr<T>>().cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn twice(x: i32) -> i32 {
        x * 2
    }

    #[test]
    fn call_null_and_equality() {
        let f = FnPtr::<fn(i32) -> i32>::new(add_one);
        assert!(!f.is_null());
        assert_eq!((*f)(1), 2);
        assert!(f == f.clone());
        assert!(f != FnPtr::new(twice as fn(i32) -> i32));
        assert!(FnPtr::<fn(i32) -> i32>::null().is_null());
        assert!(FnPtr::<fn(i32) -> i32>::null() == FnPtr::default());
        assert!(f != FnPtr::null());
    }

    #[test]
    #[should_panic(expected = "ub: null fn pointer call")]
    fn call_null_panics() {
        let f = FnPtr::<fn(i32) -> i32>::null();
        (*f)(1);
    }

    #[test]
    #[should_panic(expected = "ub: null fn pointer cast")]
    fn cast_null_panics() {
        FnPtr::<fn(i32) -> i32>::null().cast::<fn(u32) -> u32>(None);
    }

    #[test]
    fn cast_with_adapter_and_back() {
        let f = FnPtr::<fn(i32) -> i32>::new(add_one);
        let g =
            f.cast::<fn(u32) -> u32>(Some((|x: u32| add_one(x as i32) as u32) as fn(u32) -> u32));
        assert_eq!((*g)(4), 5);
        // Equality is by the original function.
        assert!(g == f.cast::<fn(u32) -> u32>(None));
        // Casting back recovers the original function, without an adapter.
        let h = g.cast::<fn(i32) -> i32>(None);
        assert!(h == f);
        assert_eq!((*h)(4), 5);
        // Casting to the same type keeps the pointer.
        let same = f.cast::<fn(i32) -> i32>(None);
        assert_eq!((*same)(9), 10);
    }

    #[test]
    #[should_panic(expected = "ub: calling through incompatible fn pointer type")]
    fn cast_without_adapter_cannot_be_called() {
        let f = FnPtr::<fn(i32) -> i32>::new(add_one);
        let g = f.cast::<fn(u32) -> u32>(None);
        (*g)(1);
    }

    #[test]
    fn any_ptr_roundtrip() {
        let f = FnPtr::<fn(i32) -> i32>::new(twice);
        let any = f.to_any();
        assert!(!any.is_null());
        let back = any.cast_fn::<fn(i32) -> i32>().unwrap();
        assert_eq!((*back)(21), 42);
        assert!(back == f);
        assert!(any.cast_fn::<fn(u8)>().is_none());
    }
}
