extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Default)]
pub struct MoveOnly {
    pub v: i32,
}
impl MoveOnly {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn move_from(o: *mut MoveOnly) -> Self {
        let mut this = Self { v: (*o).v };
        (*o).v = 0;
        this
    }
    pub unsafe fn move_assign(&mut self, o: *mut MoveOnly) -> *mut MoveOnly {
        if ((self as *mut MoveOnly) == (o)) {
            return &mut (*(self as *mut MoveOnly));
        }
        self.v = (*o).v;
        (*o).v = 0;
        return &mut (*(self as *mut MoveOnly));
    }
}
#[repr(C)]
#[derive()]
pub struct ConstMoveAssign {
    pub mark: i32,
}
impl ConstMoveAssign {
    pub unsafe fn new() -> Self {
        let mut this = Self { mark: 0 };
        this
    }
    pub unsafe fn operator_assign_pmutConstMoveAssign_rv(
        &mut self,
        o: *mut ConstMoveAssign,
    ) -> *mut ConstMoveAssign {
        self.mark = (((*o).mark) + (1));
        return &mut (*(self as *mut ConstMoveAssign));
    }
    pub unsafe fn operator_assign_pconstConstMoveAssign_rv(
        &mut self,
        o: *const ConstMoveAssign,
    ) -> *mut ConstMoveAssign {
        self.mark = (((*o).mark) + (10));
        return &mut (*(self as *mut ConstMoveAssign));
    }
}
impl Default for ConstMoveAssign {
    fn default() -> Self {
        unsafe { ConstMoveAssign::new() }
    }
}
pub unsafe fn make_0(mut v: i32) -> MoveOnly {
    let mut m: MoveOnly = MoveOnly::new({ v });
    return MoveOnly::move_from({ &mut m });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: MoveOnly = MoveOnly::new({ 1 });
    let mut b: MoveOnly = MoveOnly::new({ 2 });
    let mut c: MoveOnly = MoveOnly::new({ 3 });
    (unsafe { MoveOnly::move_assign(&mut a, &mut b) });
    assert!(((a.v) == (2)));
    assert!(((b.v) == (0)));
    (unsafe {
        let mut _o: MoveOnly = MoveOnly::new({ 3 });
        MoveOnly::move_assign(&mut b, &mut _o)
    });
    (unsafe {
        MoveOnly::move_assign(
            &mut c,
            &mut (*(unsafe { MoveOnly::move_assign(&mut a, &mut b) })),
        )
    });
    assert!((((b.v) == (0)) && ((a.v) == (0))) && ((c.v) == (3)));
    (unsafe {
        let mut _o: MoveOnly = MoveOnly::new({ 5 });
        MoveOnly::move_assign(&mut a, &mut _o)
    });
    assert!(((a.v) == (5)));
    (unsafe {
        let mut _o: MoveOnly = (unsafe { make_0(6) });
        MoveOnly::move_assign(&mut a, &mut _o)
    });
    assert!(((a.v) == (6)));
    (unsafe {
        let _o: *mut MoveOnly = &mut a;
        MoveOnly::move_assign(&mut a, _o)
    });
    assert!(((a.v) == (6)));
    let mut vec_: Vec<MoveOnly> = Vec::new();
    vec_.push(MoveOnly::new({ 7 }));
    let mut d: MoveOnly = MoveOnly::new({ 8 });
    (unsafe { MoveOnly::move_assign(&mut vec_[(0_usize)], &mut d) });
    assert!(((vec_[(0_usize)].v) == (8)));
    assert!(((d.v) == (0)));
    let mut m: ConstMoveAssign = ConstMoveAssign::new();
    let mut m1: ConstMoveAssign = ConstMoveAssign::new();
    let mut m2: ConstMoveAssign = ConstMoveAssign::new();
    let cm: ConstMoveAssign = ConstMoveAssign::new();
    (unsafe { ConstMoveAssign::operator_assign_pmutConstMoveAssign_rv(&mut m1, &mut m) });
    (unsafe { ConstMoveAssign::operator_assign_pconstConstMoveAssign_rv(&mut m2, &cm) });
    assert!(((m1.mark) == (1)));
    assert!(((m2.mark) == (10)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
