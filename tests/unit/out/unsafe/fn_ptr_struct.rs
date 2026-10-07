extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Handler {
    pub tag: i32,
    pub cb: Option<unsafe fn(i32) -> i32>,
}
pub unsafe fn double_it_0(mut x: i32) -> i32 {
    return ((x) * (2));
}
pub unsafe fn negate_1(mut x: i32) -> i32 {
    return -x;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {}
impl S {
    pub unsafe fn pick_1(mut x: i32) -> i32 {
        return ((x) + (1));
    }
    pub unsafe fn pick_2(mut x: i64) -> i32 {
        return ((x as i32) + (2));
    }
    pub unsafe fn solo(mut x: i32) -> i32 {
        return ((x) + (3));
    }
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct MoveOnly {
    pub data_: i32,
}
impl MoveOnly {
    pub unsafe fn new(mut data: i32) -> Self {
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn move_from(x: *mut MoveOnly) -> Self {
        let mut this = Self { data_: (*x).data_ };
        (*x).data_ = 0;
        this
    }
    pub unsafe fn get(&self) -> i32 {
        return self.data_;
    }
}
pub unsafe fn make_move_only_2() -> MoveOnly {
    return MoveOnly::new({ 4 });
}
pub unsafe fn scale_move_only_3(mut x: MoveOnly) -> MoveOnly {
    return MoveOnly::new({ ((unsafe { MoveOnly::get(&x) }) * (10)) });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p1: Option<unsafe fn(i32) -> i32> = (Some(S::pick_1));
    let mut p2: Option<unsafe fn(i32) -> i32> = Some(S::solo);
    assert!(((unsafe { (p1).unwrap()(5,) }) == (6)));
    assert!(((unsafe { (p2).unwrap()(5,) }) == (8)));
    assert!(((unsafe { S::pick_2(5_i64,) }) == (7)));
    let mut h3: Handler = Handler {
        tag: 3,
        cb: (Some(S::pick_1)),
    };
    assert!(((unsafe { (h3.cb).unwrap()(1,) }) == (2)));
    let mut h1: Handler = Handler {
        tag: 1,
        cb: Some(double_it_0),
    };
    let mut h2: Handler = Handler {
        tag: 2,
        cb: Some(negate_1),
    };
    assert!(!((h1.cb).is_none()));
    assert!(((unsafe { (h1.cb).unwrap()(5,) }) == (10)));
    assert!(((unsafe { (h2.cb).unwrap()(7,) }) == (-7_i32)));
    (h1.cb) = Some(negate_1);
    assert!(((unsafe { (h1.cb).unwrap()(3,) }) == (-3_i32)));
    assert!(((h1.cb) == (h2.cb)));
    let mut mk: Option<unsafe fn() -> MoveOnly> = Some(make_move_only_2);
    let mut m: MoveOnly = (unsafe { (mk).unwrap()() });
    assert!(((unsafe { MoveOnly::get(&m,) }) == (4)));
    let mut sc: Option<unsafe fn(MoveOnly) -> MoveOnly> = Some(scale_move_only_3);
    let mut n: MoveOnly = (unsafe { (sc).unwrap()(MoveOnly::move_from({ &mut m })) });
    assert!(((unsafe { MoveOnly::get(&n,) }) == (40)));
    assert!(((unsafe { MoveOnly::get(&m,) }) == (0)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
