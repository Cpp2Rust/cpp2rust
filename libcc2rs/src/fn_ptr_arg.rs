// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// Support for FnPtr::cast synthesizing a call-through adapter automatically,
// for any function pointer types T -> U.

use crate::rc::Ptr;
use crate::reinterpret::ByteRepr;
use crate::void::{AnyPtr, ErasedPtr};

// A type-erased view of one argument or return value, used only transiently
// (within a single adapted call) and never allocated.
pub(crate) enum ArgRepr<'a> {
    Bytes([u8; 16], usize),
    Ptr(&'a dyn ErasedPtr),
}

pub(crate) trait FnPtrArg: 'static {
    fn to_repr(&self) -> ArgRepr<'_>;
    // Panics if `r` describes a value that isn't a meaningful `Self` (wrong
    // kind, or same kind but incompatible size).
    fn from_repr(r: &ArgRepr) -> Self;
}

impl FnPtrArg for () {
    #[inline]
    fn to_repr(&self) -> ArgRepr<'_> {
        ArgRepr::Bytes([0; 16], 0)
    }
    #[inline]
    fn from_repr(r: &ArgRepr) -> Self {
        match r {
            ArgRepr::Bytes(_, 0) => (),
            _ => panic!("ub: calling through incompatible fn pointer type"),
        }
    }
}

macro_rules! impl_fn_ptr_arg_prim {
    ($ty:ty) => {
        impl FnPtrArg for $ty {
            #[inline]
            fn to_repr(&self) -> ArgRepr<'_> {
                let bytes = self.to_ne_bytes();
                let mut buf = [0u8; 16];
                buf[..bytes.len()].copy_from_slice(&bytes);
                ArgRepr::Bytes(buf, bytes.len())
            }
            #[inline]
            fn from_repr(r: &ArgRepr) -> Self {
                const N: usize = std::mem::size_of::<$ty>();
                match r {
                    ArgRepr::Bytes(buf, len) if *len == N => {
                        let mut a = [0u8; N];
                        a.copy_from_slice(&buf[..N]);
                        <$ty>::from_ne_bytes(a)
                    }
                    _ => panic!("ub: calling through incompatible fn pointer type"),
                }
            }
        }
    };
}

impl_fn_ptr_arg_prim!(u8);
impl_fn_ptr_arg_prim!(i8);
impl_fn_ptr_arg_prim!(u16);
impl_fn_ptr_arg_prim!(i16);
impl_fn_ptr_arg_prim!(u32);
impl_fn_ptr_arg_prim!(i32);
impl_fn_ptr_arg_prim!(u64);
impl_fn_ptr_arg_prim!(i64);
impl_fn_ptr_arg_prim!(u128);
impl_fn_ptr_arg_prim!(i128);
impl_fn_ptr_arg_prim!(usize);
impl_fn_ptr_arg_prim!(isize);
impl_fn_ptr_arg_prim!(f32);
impl_fn_ptr_arg_prim!(f64);

impl FnPtrArg for bool {
    #[inline]
    fn to_repr(&self) -> ArgRepr<'_> {
        ArgRepr::Bytes(
            [*self as u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            1,
        )
    }
    #[inline]
    fn from_repr(r: &ArgRepr) -> Self {
        match r {
            ArgRepr::Bytes(buf, 1) => buf[0] != 0,
            _ => panic!("ub: calling through incompatible fn pointer type"),
        }
    }
}

impl<T: ByteRepr> FnPtrArg for Ptr<T> {
    #[inline]
    fn to_repr(&self) -> ArgRepr<'_> {
        ArgRepr::Ptr(self)
    }
    fn from_repr(r: &ArgRepr) -> Self {
        match r {
            ArgRepr::Ptr(e) => match e.as_any().downcast_ref::<Ptr<T>>() {
                Some(exact) => exact.clone(),
                None => e.as_bytes().reinterpret_cast(),
            },
            ArgRepr::Bytes(..) => panic!("ub: calling through incompatible fn pointer type"),
        }
    }
}

impl FnPtrArg for AnyPtr {
    #[inline]
    fn to_repr(&self) -> ArgRepr<'_> {
        ArgRepr::Ptr(&*self.ptr)
    }
    fn from_repr(r: &ArgRepr) -> Self {
        match r {
            ArgRepr::Ptr(e) => e.as_bytes().to_any(),
            ArgRepr::Bytes(..) => panic!("ub: calling through incompatible fn pointer type"),
        }
    }
}

