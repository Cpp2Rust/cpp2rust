extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Inner {
    pub values: Vec<i32>,
}
impl Inner {
    pub unsafe fn move_from(_a0: *mut Inner) -> Self {
        let mut this = Self {
            values: std::mem::take(&mut (*_a0).values),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Inner) -> *mut Inner {
        self.values = std::mem::take(&mut (*_a0).values);
        return &mut (*(self as *mut Inner));
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Outer {
    pub values: Vec<i32>,
    pub inner: Inner,
    pub name: [libc::c_char; 8],
}
impl Outer {
    pub unsafe fn move_from(_a0: *mut Outer) -> Self {
        let mut this = Self {
            values: std::mem::take(&mut (*_a0).values),
            inner: Inner::move_from({ &mut (*_a0).inner }),
            name: std::array::from_fn::<_, 8, _>(|__i: usize| (*_a0).name[(__i)]),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Outer) -> *mut Outer {
        self.values = std::mem::take(&mut (*_a0).values);
        (unsafe {
            let _arg0: *mut Inner = &mut (*_a0).inner;
            Inner::move_assign(&mut self.inner, _arg0)
        });
        match (
            { ((&mut self.name as *mut [libc::c_char; 8]) as *mut ::libc::c_void) },
            { ((&mut (*_a0).name as *mut [libc::c_char; 8]) as *const ::libc::c_void) },
        ) {
            (mut __tmp_0, mut __tmp_1) => {
                if 8_usize != 0 {
                    ::std::ptr::copy_nonoverlapping(__tmp_1, __tmp_0, 8_usize as usize)
                }
                __tmp_0
            }
        };
        return &mut (*(self as *mut Outer));
    }
}
pub unsafe fn make_0(calls: *mut i32) -> Outer {
    (*calls).prefix_inc();
    return Outer {
        values: Vec::from([1, 2, 3]),
        inner: Inner {
            values: Vec::from([4, 5, 6]),
        },
        name: std::mem::transmute(*b"abc\0\0\0\0\0"),
    };
}
pub unsafe fn sum_1(values: *const Vec<i32>) -> i32 {
    let mut total: i32 = 0;
    'loop_: for value in IterRange::new((*values).begin(), (*values).end()) {
        let mut value: i32 = (*value.iter_deref());
        total += value;
    }
    return total;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut calls: i32 = 0;
    let mut member: i32 = (unsafe { sum_1(&(unsafe { make_0(&mut calls) }).values) });
    let mut nested: i32 = (unsafe { sum_1(&(unsafe { make_0(&mut calls) }).inner.values) });
    let mut element: i32 = (*(&mut ((unsafe { make_0(&mut calls) }).values)
        [(((calls) - (2)) as usize) as usize] as *mut i32));
    let mut length: usize =
        libc::strlen(((unsafe { make_0(&mut calls) }).name.as_mut_ptr()).cast_const());
    let mut aggregate: i32 = (unsafe {
        sum_1(
            &Outer {
                values: Vec::from([7]),
                inner: Inner {
                    values: Vec::from([8, 9]),
                },
                name: std::mem::transmute(*b"x\0\0\0\0\0\0\0"),
            }
            .inner
            .values,
        )
    });
    (unsafe {
        libc::printf(
            c"%d %d %d %zu %d %d\n".as_ptr() as *const libc::c_char,
            (member),
            (nested),
            (element),
            (length),
            (aggregate),
            (calls),
        )
    });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
