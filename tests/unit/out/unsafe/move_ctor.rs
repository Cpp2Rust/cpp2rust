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
    pub unsafe fn MoveOnly_pmutMoveOnly_rv(o: *mut MoveOnly) -> Self {
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
    pub unsafe fn ConstMove_pmutConstMove_rv(o: *mut ConstMove) -> Self {
        let mut this = Self {
            mark: (((*o).mark) + (1)),
        };
        this
    }
    pub unsafe fn ConstMove_pconstConstMove_rv(o: *const ConstMove) -> Self {
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
#[repr(C)]
#[derive(Default)]
pub struct ThrowingMove {
    pub v: i32,
    pub copies: i32,
    pub moves: i32,
}
impl ThrowingMove {
    pub unsafe fn ThrowingMove(mut v: i32) -> Self {
        let mut this = Self {
            v: v,
            copies: 0,
            moves: 0,
        };
        this
    }
    pub unsafe fn ThrowingMove_pconstThrowingMove(o: *const ThrowingMove) -> Self {
        let mut this = Self {
            v: (*o).v,
            copies: (((*o).copies) + (1)),
            moves: (*o).moves,
        };
        this
    }
    pub unsafe fn ThrowingMove_pmutThrowingMove_rv(o: *mut ThrowingMove) -> Self {
        let mut this = Self {
            v: (*o).v,
            copies: (*o).copies,
            moves: (((*o).moves) + (1)),
        };
        (*o).v = 0;
        this
    }
}
impl Clone for ThrowingMove {
    fn clone(&self) -> Self {
        unsafe { ThrowingMove::ThrowingMove_pconstThrowingMove(self as *const ThrowingMove) }
    }
}
#[repr(C)]
#[derive(Default)]
pub struct NoexceptMove {
    pub v: i32,
    pub copies: i32,
    pub moves: i32,
}
impl NoexceptMove {
    pub unsafe fn NoexceptMove(mut v: i32) -> Self {
        let mut this = Self {
            v: v,
            copies: 0,
            moves: 0,
        };
        this
    }
    pub unsafe fn NoexceptMove_pconstNoexceptMove(o: *const NoexceptMove) -> Self {
        let mut this = Self {
            v: (*o).v,
            copies: (((*o).copies) + (1)),
            moves: (*o).moves,
        };
        this
    }
    pub unsafe fn NoexceptMove_pmutNoexceptMove_rv(o: *mut NoexceptMove) -> Self {
        let mut this = Self {
            v: (*o).v,
            copies: (*o).copies,
            moves: (((*o).moves) + (1)),
        };
        (*o).v = 0;
        this
    }
}
impl Clone for NoexceptMove {
    fn clone(&self) -> Self {
        unsafe { NoexceptMove::NoexceptMove_pconstNoexceptMove(self as *const NoexceptMove) }
    }
}
pub unsafe fn by_value_0(mut m: MoveOnly) -> i32 {
    return m.v;
}
pub unsafe fn make_1(mut v: i32) -> MoveOnly {
    let mut m: MoveOnly = MoveOnly::MoveOnly({ v });
    return MoveOnly::MoveOnly_pmutMoveOnly_rv({ &mut m });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: MoveOnly = MoveOnly::MoveOnly({ 1 });
    let mut b: MoveOnly = MoveOnly::MoveOnly_pmutMoveOnly_rv({ &mut a });
    assert!(((b.v) == (1)));
    assert!(((a.v) == (0)));
    let mut c: MoveOnly = MoveOnly::MoveOnly_pmutMoveOnly_rv({ &mut b });
    assert!(((c.v) == (1)));
    assert!(((b.v) == (0)));
    let mut d: MoveOnly = MoveOnly::MoveOnly_pmutMoveOnly_rv({ &mut c });
    assert!(((d.v) == (1)));
    assert!(((c.v) == (0)));
    let mut e: MoveOnly = (unsafe { make_1(5) });
    assert!(((e.v) == (5)));
    assert!(((unsafe { by_value_0(MoveOnly::MoveOnly({ 6 },),) }) == (6)));
    assert!(((unsafe { by_value_0(MoveOnly::MoveOnly_pmutMoveOnly_rv({ &mut e },),) }) == (5)));
    assert!(((e.v) == (0)));
    let mut vec_: Vec<MoveOnly> = Vec::new();
    vec_.push(MoveOnly::MoveOnly({ 7 }));
    let mut f: MoveOnly = MoveOnly::MoveOnly({ 8 });
    vec_.push(MoveOnly::MoveOnly_pmutMoveOnly_rv({ &mut f }));
    assert!(((vec_[(0_usize)].v) == (7)) && ((vec_[(1_usize)].v) == (8)));
    assert!(((f.v) == (0)));
    let mut m: ConstMove = ConstMove::ConstMove();
    let mut m1: ConstMove = ConstMove::ConstMove_pmutConstMove_rv({ &mut m });
    let cm: ConstMove = ConstMove::ConstMove();
    let mut m2: ConstMove = ConstMove::ConstMove_pconstConstMove_rv({ &cm });
    assert!(((m1.mark) == (1)));
    assert!(((m2.mark) == (10)));
    let mut t: ThrowingMove = ThrowingMove::ThrowingMove({ 1 });
    let mut t1: ThrowingMove = ThrowingMove::ThrowingMove_pconstThrowingMove({ &t });
    assert!(((t1.v) == (1)));
    assert!(((t1.copies) == (1)));
    assert!(((t1.moves) == (0)));
    assert!(((t.v) == (1)));
    let mut n: NoexceptMove = NoexceptMove::NoexceptMove({ 2 });
    let mut n1: NoexceptMove = NoexceptMove::NoexceptMove_pmutNoexceptMove_rv({ &mut n });
    assert!(((n1.v) == (2)));
    assert!(((n1.copies) == (0)));
    assert!(((n1.moves) == (1)));
    assert!(((n.v) == (0)));
    let mut g: MoveOnly = MoveOnly::MoveOnly({ 3 });
    let mut g1: MoveOnly = MoveOnly::MoveOnly_pmutMoveOnly_rv({ &mut g });
    assert!(((g1.v) == (3)));
    assert!(((g.v) == (0)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