// Fixed-capacity, type-erased view of a whole argument list: `FnPtrArgs`
// (below) converts a tuple of arguments to and from this common
// intermediate, so that converting between two argument lists never needs to
// know the other side's arity at compile time.
pub(crate) struct ArgList<'a> {
    items: [Option<ArgRepr<'a>>; 16],
    len: usize,
}

impl<'a> ArgList<'a> {
    #[inline]
    fn new() -> Self {
        ArgList {
            items: [const { None }; 16],
            len: 0,
        }
    }

    #[inline]
    fn push(&mut self, r: ArgRepr<'a>) {
        self.items[self.len] = Some(r);
        self.len += 1;
    }

    #[inline]
    fn get(&self, i: usize) -> &ArgRepr<'a> {
        self.items[i]
            .as_ref()
            .expect("ub: calling through incompatible fn pointer type")
    }

    fn expect_len(&self, n: usize) {
        if self.len != n {
            panic!("ub: calling through incompatible fn pointer type");
        }
    }
}

pub(crate) trait FnPtrArgs: Sized {
    fn to_list(&self) -> ArgList<'_>;
    fn from_list(l: &ArgList) -> Self;
}

macro_rules! impl_fn_ptr_args {
    () => {
        impl_fn_ptr_args!(@gen A B C D E F G H I J K L M N O P);
    };
    (@gen $($a:ident)*) => {
        impl_fn_ptr_args!(@arity $($a)*);
        impl_fn_ptr_args!(@peel $($a)*);
    };
    (@peel) => {};
    (@peel $head:ident $($tail:ident)*) => {
        impl_fn_ptr_args!(@gen $($tail)*);
    };
    (@arity $($a:ident)*) => {
        impl<$($a: FnPtrArg,)*> FnPtrArgs for ($($a,)*) {
            #[inline]
            #[allow(unused_mut)]
            fn to_list(&self) -> ArgList<'_> {
                #[allow(non_snake_case)]
                let ($($a,)*) = self;
                let mut l = ArgList::new();
                $( l.push(FnPtrArg::to_repr($a)); )*
                l
            }
            #[allow(unused_assignments, unused_mut, unused_variables, clippy::unused_unit)]
            fn from_list(l: &ArgList) -> Self {
                l.expect_len(impl_fn_ptr_args!(@count $($a)*));
                let mut i = 0;
                ($(
                    { let v = <$a as FnPtrArg>::from_repr(l.get(i)); i += 1; v },
                )*)
            }
        }
    };
    (@count) => { 0 };
    (@count $head:ident $($tail:ident)*) => { 1 + impl_fn_ptr_args!(@count $($tail)*) };
}
impl_fn_ptr_args!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitive_round_trip_via_repr() {
        let a: u64 = 0xdead_beef;
        let repr = a.to_repr();
        let b: usize = FnPtrArg::from_repr(&repr);
        assert_eq!(b, 0xdead_beef);
    }

    #[test]
    #[should_panic(expected = "ub: calling through incompatible fn pointer type")]
    fn mismatched_size_panics() {
        let a: u32 = 1;
        let repr = a.to_repr();
        let _: u64 = FnPtrArg::from_repr(&repr);
    }

    #[test]
    #[should_panic(expected = "ub: calling through incompatible fn pointer type")]
    fn kind_mismatch_panics() {
        let a: u64 = 1;
        let repr = a.to_repr();
        let _: Ptr<u8> = FnPtrArg::from_repr(&repr);
    }

    #[test]
    fn pointer_reinterpret_via_repr() {
        let p: Ptr<u32> = Ptr::alloc(0x0102_0304u32);
        let repr = p.to_repr();
        let bytes: Ptr<u8> = FnPtrArg::from_repr(&repr);
        assert_eq!(
            bytes.read(),
            if cfg!(target_endian = "little") { 4 } else { 1 }
        );
    }

    #[test]
    fn pointer_via_any_repr() {
        let p: Ptr<i32> = Ptr::alloc(42);
        let repr = p.to_repr();
        let any: AnyPtr = FnPtrArg::from_repr(&repr);
        let repr2 = any.to_repr();
        let back: Ptr<i32> = FnPtrArg::from_repr(&repr2);
        assert_eq!(back.read(), 42);
    }

    #[test]
    #[should_panic(expected = "ub: calling through incompatible fn pointer type")]
    fn args_arity_mismatch_panics_not_compile_error() {
        let list = (1u64,).to_list();
        let _ = <(u64, u64)>::from_list(&list);
    }

    #[test]
    fn args_round_trip() {
        let args = (1u64, 2u32);
        let list = args.to_list();
        let back = <(u64, u32)>::from_list(&list);
        assert_eq!(back, args);
    }
}
