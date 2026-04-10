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

macro_rules! impl_from {
    ($ty:ty, $variant:ident) => {
        impl From<$ty> for VaArg {
            fn from(v: $ty) -> Self {
                VaArg::$variant(v)
            }
        }
    };
    ($ty:ty => $variant:ident as $cast:ty) => {
        impl From<$ty> for VaArg {
            fn from(v: $ty) -> Self {
                VaArg::$variant(v as $cast)
            }
        }
    };
}

impl_from!(i32, Int);
impl_from!(u32, UInt);
impl_from!(i64, Long);
impl_from!(u64, ULong);
impl_from!(f64, Double);

// C promotion: char/short -> int, float -> double
impl_from!(i8 => Int as i32);
impl_from!(i16 => Int as i32);
impl_from!(u8 => UInt as u32);
impl_from!(u16 => UInt as u32);
impl_from!(f32 => Double as f64);
impl_from!(AnyPtr, Ptr);

impl<T: Clone + crate::reinterpret::ByteRepr + 'static> From<crate::rc::Ptr<T>> for VaArg {
    fn from(v: crate::rc::Ptr<T>) -> Self {
        VaArg::Ptr(v.to_any())
    }
}

macro_rules! impl_from_ptr {
    ($($ty:ty),*) => {
        $(
            impl_from!(*mut $ty => RawPtr as *mut c_void);
            impl_from!(*const $ty => RawPtr as *mut c_void);
        )*
    };
}

impl_from_ptr!(c_void, i8, u8, i16, u16, i32, u32, i64, u64, f32, f64, usize, isize);

pub struct VaList<'a> {
    args: &'a [VaArg],
    pos: usize,
}

impl<'a> VaList<'a> {
    pub fn new(args: &'a [VaArg]) -> Self {
        VaList { args, pos: 0 }
    }

    pub fn arg<T: VaArgGet>(&mut self) -> T {
        let val = &self.args[self.pos];
        self.pos += 1;
        T::get(val)
    }
}

impl Default for VaList<'_> {
    fn default() -> Self {
        VaList { args: &[], pos: 0 }
    }
}

impl Clone for VaList<'_> {
    fn clone(&self) -> Self {
        VaList {
            args: self.args,
            pos: self.pos,
        }
    }
}

fn get_int(v: &VaArg) -> i64 {
    match v {
        VaArg::Int(n) => *n as i64,
        VaArg::UInt(n) => *n as i64,
        VaArg::Long(n) => *n,
        VaArg::ULong(n) => *n as i64,
        _ => panic!("VaList::arg: expected integer, got different variant"),
    }
}

fn get_uint(v: &VaArg) -> u64 {
    match v {
        VaArg::Int(n) => *n as u64,
        VaArg::UInt(n) => *n as u64,
        VaArg::Long(n) => *n as u64,
        VaArg::ULong(n) => *n,
        _ => panic!("VaList::arg: expected unsigned integer, got different variant"),
    }
}

fn get_float(v: &VaArg) -> f64 {
    match v {
        VaArg::Double(n) => *n,
        VaArg::Int(n) => *n as f64,
        VaArg::Long(n) => *n as f64,
        _ => panic!("VaList::arg: expected float, got different variant"),
    }
}

fn get_ptr(v: &VaArg) -> *mut c_void {
    match v {
        VaArg::RawPtr(p) => *p,
        _ => panic!("VaList::arg: expected pointer, got different variant"),
    }
}

pub trait VaArgGet {
    fn get(v: &VaArg) -> Self;
}

macro_rules! impl_get_int {
    ($($ty:ty),*) => {
        $(impl VaArgGet for $ty {
            fn get(v: &VaArg) -> Self { get_int(v) as $ty }
        })*
    };
}

macro_rules! impl_get_uint {
    ($($ty:ty),*) => {
        $(impl VaArgGet for $ty {
            fn get(v: &VaArg) -> Self { get_uint(v) as $ty }
        })*
    };
}

macro_rules! impl_get_ptr {
    ($($ty:ty),*) => {
        $(impl VaArgGet for $ty {
            fn get(v: &VaArg) -> Self { get_ptr(v) as $ty }
        })*
    };
}

impl_get_int!(i8, i16, i32, i64);
impl_get_uint!(u8, u16, u32, u64);

impl VaArgGet for f32 {
    fn get(v: &VaArg) -> Self {
        get_float(v) as f32
    }
}

impl VaArgGet for f64 {
    fn get(v: &VaArg) -> Self {
        get_float(v)
    }
}

macro_rules! impl_get_ptr {
    ($($ty:ty),*) => {
        $(
            impl VaArgGet for *mut $ty {
                fn get(v: &VaArg) -> Self { get_ptr(v) as *mut $ty }
            }
            impl VaArgGet for *const $ty {
                fn get(v: &VaArg) -> Self { get_ptr(v) as *const $ty }
            }
        )*
    };
}

impl_get_ptr!(c_void, i8, u8, i16, u16, i32, u32, i64, u64, f32, f64, usize, isize);

impl<T: 'static> VaArgGet for crate::rc::Ptr<T> {
    fn get(v: &VaArg) -> Self {
        match v {
            VaArg::Ptr(any) => any.cast::<T>().expect("VaList::arg: Ptr type mismatch"),
            _ => panic!("VaList::arg: expected Ptr, got different variant"),
        }
    }
}
