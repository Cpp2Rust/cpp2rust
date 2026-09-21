extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type Overload = u32;
pub const Overload_kLvalueOverload: Overload = 1;
pub const Overload_kRvalueOverload: Overload = 2;
#[repr(C)]
#[derive(Default)]
pub struct Tracked {
    pub v: i32,
    pub copies: i32,
    pub moves: i32,
}
impl Tracked {
    pub unsafe fn Tracked(mut v: i32) -> Self {
        let mut this = Self {
            v: v,
            copies: 0,
            moves: 0,
        };
        this
    }
    pub unsafe fn Tracked_pconstTracked(o: *const Tracked) -> Self {
        let mut this = Self {
            v: (*o).v,
            copies: (((*o).copies) + (1)),
            moves: (*o).moves,
        };
        this
    }
    pub unsafe fn Tracked_pmutTracked_rv(o: *mut Tracked) -> Self {
        let mut this = Self {
            v: (*o).v,
            copies: (*o).copies,
            moves: (((*o).moves) + (1)),
        };
        (*o).v = 0;
        this
    }
}
impl Clone for Tracked {
    fn clone(&self) -> Self {
        unsafe { Tracked::Tracked_pconstTracked(self as *const Tracked) }
    }
}
pub unsafe fn chosen_overload_0(_a0: *const Tracked) -> Overload {
    return Overload_kLvalueOverload;
}
pub unsafe fn chosen_overload_1(_a0: *mut Tracked) -> Overload {
    return Overload_kRvalueOverload;
}
impl Holder {
    pub unsafe fn Holder1(x: *mut Tracked) -> Self {
        let mut this = Self {
            t: Tracked::Tracked_pconstTracked({ (unsafe { forward_2(x) }) }),
        };
        this
    }
}
impl Holder {
    pub unsafe fn Holder2(x: *mut Tracked) -> Self {
        let mut this = Self {
            t: Tracked::Tracked_pmutTracked_rv({ (unsafe { forward_3(x) }) }),
        };
        this
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct Holder {
    pub t: Tracked,
}
pub unsafe fn forward_once_4(x: *mut Tracked) -> Overload {
    return (unsafe { chosen_overload_0((unsafe { forward_2(x) })) });
}
pub unsafe fn forward_once_5(x: *mut Tracked) -> Overload {
    return (unsafe { chosen_overload_1((unsafe { forward_3(x) })) });
}
pub unsafe fn forward_twice_6(x: *mut Tracked) -> Overload {
    return (unsafe { forward_once_4((unsafe { forward_2(x) })) });
}
pub unsafe fn forward_twice_7(x: *mut Tracked) -> Overload {
    return (unsafe { forward_once_5((unsafe { forward_3(x) })) });
}
pub unsafe fn forward_into_ctor_8(x: *mut Tracked) -> Holder {
    return Holder::Holder1({ (unsafe { forward_2(x) }) });
}
pub unsafe fn forward_into_ctor_9(x: *mut Tracked) -> Holder {
    return (*(unsafe { forward_3(x) }));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut lvalue: Tracked = Tracked::Tracked({ 7 });
    assert!(
        (((unsafe { forward_once_4(&mut lvalue,) }) as i32) == (Overload_kLvalueOverload as i32))
    );
    assert!(((lvalue.v) == (7)));
    assert!(
        (((unsafe {
            let mut _x: Tracked = Tracked::Tracked({ 8 });
            forward_once_5(&mut _x)
        }) as i32)
            == (Overload_kRvalueOverload as i32))
    );
    let mut relayed: Tracked = Tracked::Tracked({ 9 });
    assert!(
        (((unsafe { forward_twice_6(&mut relayed,) }) as i32) == (Overload_kLvalueOverload as i32))
    );
    assert!(((relayed.v) == (9)));
    assert!(
        (((unsafe {
            let mut _x: Tracked = Tracked::Tracked({ 10 });
            forward_twice_7(&mut _x)
        }) as i32)
            == (Overload_kRvalueOverload as i32))
    );
    let mut kept: Tracked = Tracked::Tracked({ 11 });
    let mut from_lvalue: Holder = (unsafe { forward_into_ctor_8(&mut kept) });
    assert!(((from_lvalue.t.v) == (11)));
    assert!(((from_lvalue.t.copies) == (1)));
    assert!(((from_lvalue.t.moves) == (0)));
    assert!(((kept.v) == (11)));
    let mut from_rvalue: Holder = (unsafe {
        let mut _x: Tracked = Tracked::Tracked({ 12 });
        forward_into_ctor_9(&mut _x)
    });
    assert!(((from_rvalue.t.v) == (12)));
    assert!(((from_rvalue.t.copies) == (0)));
    assert!(((from_rvalue.t.moves) == (1)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
