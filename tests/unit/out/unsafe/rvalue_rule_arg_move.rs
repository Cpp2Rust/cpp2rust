extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut copies_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut moves_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe, Default)]
pub struct Elem {
    pub v: i32,
}
impl Elem {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn copy_from(o: *const Elem) -> Self {
        let mut this = Self { v: (*o).v };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
        this
    }
    pub unsafe fn move_from(o: *mut Elem) -> Self {
        let mut this = Self { v: (*o).v };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)).prefix_inc();
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const Elem) -> *mut Elem {
        self.v = (*_a0).v;
        return &mut (*(self as *mut Elem));
    }
}
impl Clone for Elem {
    fn clone(&self) -> Self {
        unsafe { Elem::copy_from(self as *const Elem) }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v: Vec<Elem> = Vec::new();
    if 4_usize as usize > v.capacity() as usize {
        let len_0 = v.len();
        v.reserve_exact(4_usize as usize - len_0 as usize);
    };
    let mut a: Elem = Elem::new({ 1 });
    let mut b: Elem = Elem::new({ 2 });
    {
        let __it = <UnsafeRandomAccessIterator<Elem>>::from(v.end());
        let pos = __it.iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr())) as usize;
        v.insert(pos, Elem::move_from({ &mut a }));
        __it
    };
    {
        let __it = <UnsafeRandomAccessIterator<Elem>>::from(v.end());
        let pos = __it.iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr())) as usize;
        v.insert(pos, (b).clone());
        __it
    };
    let mut ok: bool = ((((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (1))
        && ((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)) == (1)))
        && ((v[(0_usize)].v) == (1)))
        && ((v[(1_usize)].v) == (2));
    let mut pa: *mut Elem = (&mut a as *mut Elem);
    (unsafe {
        libc::printf(
            c"%d %d %d\n".as_ptr() as *const libc::c_char,
            (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)),
            ((*pa).v),
        )
    });
    return if ok { 0 } else { 1 };
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copies_0);
    std::cell::LazyCell::force(&*&raw const moves_1);
}
