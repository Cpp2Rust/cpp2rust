use std::ffi::c_void;

use crate::rc::AnyPtr;

#[derive(Clone)]
pub enum VaArg {
    Int(i32),
    UInt(u32),
    Long(i64),
    ULong(u64),
    Double(f64),
    RawPtr(*mut c_void),
    Ptr(AnyPtr),
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
    (ptr: $($ty:ty),*) => {$(
        impl From<*mut $ty> for VaArg {
            fn from(v: *mut $ty) -> Self { VaArg::RawPtr(v as *mut c_void) }
        }
        impl From<*const $ty> for VaArg {
            fn from(v: *const $ty) -> Self { VaArg::RawPtr(v as *mut c_void) }
        }
    )*};
}

impl_va_arg_from!(direct: i32 => Int, u32 => UInt, i64 => Long, u64 => ULong, f64 => Double, AnyPtr => Ptr);
impl_va_arg_from!(promote: i8 => Int as i32, i16 => Int as i32, u8 => UInt as u32, u16 => UInt as u32, f32 => Double as f64);
impl_va_arg_from!(ptr: c_void, i8, u8, i16, u16, i32, u32, i64, u64, f32, f64, usize, isize);

impl<T: Clone + crate::reinterpret::ByteRepr + 'static> From<crate::rc::Ptr<T>> for VaArg {
    fn from(v: crate::rc::Ptr<T>) -> Self {
        VaArg::Ptr(v.to_any())
    }
}

#[derive(Clone, Copy, Default)]
pub struct VaList<'a> {
    args: &'a [VaArg],
    pos: usize,
}

impl<'a> VaList<'a> {
    pub fn empty() -> Self {
        VaList { args: &[], pos: 0 }
    }

    pub fn new(args: &'a [VaArg]) -> Self {
        VaList { args, pos: 0 }
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
    (ptr: $($ty:ty),*) => {$(
        impl VaArgGet for *mut $ty {
            fn get(v: &VaArg) -> Self {
                match v { VaArg::RawPtr(p) => *p as Self, _ => panic!("VaArgGet: expected pointer") }
            }
        }
        impl VaArgGet for *const $ty {
            fn get(v: &VaArg) -> Self {
                match v { VaArg::RawPtr(p) => *p as Self, _ => panic!("VaArgGet: expected pointer") }
            }
        }
    )*};
}

impl_va_arg_get!(int: i8, i16, i32, i64, u8, u16, u32, u64);
impl_va_arg_get!(float: f32, f64);
impl_va_arg_get!(ptr: c_void, i8, u8, i16, u16, i32, u32, i64, u64, f32, f64, usize, isize);

impl<T: 'static> VaArgGet for crate::rc::Ptr<T> {
    fn get(v: &VaArg) -> Self {
        match v {
            VaArg::Ptr(any) => any.cast::<T>().expect("VaArgGet: Ptr type mismatch"),
            _ => panic!("VaArgGet: expected Ptr"),
        }
    }
}
