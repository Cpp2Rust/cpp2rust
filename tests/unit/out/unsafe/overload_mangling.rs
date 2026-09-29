extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn inc_0(mut p: *mut i32) {
    (*p) += 1;
}
pub unsafe fn add_1(mut p: *mut i32, mut n: i32) {
    (*p) += n;
}
pub unsafe fn twice_2(mut n: i32) -> i32 {
    return ((n) * (2));
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Access_S_ {}
impl Access_S_ {
    pub unsafe fn get_1(&mut self, mut p: *mut S) -> i32 {
        return (*p).base;
    }
    pub unsafe fn get_2(&mut self, mut p: *const S) -> i32 {
        return (((*p).base) + (1));
    }
    pub unsafe fn ref_3(&mut self, r: *mut S) -> i32 {
        return (((*r).base) + (2));
    }
    pub unsafe fn ref_4(&mut self, r: *const S) -> i32 {
        return (((*r).base) + (3));
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct S {
    pub base: i32,
}
impl S {
    pub unsafe fn plain_4(&self, mut x: i32) -> i32 {
        return ((self.base) + (x));
    }
    pub unsafe fn plain_5(&self, mut x: i64) -> i32 {
        return (((self.base) + (x as i32)) + (1));
    }
    pub unsafe fn take_6(&self, x: *mut i32) -> i32 {
        return (((self.base) + (*x)) + (1));
    }
    pub unsafe fn take_7(&self, x: *mut i32) -> i32 {
        return (((self.base) + (*x)) + (2));
    }
    pub unsafe fn pick_8(&self, mut p: (i32, i32)) -> i32 {
        return ((self.base) + (p.0));
    }
    pub unsafe fn pick_9(&self, mut p: (i32, i64)) -> i32 {
        return ((self.base) + (p.1 as i32));
    }
    pub unsafe fn apply_10(&self, mut f: Option<unsafe fn(*mut i32)>, mut x: i32) -> i32 {
        (unsafe { (f).unwrap()((&mut x as *mut i32)) });
        return ((self.base) + (x));
    }
    pub unsafe fn apply_11(&self, mut f: Option<unsafe fn(*mut i32, i32)>, mut x: i32) -> i32 {
        (unsafe { (f).unwrap()((&mut x as *mut i32), 10) });
        return ((self.base) + (x));
    }
    pub unsafe fn apply_12(&self, mut f: Option<unsafe fn(i32) -> i32>, mut x: i32) -> i32 {
        return ((self.base) + (unsafe { (f).unwrap()(x) }));
    }
    pub unsafe fn combine_13(
        &self,
        p: *const (i32, i64),
        mut f: Option<unsafe fn(i32) -> i32>,
        mut q: *const i32,
        mut n: u64,
    ) -> i32 {
        return ((((self.base) + ((*p).1 as i32)) + (unsafe { (f).unwrap()((*q)) })) + (n as i32));
    }
    pub unsafe fn combine_14(
        &self,
        p: *const (i32, i32),
        mut f: Option<unsafe fn(*mut i32, i32)>,
        mut q: *mut i32,
        mut n: u64,
    ) -> i32 {
        (unsafe {
            let _arg0: *mut i32 = q;
            let _arg1: i32 = (n as i32);
            (f).unwrap()(_arg0, _arg1)
        });
        return (((self.base) + ((*p).0)) + (*q));
    }
    pub unsafe fn width_1_char(&self, mut x: i32) -> i32 {
        return ((self.base) + ((x) * (::std::mem::size_of::<libc::c_char>() as i32)));
    }
    pub unsafe fn width_1_int(&self, mut x: i32) -> i32 {
        return ((self.base) + ((x) * (::std::mem::size_of::<i32>() as i32)));
    }
    pub unsafe fn scale_2_2(&self, mut x: i32) -> i32 {
        return ((self.base) + ((x) * (2)));
    }
    pub unsafe fn scale_2_3(&self, mut x: i32) -> i32 {
        return ((self.base) + ((x) * (3)));
    }
    pub unsafe fn count_3(&self, mut x: i32) -> i32 {
        return (((self.base) + (x)) + (0 as i32));
    }
    pub unsafe fn count_3_int_long(&self, mut x: i32) -> i32 {
        return (((self.base) + (x)) + (2 as i32));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, Default)]
pub struct Box {
    pub v: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = S { base: 100 };
    assert!(((unsafe { S::width_1_char(&s, 3,) }) == (103)));
    assert!(((unsafe { S::width_1_int(&s, 3,) }) == (112)));
    assert!(((unsafe { S::scale_2_2(&s, 5,) }) == (110)));
    assert!(((unsafe { S::scale_2_3(&s, 5,) }) == (115)));
    assert!(((unsafe { S::count_3(&s, 1,) }) == (101)));
    assert!(((unsafe { S::count_3_int_long(&s, 1,) }) == (103)));
    assert!(((unsafe { S::plain_4(&s, 1,) }) == (101)));
    assert!(((unsafe { S::plain_5(&s, 1_i64,) }) == (102)));
    let mut y: i32 = 1;
    assert!(((unsafe { S::take_6(&s, &mut y,) }) == (102)));
    assert!(
        ((unsafe {
            let mut _x: i32 = 5;
            S::take_7(&s, &mut _x)
        }) == (107))
    );
    assert!(((unsafe { S::pick_8(&s, (1.into(), 2.into()),) }) == (101)));
    assert!(((unsafe { S::pick_9(&s, (1.into(), 2_i64.into()),) }) == (102)));
    assert!(((unsafe { S::apply_10(&s, Some(inc_0), 1,) }) == (102)));
    assert!(((unsafe { S::apply_11(&s, Some(add_1), 1,) }) == (111)));
    assert!(((unsafe { S::apply_12(&s, Some(twice_2), 3,) }) == (106)));
    let c: i32 = 3;
    assert!(
        ((unsafe {
            let mut _p: (i32, i64) = (1.into(), 2_i64.into());
            S::combine_13(&s, &mut _p, Some(twice_2), (&c as *const i32), 4_u64)
        }) == (112))
    );
    let mut z: i32 = 1;
    assert!(
        ((unsafe {
            let mut _p: (i32, i32) = (1.into(), 2.into());
            S::combine_14(&s, &mut _p, Some(add_1), (&mut z as *mut i32), 5_u64)
        }) == (107))
    );
    assert!(((z) == (6)));
    let mut a: Access_S_ = <Access_S_>::default();
    let mut cs: *const S = (&mut s as *mut S).cast_const();
    let cr: *const S = &s;
    assert!(((unsafe { Access_S_::get_1(&mut a, (&mut s as *mut S),) }) == (100)));
    assert!(((unsafe { Access_S_::get_2(&mut a, cs,) }) == (101)));
    assert!(((unsafe { Access_S_::ref_3(&mut a, &mut s,) }) == (102)));
    assert!(
        ((unsafe {
            let _r: *const S = cr;
            Access_S_::ref_4(&mut a, _r)
        }) == (103))
    );
    let mut b: Box = Box { v: 4 };
    assert!(((b.v) == (4)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
