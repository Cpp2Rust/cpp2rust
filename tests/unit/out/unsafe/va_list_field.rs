extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn sum_0(mut n: i32, __args: &[VaArg]) -> i32 {
    let mut ap: VaList = VaList::default();
    ap = VaList::new(__args);
    let mut guard: Guard_1 = Guard_1::new({ &mut ap });
    let _dtor_guard = ScopedDestructorUnsafe::new(&raw mut guard, Guard_1::destructor);
    let mut total: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        total += (*guard.ap).arg::<i32>();
        i.prefix_inc();
    }
    return total;
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Guard_1 {
    pub ap: *mut VaList,
    pub active: bool,
}
impl Guard_1 {
    pub unsafe fn new(val: *mut VaList) -> Self {
        let mut this = Self {
            ap: val,
            active: true,
        };
        this
    }
    pub unsafe fn destructor(&mut self) {
        if self.active {}
    }
}
pub unsafe fn sum_ptr_2(mut n: i32, __args: &[VaArg]) -> i32 {
    let mut ap: VaList = VaList::default();
    ap = VaList::new(__args);
    let mut cursor: Cursor_3 = Cursor_3 {
        ap: (&mut ap as *mut VaList),
    };
    let mut total: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        total += (*cursor.ap).arg::<i32>();
        i.prefix_inc();
    }
    return total;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Cursor_3 {
    pub ap: *mut VaList,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((unsafe { sum_0(3, &[(1).into(), (2).into(), (3).into(),]) }) == (6)));
    assert!(((unsafe { sum_ptr_2(2, &[(4).into(), (5).into(),]) }) == (9)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
