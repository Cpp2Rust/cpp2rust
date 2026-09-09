extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut copies_0: i32 = unsafe { 0 };
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Counted {
    pub v: i32,
}
impl Counted {
    pub unsafe fn Counted(mut v: i32) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn Counted_pconstCounted(o: *const Counted) -> Self {
        let mut this = Self { v: (*o).v };
        copies_0.prefix_inc();
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NonConst {
    pub mark: i32,
}
impl NonConst {
    pub unsafe fn NonConst() -> Self {
        let mut this = Self { mark: 0 };
        this
    }
    pub unsafe fn NonConst_pmutNonConst(o: *mut NonConst) -> Self {
        let mut this = Self {
            mark: (((*o).mark) + (1)),
        };
        this
    }
    pub unsafe fn NonConst_pconstNonConst(o: *const NonConst) -> Self {
        let mut this = Self {
            mark: (((*o).mark) + (10)),
        };
        this
    }
}
impl Default for NonConst {
    fn default() -> Self {
        unsafe { NonConst::NonConst() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct WithDefault {
    pub v: i32,
    pub tag: i32,
}
impl WithDefault {
    pub unsafe fn WithDefault(mut v: i32) -> Self {
        let mut this = Self { v: v, tag: 0 };
        this
    }
    pub unsafe fn WithDefault_pconstWithDefault_i32(
        o: *const WithDefault,
        mut tag: Option<i32>,
    ) -> Self {
        let mut tag: i32 = tag.unwrap_or(7);
        let mut this = Self {
            v: (*o).v,
            tag: tag,
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Holder {
    pub c: Counted,
    pub arr: [Counted; 2],
}
impl Default for Holder {
    fn default() -> Self {
        Holder {
            c: <Counted>::default(),
            arr: [<Counted>::default(); 2],
        }
    }
}
pub unsafe fn by_value_1(mut c: Counted) -> i32 {
    return c.v;
}
pub unsafe fn make_2(mut v: i32) -> Counted {
    let mut c: Counted = Counted::Counted({ v });
    return c;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: Counted = Counted::Counted({ 1 });
    let mut b: Counted = a;
    let mut c: Counted = a;
    let mut d: Counted = a;
    assert!(((copies_0) == (3)));
    assert!((((b.v) == (1)) && ((c.v) == (1))) && ((d.v) == (1)));
    assert!(((unsafe { by_value_1(a,) }) == (1)));
    assert!(((copies_0) == (4)));
    let mut e: Counted = (unsafe { make_2(5) });
    assert!(((e.v) == (5)));
    assert!(((copies_0) == (5)));
    let mut f: Counted = Counted::Counted({ 6 });
    assert!(((f.v) == (6)));
    assert!(((copies_0) == (5)));
    let g: Counted = Counted::Counted({ 7 });
    let mut h: Counted = g;
    assert!(((h.v) == (7)));
    assert!(((copies_0) == (6)));
    let mut hold: Holder = Holder {
        c: Counted::Counted({ 8 }),
        arr: [Counted::Counted({ 9 }), Counted::Counted({ 10 })],
    };
    let mut hold2: Holder = hold;
    assert!(
        (((hold2.c.v) == (8)) && ((hold2.arr[(0) as usize].v) == (9)))
            && ((hold2.arr[(1) as usize].v) == (10))
    );
    assert!(((copies_0) == (9)));
    let mut vec_: Vec<Counted> = Vec::new();
    {
        let a0_clone = a.clone();
        vec_.push(a0_clone)
    };
    assert!(((vec_[(0_usize)].v) == (1)));
    assert!(((copies_0) == (10)));
    let mut n: NonConst = NonConst::NonConst();
    let mut n1: NonConst = n;
    let cn: NonConst = NonConst::NonConst();
    let mut n2: NonConst = cn;
    assert!(((n1.mark) == (1)));
    assert!(((n2.mark) == (10)));
    let mut w: WithDefault = WithDefault::WithDefault({ 3 });
    let mut w1: WithDefault = w;
    let mut w2: WithDefault = w;
    assert!(((w1.v) == (3)) && ((w1.tag) == (7)));
    assert!(((w2.v) == (3)) && ((w2.tag) == (9)));
    return 0;
}
