extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut then_runs_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut else_runs_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut dtorCount_2: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct ConvertsToBool {
    pub value: i32,
}
impl ConvertsToBool {
    pub unsafe fn to_bool(&self) -> bool {
        return ((self.value) != (0));
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_2)).prefix_inc();
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    {
        let mut condition: ConvertsToBool = ConvertsToBool { value: 0 };
        let _dtor_condition =
            std::mem::DropGuard::new(&raw mut condition, |__p| unsafe { (*__p).destructor() });
        if (unsafe { ConvertsToBool::to_bool(&condition) }) {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut then_runs_0)).prefix_inc();
        } else {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut else_runs_1)).prefix_inc();
            assert!(((condition.value) == (0)));
            assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_2)) == (0)));
        }
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut then_runs_0)) == (0)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut else_runs_1)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_2)) == (1)));
    {
        let mut condition: ConvertsToBool = ConvertsToBool { value: 6 };
        let _dtor_condition =
            std::mem::DropGuard::new(&raw mut condition, |__p| unsafe { (*__p).destructor() });
        if (unsafe { ConvertsToBool::to_bool(&condition) }) {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut then_runs_0)).prefix_inc();
            assert!(((condition.value) == (6)));
            assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_2)) == (1)));
        } else {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut else_runs_1)).prefix_inc();
        }
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut then_runs_0)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut else_runs_1)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_2)) == (2)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const then_runs_0);
    std::cell::LazyCell::force(&*&raw const else_runs_1);
    std::cell::LazyCell::force(&*&raw const dtorCount_2);
}
