extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut copies_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(VaArg, FnPtrArg, Default)]
pub struct Counted {
    pub v: i32,
}
impl Counted {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn copy_from(o: *const Counted) -> Self {
        let mut this = Self { v: (*o).v };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
        this
    }
}
impl Clone for Counted {
    fn clone(&self) -> Self {
        unsafe { Counted::copy_from(self as *const Counted) }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg)]
pub struct NonConst {
    pub mark: i32,
}
impl NonConst {
    pub unsafe fn new() -> Self {
        let mut this = Self { mark: 0 };
        this
    }
    pub unsafe fn new_1(o: *mut NonConst) -> Self {
        let mut this = Self {
            mark: (((*o).mark) + (1)),
        };
        this
    }
    pub unsafe fn new_2(o: *const NonConst) -> Self {
        let mut this = Self {
            mark: (((*o).mark) + (10)),
        };
        this
    }
}
impl Clone for NonConst {
    fn clone(&self) -> Self {
        unsafe { NonConst::new_1(self as *const NonConst as *mut NonConst) }
    }
}
impl Default for NonConst {
    fn default() -> Self {
        unsafe { NonConst::new() }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, Default)]
pub struct Ignored {
    pub v: i32,
}
impl Ignored {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn copy_from(_a0: *const Ignored) -> Self {
        let mut this = Self { v: -1_i32 };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
        this
    }
}
impl Clone for Ignored {
    fn clone(&self) -> Self {
        unsafe { Ignored::copy_from(self as *const Ignored) }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct Holder {
    pub c: Counted,
    pub arr: [Counted; 2],
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct Box_int_ {
    pub val: i32,
}
impl Box_int_ {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { val: v };
        this
    }
    pub unsafe fn move_from(_a0: *mut Box_int_) -> Self {
        let mut this = Self { val: (*_a0).val };
        this
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct Owner {
    pub box_: Box_int_,
}
impl Owner {
    pub unsafe fn move_from(_a0: *mut Owner) -> Self {
        let mut this = Self {
            box_: Box_int_::move_from({ &mut (*_a0).box_ }),
        };
        this
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, Default)]
pub struct OutOfLine {
    pub v: i32,
}
impl OutOfLine {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn copy_from(o: *const OutOfLine) -> Self {
        let mut this = Self {
            v: (((*o).v) + (100)),
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
        this
    }
}
impl Clone for OutOfLine {
    fn clone(&self) -> Self {
        unsafe { OutOfLine::copy_from(self as *const OutOfLine) }
    }
}
impl OutOfLine {}
#[repr(C)]
#[derive(VaArg, FnPtrArg)]
pub struct Tracked {
    pub copied_from: i32,
}
impl Tracked {
    pub unsafe fn copy_from(o: *const Tracked) -> Self {
        let mut this = Self { copied_from: 0 };
        (*(&raw const (*o).copied_from).cast_mut()).prefix_inc();
        this
    }
    pub unsafe fn copy_assign(&mut self, o: *const Tracked) -> *mut Tracked {
        (*(&raw const (*o).copied_from).cast_mut()).prefix_inc();
        return &mut (*(self as *mut Tracked));
    }
}
impl Clone for Tracked {
    fn clone(&self) -> Self {
        unsafe { Tracked::copy_from(self as *const Tracked) }
    }
}
impl Default for Tracked {
    fn default() -> Self {
        Tracked { copied_from: 0 }
    }
}
pub unsafe fn touch_1(mut t: *const Tracked) {
    (*(&raw const (*t).copied_from).cast_mut()).prefix_inc();
}
pub unsafe fn by_value_2(mut c: Counted) -> i32 {
    return c.v;
}
pub unsafe fn make_3(mut v: i32) -> Counted {
    let mut c: Counted = Counted::new({ v });
    return Counted::copy_from({ &c });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: Counted = Counted::new({ 1 });
    let mut b: Counted = Counted::copy_from({ &a });
    let mut c: Counted = Counted::copy_from({ &a });
    let mut d: Counted = Counted::copy_from({ &a });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (3)));
    assert!((((b.v) == (1)) && ((c.v) == (1))) && ((d.v) == (1)));
    assert!(((unsafe { by_value_2(Counted::copy_from({ &a },),) }) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (4)));
    let mut e: Counted = (unsafe { make_3(5) });
    assert!(((e.v) == (5)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (5)));
    let mut f: Counted = Counted::new({ 6 });
    assert!(((f.v) == (6)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (5)));
    let g: Counted = Counted::new({ 7 });
    let mut h: Counted = Counted::copy_from({ &g });
    assert!(((h.v) == (7)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (6)));
    let mut hold: Holder = Holder {
        c: Counted::new({ 8 }),
        arr: [Counted::new({ 9 }), Counted::new({ 10 })],
    };
    let mut hold2: Holder = hold.clone();
    assert!(
        (((hold2.c.v) == (8)) && ((hold2.arr[(0) as usize].v) == (9)))
            && ((hold2.arr[(1) as usize].v) == (10))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (9)));
    let mut vec_: Vec<Counted> = Vec::new();
    {
        let a0_clone = a.clone();
        vec_.push(a0_clone)
    };
    assert!(((vec_[(0_usize)].v) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (10)));
    let mut i1: Ignored = Ignored::new({ 1 });
    let mut i2: Ignored = Ignored::copy_from({ &i1 });
    assert!(((i1.v) == (1)) && ((i2.v) == (-1_i32)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (11)));
    let mut o: Owner = Owner {
        box_: Box_int_::new({ 3 }),
    };
    let mut o2: Owner = Owner::move_from({ &mut o });
    assert!(((o2.box_.val) == (3)));
    let mut ol1: OutOfLine = OutOfLine::new({ 5 });
    let mut ol2: OutOfLine = OutOfLine::copy_from({ &ol1 });
    assert!(((ol2.v) == (105)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (12)));
    let mut n: NonConst = NonConst::new();
    let mut n1: NonConst = NonConst::new_1({ &mut n });
    let cn: NonConst = NonConst::new();
    let mut n2: NonConst = NonConst::new_2({ &cn });
    assert!(((n1.mark) == (1)));
    assert!(((n2.mark) == (10)));
    let mut t1: Tracked = <Tracked>::default();
    let mut t2: Tracked = Tracked::copy_from({ &t1 });
    (unsafe { Tracked::copy_assign(&mut t2, &t1) });
    assert!(((t1.copied_from) == (2)));
    let ct: Tracked = <Tracked>::default();
    let mut t3: Tracked = Tracked::copy_from({ &ct });
    (unsafe { touch_1((&ct as *const Tracked)) });
    assert!(((ct.copied_from) == (2)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copies_0);
}
