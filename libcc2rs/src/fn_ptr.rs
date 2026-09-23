// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#![allow(private_bounds)]

use std::any::{Any, TypeId};
use std::rc::Rc;

use crate::fn_ptr_arg::{ArgList, ArgRepr, FnPtrArg, FnPtrArgs};
use crate::rc::Ptr;
use crate::reinterpret::ByteRepr;
use crate::void::{AnyPtr, ErasedPtr};

pub(crate) trait FnSig: Copy + 'static {
    type Args: FnPtrArgs;
    type Ret: FnPtrArg;
    fn fn_addr(&self) -> usize;
    fn call_direct(self, args: Self::Args) -> Self::Ret;
}

macro_rules! impl_fn_sig {
    () => {
        impl_fn_sig!(@gen A B C D E F G H I J K L M N O P);
    };
    (@gen $($a:ident)*) => {
        impl<R: FnPtrArg $(, $a: FnPtrArg)*> FnSig for fn($($a,)*) -> R {
            type Args = ($($a,)*);
            type Ret = R;
            #[inline]
            fn fn_addr(&self) -> usize { *self as *const () as usize }

            #[inline]
            #[allow(non_snake_case)]
            fn call_direct(self, args: Self::Args) -> R {
                let ($($a,)*) = args;
                self($($a,)*)
            }
        }
        impl_fn_sig!(@peel $($a)*);
    };
    (@peel) => {};
    (@peel $head:ident $($tail:ident)*) => {
        impl_fn_sig!(@gen $($tail)*);
    };
}
impl_fn_sig!();

// Build a trampoline adaptor for a function pointer of one type to be called
// through a different type.
trait Adapted: Any {
    fn call_adapted(&self, args: ArgList<'_>, sink: &mut dyn FnMut(ArgRepr<'_>));
}

impl<T: FnSig> Adapted for T {
    fn call_adapted(&self, args: ArgList<'_>, sink: &mut dyn FnMut(ArgRepr<'_>)) {
        let converted_args = T::Args::from_list(&args);
        let result = self.call_direct(converted_args);
        sink(result.to_repr());
    }
}

pub struct FnPtr<T: FnSig> {
    // Address of the function this pointer was created from. 0 for null,
    // which is never the address of a function.
    addr: usize,
    // The function as directly callable through T: the fast path, taken
    // whenever this pointer's type hasn't changed (or was cast back to a
    // type it already had), with no conversion overhead at all.
    current: Option<T>,
    // The function this pointer was created from, kept type-erased (but
    // still callable, through `Adapted`).
    original: Option<Rc<dyn Adapted>>,
}

impl<T: FnSig> FnPtr<T> {
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

    #[inline]
    pub fn new(f: T) -> Self {
        FnPtr {
            addr: f.fn_addr(),
            current: Some(f),
            original: None,
        }
    }

    // Calls this pointer's function with `args`, dispatching to whichever of
    // `current`/`original` is set. Used by the per-arity `call` methods
    // below, which are what generated code actually calls.
    fn call_args(&self, args: T::Args) -> T::Ret {
        if self.is_null() {
            panic!("ub: null fn pointer call");
        }
        if let Some(f) = self.current {
            return f.call_direct(args);
        }
        if let Some(original) = &self.original {
            let mut result = None;
            original.call_adapted(args.to_list(), &mut |repr| {
                result = Some(T::Ret::from_repr(&repr));
            });
            return result.expect("ub: calling through incompatible fn pointer type");
        }
        panic!("ub: calling through incompatible fn pointer type");
    }

    // Builds a `FnPtr<U>` for the same function, automatically synthesizing
    // a call-through adaptor when U isn't this pointer's current or original
    // type. Casting the result back to a type it already had recovers that
    // exact function, calling it directly rather than through an adaptor.
    pub fn cast<U: FnSig>(&self) -> FnPtr<U> {
        assert!(!self.is_null(), "ub: null fn pointer cast");

        // The current function, if it already has type U.
        let current_as_u = self.current.as_ref().and_then(|current| {
            let current: &dyn Any = current;
            current.downcast_ref::<U>().copied()
        });
        // The function this pointer was created from, if it has type U.
        // While `original` is unset, that is the current function.
        let original_as_u = match &self.original {
            Some(original) => {
                let original: &dyn Any = &**original;
                original.downcast_ref::<U>().copied()
            }
            None => current_as_u,
        };

        // Casting to the same type keeps the pointer as is.
        let original = if TypeId::of::<T>() == TypeId::of::<U>() {
            self.original.clone()
        } else {
            self.boxed_original()
        };

        FnPtr {
            addr: self.addr,
            current: current_as_u.or(original_as_u),
            original,
        }
    }

    fn boxed_original(&self) -> Option<Rc<dyn Adapted>> {
        match &self.original {
            Some(original) => Some(original.clone()),
            None => self
                .current
                .map(|current| Rc::new(current) as Rc<dyn Adapted>),
        }
    }
}

