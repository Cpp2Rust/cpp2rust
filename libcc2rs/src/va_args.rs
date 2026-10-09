// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::any::Any;
use std::ffi::c_void;
use std::rc::Rc;

use crate::void::AnyPtr;

#[derive(Clone)]
pub enum VaArg {
    Int(i32),
    UInt(u32),
    Long(i64),
    ULong(u64),
    Double(f64),
    RawPtr(*mut c_void),
    Ptr(AnyPtr),
    Record(Rc<dyn Any>),
}

macro_rules! impl_va_arg_from {
    (direct: $($ty:ty => $variant:ident),*) => {$(
        impl From<$ty> for VaArg {
            fn from(v: $ty) -> Self { VaArg::$variant(v) }
        }
    )*};
    (promote: $($ty:ty => $variant:ident as $cast:ty),*) => {$(
        impl From<$ty> for VaArg {
            fn from(v: $ty) -> Self { VaArg::$variant(v as $cast) }
        }
    )*};
}

impl_va_arg_from!(direct: i32 => Int, u32 => UInt, i64 => Long, u64 => ULong, f64 => Double, AnyPtr => Ptr);
impl_va_arg_from!(promote: i8 => Int as i32, i16 => Int as i32, u8 => UInt as u32, u16 => UInt as u32, f32 => Double as f64, usize => ULong as u64, isize => Long as i64);

impl<T> From<*mut T> for VaArg {
    fn from(v: *mut T) -> Self {
        VaArg::RawPtr(v as *mut c_void)
    }
}
impl<T> From<*const T> for VaArg {
    fn from(v: *const T) -> Self {
        VaArg::RawPtr(v as *mut c_void)
    }
}

impl<T: crate::reinterpret::ByteRepr + 'static> From<crate::rc::Ptr<T>> for VaArg {
    fn from(v: crate::rc::Ptr<T>) -> Self {
        VaArg::Ptr(v.to_any())
    }
}

#[derive(Clone, Default)]
pub struct VaList {
    args: Rc<[VaArg]>,
    pos: usize,
}

impl crate::reinterpret::ByteRepr for VaList {}

impl VaList {
    pub fn new(args: &[VaArg]) -> Self {
        VaList {
            args: Rc::from(args),
            pos: 0,
        }
    }

    pub fn arg<T: VaArgGet>(&mut self) -> T {
        let val = &self.args[self.pos];
        self.pos += 1;
        T::get(val)
    }
}

pub trait VaArgGet {
    fn get(v: &VaArg) -> Self;
}

macro_rules! impl_va_arg_get {
    (int: $($ty:ty),*) => {$(
        impl VaArgGet for $ty {
            fn get(v: &VaArg) -> Self {
                match v {
                    VaArg::Int(n) => *n as Self,
                    VaArg::UInt(n) => *n as Self,
                    VaArg::Long(n) => *n as Self,
                    VaArg::ULong(n) => *n as Self,
                    _ => panic!("VaArgGet: expected integer"),
                }
            }
        }
    )*};
    (float: $($ty:ty),*) => {$(
        impl VaArgGet for $ty {
            fn get(v: &VaArg) -> Self {
                match v {
                    VaArg::Double(n) => *n as Self,
                    VaArg::Int(n) => *n as Self,
                    VaArg::Long(n) => *n as Self,
                    _ => panic!("VaArgGet: expected float"),
                }
            }
        }
    )*};
}

pub fn va_record_get<T: Any + Clone>(v: &VaArg) -> T {
    match v {
        VaArg::Record(r) => r
            .downcast_ref::<T>()
            .expect("VaArgGet: record type mismatch")
            .clone(),
        _ => panic!("VaArgGet: expected record"),
    }
}

impl_va_arg_get!(int: i8, i16, i32, i64, u8, u16, u32, u64);
impl_va_arg_get!(float: f32, f64);

impl<T> VaArgGet for *mut T {
    fn get(v: &VaArg) -> Self {
        match v {
            VaArg::RawPtr(p) => *p as Self,
            _ => panic!("VaArgGet: expected pointer"),
        }
    }
}
impl<T> VaArgGet for *const T {
    fn get(v: &VaArg) -> Self {
        match v {
            VaArg::RawPtr(p) => *p as Self,
            _ => panic!("VaArgGet: expected pointer"),
        }
    }
}

impl<T: 'static + crate::ByteRepr> VaArgGet for crate::rc::Ptr<T> {
    fn get(v: &VaArg) -> Self {
        match v {
            VaArg::Ptr(any) => any.reinterpret_cast::<T>(),
            _ => panic!("VaArgGet: expected Ptr"),
        }
    }
}

impl VaArgGet for AnyPtr {
    fn get(v: &VaArg) -> Self {
        match v {
            VaArg::Ptr(any) => any.clone(),
            _ => panic!("VaArgGet: expected Ptr"),
        }
    }
}

impl<T: crate::fn_ptr::FnSig> From<crate::FnPtr<T>> for VaArg {
    fn from(v: crate::FnPtr<T>) -> Self {
        VaArg::Ptr(v.to_any())
    }
}

impl<T: crate::fn_ptr::FnSig> VaArgGet for crate::FnPtr<T> {
    fn get(v: &VaArg) -> Self {
        match v {
            VaArg::Ptr(any) => any.cast_fn::<T>().expect("VaArgGet: FnPtr type mismatch"),
            _ => panic!("VaArgGet: expected FnPtr"),
        }
    }
}
