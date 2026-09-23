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
    pub unsafe fn new(mut v: i32) -> Self {
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
pub unsafe fn forward_by_decltype_2(x: *mut Tracked) -> Overload {
    return (unsafe { chosen_overload_0(x) });
}
pub unsafe fn forward_by_decltype_3(x: *mut Tracked) -> Overload {
    return (unsafe { chosen_overload_1(x) });
}
pub unsafe fn forward_abbreviated_4(x: *mut Tracked) -> Overload {
    return (unsafe { chosen_overload_0(x) });
}
pub unsafe fn forward_abbreviated_5(x: *mut Tracked) -> Overload {
    return (unsafe { chosen_overload_1(x) });
}
pub unsafe fn forward_abbreviated_pack_6(args_0: *mut Tracked, args_1: *mut Tracked) -> i32 {
    return (((unsafe { chosen_overload_0(args_0) }) as i32)
        + ((unsafe { chosen_overload_1(args_1) }) as i32));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: Tracked = Tracked::new({ 3 });
    assert!(
        (((unsafe { forward_by_decltype_2(&mut a,) }) as i32) == (Overload_kLvalueOverload as i32))
    );
    assert!(((a.v) == (3)));
    assert!(
        (((unsafe {
            let mut _x: Tracked = Tracked::new({ 4 });
            forward_by_decltype_3(&mut _x)
        }) as i32)
            == (Overload_kRvalueOverload as i32))
    );
    let mut b: Tracked = Tracked::new({ 5 });
    assert!(
        (((unsafe { forward_abbreviated_4(&mut b,) }) as i32) == (Overload_kLvalueOverload as i32))
    );
    assert!(((b.v) == (5)));
    assert!(
        (((unsafe {
            let mut _x: Tracked = Tracked::new({ 6 });
            forward_abbreviated_5(&mut _x)
        }) as i32)
            == (Overload_kRvalueOverload as i32))
    );
    let mut c: Tracked = Tracked::new({ 7 });
    assert!(
        ((unsafe {
            let mut _args_1: Tracked = Tracked::new({ 8 });
            forward_abbreviated_pack_6(&mut c, &mut _args_1)
        }) == ((Overload_kLvalueOverload as i32) + (Overload_kRvalueOverload as i32)))
    );
    assert!(((c.v) == (7)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