// Per-arity, positional `call` methods -- what generated code actually
// invokes, e.g. `fn_ptr.call(a0, a1)`.
macro_rules! impl_fn_ptr_call {
    () => {
        impl_fn_ptr_call!(@gen A B C D E F G H I J K L M N O P);
    };
    (@gen $($a:ident)*) => {
        impl<R: FnPtrArg $(, $a: FnPtrArg)*> FnPtr<fn($($a,)*) -> R> {
            #[inline]
            #[allow(non_snake_case, clippy::too_many_arguments)]
            pub fn call(&self $(, $a: $a)*) -> R {
                self.call_args(($($a,)*))
            }
        }
        impl_fn_ptr_call!(@peel $($a)*);
    };
    (@peel) => {};
    (@peel $head:ident $($tail:ident)*) => {
        impl_fn_ptr_call!(@gen $($tail)*);
    };
}
impl_fn_ptr_call!();

impl<T: FnSig> Clone for FnPtr<T> {
    fn clone(&self) -> Self {
        FnPtr {
            addr: self.addr,
            current: self.current,
            original: self.original.clone(),
        }
    }
}

impl<T: FnSig> Default for FnPtr<T> {
    fn default() -> Self {
        Self::null()
    }
}

impl<T: FnSig> PartialEq for FnPtr<T> {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr
    }
}

impl<T: FnSig> Eq for FnPtr<T> {}

impl<T: FnSig> ByteRepr for FnPtr<T> {}

impl<T: FnSig> ErasedPtr for FnPtr<T> {
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

impl<T: FnSig> FnPtr<T> {
    pub fn to_any(&self) -> AnyPtr {
        AnyPtr {
            ptr: Rc::new(self.clone()),
        }
    }
}

impl AnyPtr {
    pub fn cast_fn<T: FnSig>(&self) -> Option<FnPtr<T>> {
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
        assert_eq!(f.call(1), 2);
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
        f.call(1);
    }

    #[test]
    #[should_panic(expected = "ub: null fn pointer cast")]
    fn cast_null_panics() {
        FnPtr::<fn(i32) -> i32>::null().cast::<fn(u32) -> u32>();
    }

    #[test]
    fn cast_to_same_type_keeps_pointer() {
        let f = FnPtr::<fn(i32) -> i32>::new(add_one);
        let same = f.cast::<fn(i32) -> i32>();
        assert_eq!(same.call(9), 10);
        assert!(same == f);
    }

    #[test]
    fn cast_same_size_primitive_and_back() {
        fn h(b: usize) -> u64 {
            b as u64
        }
        let f = FnPtr::<fn(usize) -> u64>::new(h);
        let g = f.cast::<fn(u64) -> u64>();
        assert_eq!(g.call(3), 3);
        // Equality is by the original function.
        assert!(g == f.cast::<fn(u64) -> u64>());
        let back = g.cast::<fn(usize) -> u64>();
        assert!(back == f);
        assert_eq!(back.call(3), 3);
    }

    #[test]
    fn cast_pointer_argument_via_reinterpret_cast() {
        fn add_offset(base: Ptr<i32>, offset: i32) -> i32 {
            base.read() + offset
        }
        let val: Ptr<i32> = Ptr::alloc(100);
        let f = FnPtr::<fn(Ptr<i32>, i32) -> i32>::new(add_offset);
        let g = f.cast::<fn(AnyPtr, i32) -> i32>();
        assert_eq!(g.call(val.to_any(), 42), 142);
    }

    #[test]
    #[should_panic(expected = "ub: calling through incompatible fn pointer type")]
    fn cast_incompatible_arity_compiles_but_panics_if_called() {
        let f = FnPtr::<fn(i32, i32) -> i32>::new(|a: i32, b: i32| a + b);
        let g = f.cast::<fn() -> i32>();
        g.call();
    }

    #[test]
    fn cast_chain_through_two_non_matching_types() {
        fn h(x: i32) -> i32 {
            x + 1
        }
        let a = FnPtr::<fn(i32) -> i32>::new(h);
        let b = a.cast::<fn()>();
        let c = b.cast::<fn(u32) -> u32>();
        assert_eq!(c.call(4), 5);
    }

    #[test]
    fn any_ptr_roundtrip() {
        let f = FnPtr::<fn(i32) -> i32>::new(twice);
        let any = f.to_any();
        assert!(!any.is_null());
        let back = any.cast_fn::<fn(i32) -> i32>().unwrap();
        assert_eq!(back.call(21), 42);
        assert!(back == f);
        assert!(any.cast_fn::<fn(u8)>().is_none());
    }

    #[test]
    #[should_panic(expected = "ub: calling through incompatible fn pointer type")]
    fn size_mismatch_does_not_silently_pass_smaller_arg() {
        fn f(x: u32) -> u32 {
            x
        }
        let g = FnPtr::<fn(u32) -> u32>::new(f).cast::<fn(u64) -> u64>();
        g.call(1);
    }
}
