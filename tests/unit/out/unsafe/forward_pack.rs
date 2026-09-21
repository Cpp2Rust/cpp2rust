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
pub const Overload_kIntLvalueOverload: Overload = 3;
pub const Overload_kIntRvalueOverload: Overload = 4;
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
pub unsafe fn chosen_overload_2(_a0: *const i32) -> Overload {
    return Overload_kIntLvalueOverload;
}
pub unsafe fn chosen_overload_3(_a0: *mut i32) -> Overload {
    return Overload_kIntRvalueOverload;
}
pub unsafe fn forward_pack_4() -> i32 {
    let mut digits: i32 = 0;
    ();
    return digits;
}
pub unsafe fn forward_pack_5(args: *mut Tracked) -> i32 {
    let mut digits: i32 = 0;
    digits = (((digits) * (10)) + ((unsafe { chosen_overload_0(args) }) as i32));
    return digits;
}
pub unsafe fn forward_pack_6(args: *mut Tracked) -> i32 {
    let mut digits: i32 = 0;
    digits = (((digits) * (10)) + ((unsafe { chosen_overload_1(args) }) as i32));
    return digits;
}
pub unsafe fn forward_pack_7(
    args_0: *mut Tracked,
    args_1: *mut Tracked,
    args_2: *mut i32,
    args_3: *mut i32,
) -> i32 {
    let mut digits: i32 = 0;
    {
        digits = (((digits) * (10)) + ((unsafe { chosen_overload_0(args_0) }) as i32));
        {
            digits = (((digits) * (10)) + ((unsafe { chosen_overload_1(args_1) }) as i32));
            {
                digits = (((digits) * (10)) + ((unsafe { chosen_overload_2(args_2) }) as i32));
                digits = (((digits) * (10)) + ((unsafe { chosen_overload_3(args_3) }) as i32))
            }
        }
    };
    return digits;
}
impl Pair {
    pub unsafe fn Pair(x: *mut Tracked, y: *mut Tracked) -> Self {
        let mut this = Self {
            a: Tracked::Tracked_pconstTracked({ x }),
            b: Tracked::Tracked_pmutTracked_rv({ y }),
        };
        this
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct Pair {
    pub a: Tracked,
    pub b: Tracked,
}
pub unsafe fn forward_pack_into_ctor_8(args_0: *mut Tracked, args_1: *mut Tracked) -> Pair {
    return Pair::Pair({ args_0 }, { args_1 });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((unsafe { forward_pack_4() }) == (0)));
    let mut a: Tracked = Tracked::Tracked({ 1 });
    assert!(((unsafe { forward_pack_5(&mut a,) }) == (Overload_kLvalueOverload as i32)));
    assert!(((a.v) == (1)));
    assert!(
        ((unsafe {
            let mut _args: Tracked = Tracked::Tracked({ 2 });
            forward_pack_6(&mut _args)
        }) == (Overload_kRvalueOverload as i32))
    );
    let mut i: i32 = 3;
    assert!(
        ((unsafe {
            let mut _args_1: Tracked = Tracked::Tracked({ 4 });
            let mut _args_3: i32 = 5;
            forward_pack_7(&mut a, &mut _args_1, &mut i, &mut _args_3)
        }) == (1234))
    );
    assert!(((a.v) == (1)));
    assert!(((i) == (3)));
    let mut lhs: Tracked = Tracked::Tracked({ 6 });
    let mut p: Pair = (unsafe {
        let mut _args_1: Tracked = Tracked::Tracked({ 7 });
        forward_pack_into_ctor_8(&mut lhs, &mut _args_1)
    });
    assert!(((p.a.v) == (6)));
    assert!(((p.a.copies) == (1)));
    assert!(((p.b.v) == (7)));
    assert!(((p.b.copies) == (0)));
    assert!(((p.b.moves) == (1)));
    assert!(((lhs.v) == (6)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
