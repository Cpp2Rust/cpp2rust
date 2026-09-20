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
pub struct S {
    pub base: i32,
}
impl S {
    pub unsafe fn plain_i32_const(&self, mut x: i32) -> i32 {
        return ((self.base) + (x));
    }
    pub unsafe fn plain_i64_const(&self, mut x: i64) -> i32 {
        return (((self.base) + (x as i32)) + (1));
    }
    pub unsafe fn take_pmuti32_const(&self, x: *mut i32) -> i32 {
        return (((self.base) + (*x)) + (1));
    }
    pub unsafe fn take_pmuti32_rv_const(&self, x: *mut i32) -> i32 {
        return (((self.base) + (*x)) + (2));
    }
    pub unsafe fn pick_i32_i32_const(&self, mut p: (i32, i32)) -> i32 {
        return ((self.base) + (p.0));
    }
    pub unsafe fn pick_i32_i64_const(&self, mut p: (i32, i64)) -> i32 {
        return ((self.base) + (p.1 as i32));
    }
    pub unsafe fn apply_Optionunsafefnpmuti32__i32_const(
        &self,
        mut f: Option<unsafe fn(*mut i32)>,
        mut x: i32,
    ) -> i32 {
        (unsafe { (f).unwrap()((&mut x as *mut i32)) });
        return ((self.base) + (x));
    }
    pub unsafe fn apply_Optionunsafefnpmuti32_i32__i32_const(
        &self,
        mut f: Option<unsafe fn(*mut i32, i32)>,
        mut x: i32,
    ) -> i32 {
        (unsafe { (f).unwrap()((&mut x as *mut i32), 10) });
        return ((self.base) + (x));
    }
    pub unsafe fn apply_Optionunsafefni32_i32_i32_const(
        &self,
        mut f: Option<unsafe fn(i32) -> i32>,
        mut x: i32,
    ) -> i32 {
        return ((self.base) + (unsafe { (f).unwrap()(x) }));
    }
    pub unsafe fn width_i32__char_const(&self, mut x: i32) -> i32 {
        return ((self.base) + ((x) * (::std::mem::size_of::<libc::c_char>() as i32)));
    }
    pub unsafe fn width_i32__int_const(&self, mut x: i32) -> i32 {
        return ((self.base) + ((x) * (::std::mem::size_of::<i32>() as i32)));
    }
    pub unsafe fn scale_i32__2_const(&self, mut x: i32) -> i32 {
        return ((self.base) + ((x) * (2)));
    }
    pub unsafe fn scale_i32__3_const(&self, mut x: i32) -> i32 {
        return ((self.base) + ((x) * (3)));
    }
    pub unsafe fn count_i32_const(&self, mut x: i32) -> i32 {
        return (((self.base) + (x)) + (0 as i32));
    }
    pub unsafe fn count_i32__int_long_const(&self, mut x: i32) -> i32 {
        return (((self.base) + (x)) + (2 as i32));
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
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
    assert!(((unsafe { S::width_i32__char_const(&s, 3,) }) == (103)));
    assert!(((unsafe { S::width_i32__int_const(&s, 3,) }) == (112)));
    assert!(((unsafe { S::scale_i32__2_const(&s, 5,) }) == (110)));
    assert!(((unsafe { S::scale_i32__3_const(&s, 5,) }) == (115)));
    assert!(((unsafe { S::count_i32_const(&s, 1,) }) == (101)));
    assert!(((unsafe { S::count_i32__int_long_const(&s, 1,) }) == (103)));
    assert!(((unsafe { S::plain_i32_const(&s, 1,) }) == (101)));
    assert!(((unsafe { S::plain_i64_const(&s, 1_i64,) }) == (102)));
    let mut y: i32 = 1;
    assert!(((unsafe { S::take_pmuti32_const(&s, &mut y,) }) == (102)));
    assert!(
        ((unsafe {
            let mut _x: i32 = 5;
            S::take_pmuti32_rv_const(&s, &mut _x)
        }) == (107))
    );
    assert!(((unsafe { S::pick_i32_i32_const(&s, (1.into(), 2.into()),) }) == (101)));
    assert!(((unsafe { S::pick_i32_i64_const(&s, (1.into(), 2_i64.into()),) }) == (102)));
    assert!(((unsafe { S::apply_Optionunsafefnpmuti32__i32_const(&s, Some(inc_0), 1,) }) == (102)));
    assert!(
        ((unsafe { S::apply_Optionunsafefnpmuti32_i32__i32_const(&s, Some(add_1), 1,) }) == (111))
    );
    assert!(
        ((unsafe { S::apply_Optionunsafefni32_i32_i32_const(&s, Some(twice_2), 3,) }) == (106))
    );
    let mut b: Box = Box { v: 4 };
    assert!(((b.v) == (4)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
