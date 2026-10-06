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
pub struct Owner {
    pub p: *mut i32,
}
impl Owner {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self {
            p: (Box::leak(Box::new(v)) as *mut i32),
        };
        this
    }
    pub unsafe fn move_from(o: *mut Owner) -> Self {
        let mut this = Self { p: (*o).p };
        (*o).p = std::ptr::null_mut();
        this
    }
    pub unsafe fn destructor(&mut self) {
        {
            let __p = self.p;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(__p))
            }
        };
    }
    pub unsafe fn take(&mut self) -> FnPtr<fn() -> i32> {
        return lambda_unsafe!(
            {
                let self_: Owner = Owner::move_from({ &mut (*(self as *mut Owner)) });
            },
            || -> i32 {
                return (*self_.p);
            },
            move_from {
                self_: Owner::move_from({ &mut self_ },),
            },
            destroy = {
                Owner::destructor(&mut self.self_);
            }
        );
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut o: Owner = Owner::new({ 5 });
    let _dtor_o = ScopedDestructorUnsafe::new(&raw mut o, Owner::destructor);
    let mut f: FnPtr<fn() -> i32> = lambda_unsafe!(
        {
            let h: Owner = Owner::move_from({ &mut o });
        },
        || -> i32 {
            return (*h.p);
        },
        move_from {
            h: Owner::move_from({ &mut h },),
        },
        destroy = {
            Owner::destructor(&mut self.h);
        }
    );
    let _dtor_f = ScopedDestructorUnsafe::new(&raw mut f, |__f| __f.destroy());
    assert!((o.p).is_null());
    assert!(((unsafe { f.call() }) == (5)));
    let mut g: FnPtr<fn() -> i32> = f.move_from();
    let _dtor_g = ScopedDestructorUnsafe::new(&raw mut g, |__f| __f.destroy());
    assert!(((unsafe { g.call() }) == (5)));
    let mut total: i32 = 0;
    let mut consume: FnPtr<fn()> = lambda_unsafe!(
        {
            let h: Owner = Owner::new({ 7 });
            let total: *mut i32 = &mut total;
        },
        || {
            (*total) += (*h.p);
            (*h.p) = 0;
        },
        move_from {
            h: Owner::move_from({ &mut h },),
            total: total,
        },
        destroy = {
            Owner::destructor(&mut self.h);
        }
    );
    let _dtor_consume = ScopedDestructorUnsafe::new(&raw mut consume, |__f| __f.destroy());
    (unsafe { consume.call() });
    (unsafe { consume.call() });
    assert!(((total) == (7)));
    let mut o2: Owner = Owner::new({ 9 });
    let _dtor_o2 = ScopedDestructorUnsafe::new(&raw mut o2, Owner::destructor);
    let mut t: FnPtr<fn() -> i32> = (unsafe { Owner::take(&mut o2) });
    let _dtor_t = ScopedDestructorUnsafe::new(&raw mut t, |__f| __f.destroy());
    assert!((o2.p).is_null());
    assert!(((unsafe { t.call() }) == (9)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
