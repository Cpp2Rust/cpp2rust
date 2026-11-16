extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut copyCount_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut moveCount_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct CountsCopiesAndMoves {}
impl CountsCopiesAndMoves {
    pub unsafe fn copy_from(_a0: *const CountsCopiesAndMoves) -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_0)).prefix_inc();
        this
    }
    pub unsafe fn move_from(_a0: *mut CountsCopiesAndMoves) -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moveCount_1)).prefix_inc();
        this
    }
}
impl Clone for CountsCopiesAndMoves {
    fn clone(&self) -> Self {
        unsafe { CountsCopiesAndMoves::copy_from(self as *const CountsCopiesAndMoves) }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct HoldsCounted {
    pub member: CountsCopiesAndMoves,
}
impl HoldsCounted {
    pub unsafe fn move_from(_a0: *mut HoldsCounted) -> Self {
        let mut this = Self {
            member: CountsCopiesAndMoves::move_from({ &mut (*_a0).member }),
        };
        this
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut pm: MemberPtrUnsafe<HoldsCounted, CountsCopiesAndMoves> =
        <MemberPtrUnsafe<HoldsCounted, CountsCopiesAndMoves>>::from_offset(::std::mem::offset_of!(
            HoldsCounted,
            member
        ));
    let mut holder: HoldsCounted = <HoldsCounted>::default();
    let mut from_lvalue: CountsCopiesAndMoves =
        CountsCopiesAndMoves::copy_from({ &(*(&mut holder as *mut HoldsCounted).member(pm)) });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_0)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut moveCount_1)) == (0)));
    let mut from_xvalue: CountsCopiesAndMoves =
        CountsCopiesAndMoves::move_from({ &mut (*(&mut holder as *mut HoldsCounted).member(pm)) });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_0)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut moveCount_1)) == (1)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copyCount_0);
    std::cell::LazyCell::force(&*&raw const moveCount_1);
}
