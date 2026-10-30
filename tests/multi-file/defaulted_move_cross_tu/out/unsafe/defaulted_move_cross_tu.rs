extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, MoveAssignUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct S {
    pub v: Vec<i32>,
    pub n: [i32; 2],
}
impl S {
    pub unsafe fn new(mut x: i32) -> Self {
        let mut this = Self {
            v: vec![x; (x as usize) as usize],
            n: [x, ((x) + (1))],
        };
        this
    }
    pub unsafe fn move_from(_a0: *mut S) -> Self {
        let mut this = Self {
            v: std::mem::take(&mut (*_a0).v),
            n: std::array::from_fn::<_, 2, _>(|__i: usize| (*_a0).n[(__i)]),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut S) -> *mut S {
        self.v = std::mem::take(&mut (*_a0).v);
        {
            if 8_usize != 0 {
                ::std::ptr::copy_nonoverlapping(
                    ((&mut (*_a0).n as *mut [i32; 2]) as *const ::libc::c_void),
                    ((&mut self.n as *mut [i32; 2]) as *mut ::libc::c_void),
                    8_usize as usize,
                )
            }
            ((&mut self.n as *mut [i32; 2]) as *mut ::libc::c_void)
        };
        return &mut (*(self as *mut S));
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct T {
    pub v: Vec<i32>,
}
impl T {
    pub unsafe fn size(&self) -> i32 {
        return (self.v.len() as i32);
    }
    pub unsafe fn move_from(_a0: *mut T) -> Self {
        let mut this = Self {
            v: std::mem::take(&mut (*_a0).v),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut T) -> *mut T {
        self.v = std::mem::take(&mut (*_a0).v);
        return &mut (*(self as *mut T));
    }
}
pub unsafe fn sum_0(s: *const S) -> i32 {
    return ((((*s).v.len() as i32) + ((*s).n[(0) as usize])) + ((*s).n[(1) as usize]));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = S::new({ 2 });
    assert!(((unsafe { sum_0(&s,) }) == (7)));
    assert!(((unsafe { shuffle_1(3,) }) == (10)));
    let mut t: T = T {
        v: Vec::from([1, 2]),
    };
    assert!(((unsafe { T::size(&t,) }) == (2)));
    return 0;
}
pub unsafe fn shuffle_1(mut x: i32) -> i32 {
    let mut a: S = S::new({ x });
    let mut b: S = S::move_from({ &mut a });
    assert!(a.v.is_empty());
    let mut c: S = S::new({ 1 });
    (unsafe { S::move_assign(&mut c, &mut b) });
    assert!(b.v.is_empty());
    let mut t: T = T { v: Vec::from([1]) };
    let mut u: T = T::move_from({ &mut t });
    let mut w: T = <T>::default();
    (unsafe { T::move_assign(&mut w, &mut u) });
    assert!(
        (((unsafe { T::size(&t,) }) == (0)) && ((unsafe { T::size(&u,) }) == (0)))
            && ((unsafe { T::size(&w,) }) == (1))
    );
    return (unsafe { sum_0(&c) });
}
pub unsafe fn __cpp2rust_init_globals() {}
