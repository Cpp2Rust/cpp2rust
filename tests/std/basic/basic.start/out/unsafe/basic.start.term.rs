extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Tracker {
    pub tag: i32,
}
impl Tracker {
    pub unsafe fn new(mut tag: i32) -> Self {
        let mut this = Self { tag: tag };
        (unsafe { libc::printf(c"construct %d\n".as_ptr() as *const libc::c_char, (tag)) });
        this
    }
    pub unsafe fn destructor(&mut self) {
        (unsafe { libc::printf(c"destroy %d\n".as_ptr() as *const libc::c_char, (self.tag)) });
    }
    pub unsafe fn touch(&self) {
        (unsafe { libc::printf(c"touch %d\n".as_ptr() as *const libc::c_char, (self.tag)) });
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct HoldsTracker {
    pub member: Tracker,
}
impl HoldsTracker {
    pub unsafe fn destructor(&mut self) {
        Tracker::destructor(&mut self.member);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TriviallyDestructible {
    pub value: i32,
}
pub static mut global_0: std::cell::LazyCell<Tracker> = std::cell::LazyCell::new(|| unsafe {
    {
        let __static = Tracker::new({ 1 });
        libcc2rs::register_static_destructor(|| {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut global_0)).destructor()
        });
        __static
    }
});
pub unsafe fn never_called_1() {
    (unsafe {
        Tracker::touch(&(*std::cell::LazyCell::force_mut(&mut *&raw mut never_constructed_2)))
    });
}
pub unsafe fn test_3() {
    (unsafe { Tracker::touch(&(*std::cell::LazyCell::force_mut(&mut *&raw mut global_0))) });
}
static mut never_constructed_2: std::cell::LazyCell<Tracker> =
    std::cell::LazyCell::new(|| unsafe {
        {
            let __static = Tracker::new({ 100 });
            libcc2rs::register_static_destructor(|| {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut never_constructed_2)).destructor()
            });
            __static
        }
    });
pub unsafe fn local_static_during_global_init_4() -> *const Tracker {
    return &(*std::cell::LazyCell::force_mut(&mut *&raw mut during_init_5));
}
pub static mut before_6: std::cell::LazyCell<Tracker> = std::cell::LazyCell::new(|| unsafe {
    {
        let __static = Tracker::new({ 2 });
        libcc2rs::register_static_destructor(|| {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut before_6)).destructor()
        });
        __static
    }
});
pub static mut after_7: std::cell::LazyCell<Tracker> = std::cell::LazyCell::new(|| unsafe {
    {
        let __static =
            Tracker::new({ (((*(unsafe { local_static_during_global_init_4() })).tag) + (1)) });
        libcc2rs::register_static_destructor(|| {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut after_7)).destructor()
        });
        __static
    }
});
pub static mut holder_8: std::cell::LazyCell<HoldsTracker> = std::cell::LazyCell::new(|| unsafe {
    {
        let __static = HoldsTracker {
            member: Tracker::new({ 5 }),
        };
        libcc2rs::register_static_destructor(|| {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut holder_8)).destructor()
        });
        __static
    }
});
pub static mut trivial_9: std::cell::LazyCell<TriviallyDestructible> =
    std::cell::LazyCell::new(|| unsafe { TriviallyDestructible { value: 6 } });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p3_reverse_completion_order_WithStaticMember {}
pub static mut member_10: std::cell::LazyCell<Tracker> = std::cell::LazyCell::new(|| unsafe {
    {
        let __static = Tracker::new({ 7 });
        libcc2rs::register_static_destructor(|| {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut member_10)).destructor()
        });
        __static
    }
});
pub static mut capturing_11: std::cell::LazyCell<FnPtr<fn() -> i32>> =
    std::cell::LazyCell::new(|| unsafe {
        {
            let __static = lambda_unsafe!(
                {
                    let captured: Tracker = Tracker::new({ 8 });
                },
                || -> i32 {
                    return captured.tag;
                }
            );
            libcc2rs::register_static_destructor(|| {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut capturing_11)).destroy()
            });
            __static
        }
    });
pub unsafe fn run_local_static_12() {
    (unsafe { Tracker::touch(&(*std::cell::LazyCell::force_mut(&mut *&raw mut during_main_13))) });
}
pub unsafe fn test_14() {
    (unsafe { run_local_static_12() });
    (unsafe { run_local_static_12() });
    (unsafe { Tracker::touch(&(*(unsafe { local_static_during_global_init_4() }))) });
    (unsafe {
        libc::printf(
            c"trivial %d\n".as_ptr() as *const libc::c_char,
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut trivial_9)).value),
        )
    });
    (unsafe { Tracker::touch(&(*std::cell::LazyCell::force_mut(&mut *&raw mut member_10))) });
    (unsafe {
        libc::printf(
            c"captured %d\n".as_ptr() as *const libc::c_char,
            (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut capturing_11)).call() }),
        )
    });
}
static mut during_init_5: std::cell::LazyCell<Tracker> = std::cell::LazyCell::new(|| unsafe {
    {
        let __static = Tracker::new({ 3 });
        libcc2rs::register_static_destructor(|| {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut during_init_5)).destructor()
        });
        __static
    }
});
static mut during_main_13: std::cell::LazyCell<Tracker> = std::cell::LazyCell::new(|| unsafe {
    {
        let __static = Tracker::new({ 9 });
        libcc2rs::register_static_destructor(|| {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut during_main_13)).destructor()
        });
        __static
    }
});
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_3() });
    (unsafe { test_14() });
    (unsafe { libc::printf(c"return from main\n".as_ptr() as *const libc::c_char) });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const global_0);
    std::cell::LazyCell::force(&*&raw const before_6);
    std::cell::LazyCell::force(&*&raw const after_7);
    std::cell::LazyCell::force(&*&raw const holder_8);
    std::cell::LazyCell::force(&*&raw const trivial_9);
    std::cell::LazyCell::force(&*&raw const member_10);
    std::cell::LazyCell::force(&*&raw const capturing_11);
}
