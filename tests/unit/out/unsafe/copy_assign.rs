extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut assigns_0: i32 = unsafe { 0 };
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Partial {
    pub v: i32,
    pub keep: i32,
}
impl Partial {
    pub unsafe fn Partial(mut v: i32, mut keep: i32) -> Self {
        let mut this = Self { v: v, keep: keep };
        this
    }
    pub unsafe fn Partial_pconstPartial(o: *const Partial) -> Self {
        let mut this = Self {
            v: (*o).v,
            keep: (*o).keep,
        };
        this
    }
    pub unsafe fn operator_assign(&mut self, o: *const Partial) -> *mut Partial {
        if (((self as *mut Partial).cast_const()) == (o)) {
            return &mut (*(self as *mut Partial)) as *mut Partial;
        }
        self.v = (*o).v;
        assigns_0.prefix_inc();
        return &mut (*(self as *mut Partial)) as *mut Partial;
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NonConstAssign {
    pub mark: i32,
}
impl NonConstAssign {
    pub unsafe fn NonConstAssign() -> Self {
        let mut this = Self { mark: 0 };
        this
    }
    pub unsafe fn operator_assign_pmutNonConstAssign(
        &mut self,
        o: *mut NonConstAssign,
    ) -> *mut NonConstAssign {
        self.mark = (((*o).mark) + (1));
        return &mut (*(self as *mut NonConstAssign)) as *mut NonConstAssign;
    }
    pub unsafe fn operator_assign_pconstNonConstAssign(
        &mut self,
        o: *const NonConstAssign,
    ) -> *mut NonConstAssign {
        self.mark = (((*o).mark) + (10));
        return &mut (*(self as *mut NonConstAssign)) as *mut NonConstAssign;
    }
}
impl Default for NonConstAssign {
    fn default() -> Self {
        unsafe { NonConstAssign::NonConstAssign() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RefQualified {
    pub mark: i32,
}
impl RefQualified {
    pub unsafe fn RefQualified() -> Self {
        let mut this = Self { mark: 0 };
        this
    }
    pub unsafe fn operator_assign_pconstRefQualified_lref(
        &mut self,
        o: *const RefQualified,
    ) -> *mut RefQualified {
        self.mark = (((*o).mark) + (1));
        return &mut (*(self as *mut RefQualified)) as *mut RefQualified;
    }
    pub unsafe fn operator_assign_pconstRefQualified_rref(
        &mut self,
        o: *const RefQualified,
    ) -> *mut RefQualified {
        self.mark = (((*o).mark) + (10));
        return (self as *mut RefQualified);
    }
}
impl Default for RefQualified {
    fn default() -> Self {
        unsafe { RefQualified::RefQualified() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Holder {
    pub p: Partial,
    pub arr: [Partial; 2],
}
impl Default for Holder {
    fn default() -> Self {
        Holder {
            p: <Partial>::default(),
            arr: [<Partial>::default(); 2],
        }
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: Partial = Partial::Partial({ 1 }, { 100 });
    let mut b: Partial = Partial::Partial({ 2 }, { 200 });
    let mut c: Partial = Partial::Partial({ 3 }, { 300 });
    (unsafe { Partial::operator_assign(&mut a, &b as *const Partial) });
    assert!(((a.v) == (2)) && ((a.keep) == (100)));
    assert!(((assigns_0) == (1)));
    (unsafe {
        Partial::operator_assign(
            &mut c,
            &(*(unsafe { Partial::operator_assign(&mut a, &b as *const Partial) }))
                as *const Partial,
        )
    });
    assert!(((c.v) == (2)) && ((c.keep) == (300)));
    assert!(((assigns_0) == (3)));
    (unsafe {
        let _o: *const Partial = &a as *const Partial;
        Partial::operator_assign(&mut a, _o)
    });
    assert!(((assigns_0) == (3)));
    (unsafe {
        let mut _o: Partial = Partial::Partial({ 9 }, { 900 });
        Partial::operator_assign(&mut a, &mut _o)
    });
    assert!(((a.v) == (9)) && ((a.keep) == (100)));
    assert!(((assigns_0) == (4)));
    let ra: *mut Partial = &mut a as *mut Partial;
    (unsafe {
        let _o: *const Partial = &c as *const Partial;
        Partial::operator_assign(&mut (*ra), _o)
    });
    assert!(((a.v) == (2)));
    let mut pa: *mut Partial = (&mut a as *mut Partial);
    (unsafe {
        let _o: *const Partial = &b as *const Partial;
        Partial::operator_assign(&mut (*pa), _o)
    });
    assert!(((a.v) == (2)));
    assert!(((assigns_0) == (6)));
    let mut h: Holder = Holder {
        p: Partial::Partial({ 4 }, { 40 }),
        arr: [
            Partial::Partial({ 5 }, { 50 }),
            Partial::Partial({ 6 }, { 60 }),
        ],
    };
    (unsafe { Partial::operator_assign(&mut h.p, &b as *const Partial) });
    (unsafe { Partial::operator_assign(&mut h.arr[(1) as usize], &c as *const Partial) });
    assert!(((h.p.v) == (2)) && ((h.p.keep) == (40)));
    assert!(((h.arr[(1) as usize].v) == (2)) && ((h.arr[(1) as usize].keep) == (60)));
    assert!(((assigns_0) == (8)));
    let mut n: NonConstAssign = NonConstAssign::NonConstAssign();
    let mut n1: NonConstAssign = NonConstAssign::NonConstAssign();
    let mut n2: NonConstAssign = NonConstAssign::NonConstAssign();
    let cn: NonConstAssign = NonConstAssign::NonConstAssign();
    (unsafe {
        NonConstAssign::operator_assign_pmutNonConstAssign(&mut n1, &mut n as *mut NonConstAssign)
    });
    (unsafe {
        NonConstAssign::operator_assign_pconstNonConstAssign(&mut n2, &cn as *const NonConstAssign)
    });
    assert!(((n1.mark) == (1)));
    assert!(((n2.mark) == (10)));
    let mut r: RefQualified = RefQualified::RefQualified();
    let mut r1: RefQualified = RefQualified::RefQualified();
    (unsafe {
        RefQualified::operator_assign_pconstRefQualified_lref(&mut r1, &r as *const RefQualified)
    });
    assert!(((r1.mark) == (1)));
    let mut r2: RefQualified = (*(unsafe {
        RefQualified::operator_assign_pconstRefQualified_rref(
            &mut RefQualified::RefQualified(),
            &r as *const RefQualified,
        )
    }));
    assert!(((r2.mark) == (10)));
    return 0;
}
