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
#[derive(Default)]
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
impl Clone for Counted {
    fn clone(&self) -> Self {
        unsafe { Counted::Counted_pconstCounted(self as *const Counted) }
    }
}
#[repr(C)]
#[derive()]
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
impl Clone for NonConst {
    fn clone(&self) -> Self {
        unsafe { NonConst::NonConst_pmutNonConst(self as *const NonConst as *mut NonConst) }
    }
}
impl Default for NonConst {
    fn default() -> Self {
        unsafe { NonConst::NonConst() }
    }
}
#[repr(C)]
#[derive(Clone)]
pub struct Holder {
    pub c: Counted,
    pub arr: [Counted; 2],
}
impl Default for Holder {
    fn default() -> Self {
        Holder {
            c: <Counted>::default(),
            arr: std::array::from_fn::<_, 2, _>(|_| <Counted>::default()),
        }
    }
}
pub unsafe fn by_value_1(mut c: Counted) -> i32 {
    return c.v;
}
pub unsafe fn make_2(mut v: i32) -> Counted {
    let mut c: Counted = Counted::Counted({ v });
    return Counted::Counted_pconstCounted({ &mut c });
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: Counted = Counted::Counted({ 1 });
    let mut b: Counted = Counted::Counted_pconstCounted({ &a as *const Counted });
    let mut c: Counted = Counted::Counted_pconstCounted({ &a as *const Counted });
    let mut d: Counted = Counted::Counted_pconstCounted({ &a as *const Counted });
    assert!(((copies_0) == (3)));
    assert!((((b.v) == (1)) && ((c.v) == (1))) && ((d.v) == (1)));
    assert!(
        ((unsafe { by_value_1(Counted::Counted_pconstCounted({ &a as *const Counted },),) })
            == (1))
    );
    assert!(((copies_0) == (4)));
    let mut e: Counted = (unsafe { make_2(5) });
    assert!(((e.v) == (5)));
    assert!(((copies_0) == (5)));
    let mut f: Counted = Counted::Counted({ 6 });
    assert!(((f.v) == (6)));
    assert!(((copies_0) == (5)));
    let g: Counted = Counted::Counted({ 7 });
    let mut h: Counted = Counted::Counted_pconstCounted({ &g as *const Counted });
    assert!(((h.v) == (7)));
    assert!(((copies_0) == (6)));
    let mut hold: Holder = Holder {
        c: Counted::Counted({ 8 }),
        arr: [Counted::Counted({ 9 }), Counted::Counted({ 10 })],
    };
    let mut hold2: Holder = hold.clone();
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
    let mut n1: NonConst = NonConst::NonConst_pmutNonConst({ &mut n as *mut NonConst });
    let cn: NonConst = NonConst::NonConst();
    let mut n2: NonConst = NonConst::NonConst_pconstNonConst({ &cn as *const NonConst });
    assert!(((n1.mark) == (1)));
    assert!(((n2.mark) == (10)));
    return 0;
}
