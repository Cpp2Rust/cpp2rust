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
pub unsafe fn copy_or_move_into_param_4(mut t: Tracked) -> Tracked {
    return Tracked::Tracked_pmutTracked_rv({ &mut t });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: Tracked = Tracked::Tracked({ 5 });
    assert!(
        (((unsafe { chosen_overload_0((unsafe { forward_5(&mut a,) }),) }) as i32)
            == (Overload_kLvalueOverload as i32))
    );
    assert!(((a.v) == (5)));
    assert!(
        (((unsafe { chosen_overload_1((unsafe { forward_6(&mut a,) }),) }) as i32)
            == (Overload_kRvalueOverload as i32))
    );
    assert!(((a.v) == (5)));
    let mut b: Tracked = Tracked::Tracked({ 6 });
    let mut moved: Tracked = (unsafe {
        copy_or_move_into_param_4(Tracked::Tracked_pmutTracked_rv({
            (unsafe { forward_6(&mut b) })
        }))
    });
    assert!(((moved.v) == (6)));
    assert!(((moved.copies) == (0)));
    assert!(((moved.moves) == (2)));
    assert!(((b.v) == (0)));
    let mut c: Tracked = Tracked::Tracked({ 7 });
    let mut copied: Tracked = (unsafe {
        copy_or_move_into_param_4(Tracked::Tracked_pconstTracked({
            (unsafe { forward_5(&mut c) })
        }))
    });
    assert!(((copied.v) == (7)));
    assert!(((copied.copies) == (1)));
    assert!(((copied.moves) == (1)));
    assert!(((c.v) == (7)));
    let mut i: i32 = 8;
    assert!(
        (((unsafe { chosen_overload_2((unsafe { forward_7(&mut i,) }),) }) as i32)
            == (Overload_kIntLvalueOverload as i32))
    );
    assert!(
        (((unsafe { chosen_overload_3((unsafe { forward_8(&mut i,) }),) }) as i32)
            == (Overload_kIntRvalueOverload as i32))
    );
    assert!(
        (((unsafe { chosen_overload_3((unsafe { forward_9(&mut i,) }),) }) as i32)
            == (Overload_kIntRvalueOverload as i32))
    );
    assert!(((i) == (8)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
