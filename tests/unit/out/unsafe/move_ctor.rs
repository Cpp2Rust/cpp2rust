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
    pub unsafe fn MoveOnly(mut v: i32) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn MoveOnly_pmutMoveOnly(o: *mut MoveOnly) -> Self {
        let mut this = Self { v: (*o).v };
        (*o).v = 0;
        this
    }
}
#[repr(C)]
#[derive()]
pub struct ConstMove {
    pub mark: i32,
}
impl ConstMove {
    pub unsafe fn ConstMove() -> Self {
        let mut this = Self { mark: 0 };
        this
    }
    pub unsafe fn ConstMove_pmutConstMove(o: *mut ConstMove) -> Self {
        let mut this = Self {
            mark: (((*o).mark) + (1)),
        };
        this
    }
    pub unsafe fn ConstMove_pconstConstMove(o: *const ConstMove) -> Self {
        let mut this = Self {
            mark: (((*o).mark) + (10)),
        };
        this
    }
}
impl Default for ConstMove {
    fn default() -> Self {
        unsafe { ConstMove::ConstMove() }
    }
}
pub unsafe fn by_value_0(mut m: MoveOnly) -> i32 {
    return m.v;
}
pub unsafe fn make_1(mut v: i32) -> MoveOnly {
    let mut m: MoveOnly = MoveOnly::MoveOnly({ v });
    return MoveOnly::MoveOnly_pmutMoveOnly({ &mut m });
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: MoveOnly = MoveOnly::MoveOnly({ 1 });
    let mut b: MoveOnly = MoveOnly::MoveOnly_pmutMoveOnly({ &mut a });
    assert!(((b.v) == (1)));
    assert!(((a.v) == (0)));
    let mut c: MoveOnly = MoveOnly::MoveOnly_pmutMoveOnly({ &mut b });
    assert!(((c.v) == (1)));
    assert!(((b.v) == (0)));
    let mut d: MoveOnly = MoveOnly::MoveOnly_pmutMoveOnly({ &mut c });
    assert!(((d.v) == (1)));
    assert!(((c.v) == (0)));
    let mut e: MoveOnly = (unsafe { make_1(5) });
    assert!(((e.v) == (5)));
    assert!(((unsafe { by_value_0(MoveOnly::MoveOnly({ 6 },),) }) == (6)));
    assert!(((unsafe { by_value_0(MoveOnly::MoveOnly_pmutMoveOnly({ &mut e },),) }) == (5)));
    assert!(((e.v) == (0)));
    let mut vec_: Vec<MoveOnly> = Vec::new();
    vec_.push(MoveOnly::MoveOnly({ 7 }));
    let mut f: MoveOnly = MoveOnly::MoveOnly({ 8 });
    vec_.push(std::mem::take(&mut f));
    assert!(((vec_[(0_usize)].v) == (7)) && ((vec_[(1_usize)].v) == (8)));
    assert!(((f.v) == (0)));
    let mut m: ConstMove = ConstMove::ConstMove();
    let mut m1: ConstMove = ConstMove::ConstMove_pmutConstMove({ &mut m });
    let cm: ConstMove = ConstMove::ConstMove();
    let mut m2: ConstMove = ConstMove::ConstMove_pconstConstMove({ &cm });
    assert!(((m1.mark) == (1)));
    assert!(((m2.mark) == (10)));
    return 0;
}
