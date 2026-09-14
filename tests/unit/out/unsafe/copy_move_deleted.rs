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
pub struct NoCopy {
    pub v: i32,
}
impl NoCopy {
    pub unsafe fn NoCopy(mut v: i32) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn NoCopy_pmutNoCopy(o: *mut NoCopy) -> Self {
        let mut this = Self { v: (*o).v };
        (*o).v = 0;
        this
    }
    pub unsafe fn operator_assign_pmutNoCopy(&mut self, o: *mut NoCopy) -> *mut NoCopy {
        self.v = (*o).v;
        (*o).v = 0;
        return &mut (*(self as *mut NoCopy)) as *mut NoCopy;
    }
}
#[repr(C)]
#[derive()]
pub struct PrivateCopy {
    pub v: i32,
}
impl PrivateCopy {
    pub unsafe fn PrivateCopy() -> Self {
        let mut this = Self { v: 0 };
        this
    }
    pub unsafe fn PrivateCopy_pmutPrivateCopy(o: *mut PrivateCopy) -> Self {
        let mut this = Self { v: (*o).v };
        (*o).v = 0;
        this
    }
    pub unsafe fn operator_assign_pmutPrivateCopy(
        &mut self,
        o: *mut PrivateCopy,
    ) -> *mut PrivateCopy {
        self.v = (*o).v;
        (*o).v = 0;
        return &mut (*(self as *mut PrivateCopy)) as *mut PrivateCopy;
    }
}
impl Default for PrivateCopy {
    fn default() -> Self {
        unsafe { PrivateCopy::PrivateCopy() }
    }
}
#[repr(C)]
#[derive()]
pub struct Immovable {
    pub v: i32,
}
impl Immovable {
    pub unsafe fn Immovable() -> Self {
        let mut this = Self { v: 0 };
        this
    }
}
impl Default for Immovable {
    fn default() -> Self {
        unsafe { Immovable::Immovable() }
    }
}
#[repr(C)]
#[derive(Default)]
pub struct Container {
    pub inner: NoCopy,
    pub tag: i32,
}
impl Container {
    pub unsafe fn Container_pmutContainer(_a0: *mut Container) -> Self {
        let mut this = Self {
            inner: NoCopy::NoCopy_pmutNoCopy({ &mut (*_a0).inner as *mut NoCopy }),
            tag: (*_a0).tag,
        };
        this
    }
}
pub unsafe fn bump_0(mut p: *mut NoCopy) {
    (*p).v.postfix_inc();
}
pub unsafe fn bump_ref_1(r: *mut Immovable) {
    (*r).v.postfix_inc();
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: NoCopy = NoCopy::NoCopy({ 1 });
    let mut b: NoCopy = NoCopy::NoCopy_pmutNoCopy({ &mut a as *mut NoCopy });
    assert!(((b.v) == (1)) && ((a.v) == (0)));
    (unsafe { NoCopy::operator_assign_pmutNoCopy(&mut a, &mut b as *mut NoCopy) });
    assert!(((a.v) == (1)) && ((b.v) == (0)));
    (unsafe { bump_0((&mut a as *mut NoCopy)) });
    assert!(((a.v) == (2)));
    let mut p: PrivateCopy = PrivateCopy::PrivateCopy();
    p.v = 3;
    let mut q: PrivateCopy =
        PrivateCopy::PrivateCopy_pmutPrivateCopy({ &mut p as *mut PrivateCopy });
    assert!(((q.v) == (3)) && ((p.v) == (0)));
    (unsafe { PrivateCopy::operator_assign_pmutPrivateCopy(&mut p, &mut q as *mut PrivateCopy) });
    assert!(((p.v) == (3)) && ((q.v) == (0)));
    let mut im: Immovable = Immovable::Immovable();
    im.v = 4;
    (unsafe { bump_ref_1(&mut im as *mut Immovable) });
    let mut pim: *mut Immovable = (&mut im as *mut Immovable);
    assert!((((*pim).v) == (5)));
    let mut c: Container = Container {
        inner: NoCopy::NoCopy({ 6 }),
        tag: 7,
    };
    let mut d: Container = Container::Container_pmutContainer({ &mut c as *mut Container });
    assert!((((d.inner.v) == (6)) && ((d.tag) == (7))) && ((c.inner.v) == (0)));
    return 0;
}
