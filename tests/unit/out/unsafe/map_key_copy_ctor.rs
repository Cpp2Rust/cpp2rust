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
pub static mut made_2: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct Key {
    pub k: i32,
}
impl Key {
    pub unsafe fn new(mut k: i32) -> Self {
        let mut this = Self { k: k };
        this
    }
    pub unsafe fn copy_from(o: *const Key) -> Self {
        let mut this = Self { k: (*o).k };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
        this
    }
    pub unsafe fn move_from(o: *mut Key) -> Self {
        let mut this = Self { k: (*o).k };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)).prefix_inc();
        this
    }
    pub unsafe fn operator_lt(&self, o: *const Key) -> bool {
        return ((self.k) < ((*o).k));
    }
}
impl std::cmp::Ord for Key {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if Key::operator_lt(self, other as *const Key) {
                std::cmp::Ordering::Less
            } else if Key::operator_lt(other, self as *const Key) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(Key::operator_lt(self, other as *const Key))
                && !(Key::operator_lt(other, self as *const Key))
        }
    }
}
impl std::cmp::Eq for Key {}
impl Clone for Key {
    fn clone(&self) -> Self {
        unsafe { Key::copy_from(self as *const Key) }
    }
}
pub unsafe fn make_key_3(mut k: i32) -> Key {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut made_2)).prefix_inc();
    return Key::new({ k });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut ok: bool = true;
    let mut m: UnsafeMap<Key, (Key, i32)> = UnsafeMap::new();
    let mut a: Key = Key::new({ 1 });
    let mut b: Key = Key::new({ 2 });
    let mut c: Key = Key::new({ 3 });
    (*m.subscript(&a)) = 10;
    ok = ((ok as i32)
        & (((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (1)) as i32))
        != 0;
    (*m.subscript(&b)) = 20;
    ok = ((ok as i32)
        & (((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (2)) as i32))
        != 0;
    (*m.subscript(&a)) = 11;
    ok = ((ok as i32)
        & (((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (2)) as i32))
        != 0;
    ok = ((ok as i32) & ((((*m.find(&a).iter_deref()).1) == (11)) as i32)) != 0;
    ok = ((ok as i32) & ((((*m.get(&b).expect("out of range!")).1) == (20)) as i32)) != 0;
    ok = ((ok as i32)
        & (((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (2)) as i32))
        != 0;
    (*m.subscript_move(&mut (unsafe { make_key_3(4) }), |__k| cpp_move_unsafe!(__k))) = 40;
    (*m.subscript_move(&mut (unsafe { make_key_3(4) }), |__k| cpp_move_unsafe!(__k))) = 41;
    ok = ((ok as i32)
        & (((((*std::cell::LazyCell::force_mut(&mut *&raw mut made_2)) == (2))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)) == (1)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (2)))
            as i32))
        != 0;
    (*m.subscript_move(&mut c, |__k| cpp_move_unsafe!(__k))) = 30;
    (*m.subscript_move(&mut c, |__k| cpp_move_unsafe!(__k))) = 31;
    ok = ((ok as i32)
        & (((((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)) == (2))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (2)))
            && ((c.k) == (3))) as i32))
        != 0;
    ok = ((ok as i32) & (((m.len()) == (4_usize)) as i32)) != 0;
    (*m.subscript(&(*m.begin().iter_deref()).0)) += 1;
    ok = ((ok as i32)
        & ((((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (2))
            && ((*m.subscript(&a)) == (12))) as i32))
        != 0;
    let mut first: *const Key = (&(*m.begin().iter_deref()).0 as *const Key);
    (*m.subscript_move(&mut Key::new({ 0 }), |__k| cpp_move_unsafe!(__k))) = 0;
    ok = ((ok as i32)
        & (((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)) == (3)) as i32))
        != 0;
    ok = ((ok as i32) & (((&(*m.find(&a).iter_deref()).0 as *const Key) == (first)) as i32)) != 0;
    let mut n: UnsafeMap<Key, (Key, i32)> = m.clone();
    ok = ((ok as i32)
        & (((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == ((2) + (5))) as i32))
        != 0;
    ok = ((ok as i32) & ((((n.len()) == (5_usize)) && ((*n.subscript(&a)) == (12))) as i32)) != 0;
    m.erase(&m.find(&a));
    ok = ((ok as i32) & (((m.erase_key(&b)) == (1_usize)) as i32)) != 0;
    ok = ((ok as i32) & (((m.erase_key(&b)) == (0_usize)) as i32)) != 0;
    ok = ((ok as i32) & (((m.erase_key(&(*m.begin().iter_deref()).0)) == (1_usize)) as i32)) != 0;
    ok = ((ok as i32)
        & ((((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (7))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)) == (3)))
            as i32))
        != 0;
    ok = ((ok as i32) & ((((m.len()) == (2_usize)) && ((n.len()) == (5_usize))) as i32)) != 0;
    let mut sum: i32 = 0;
    'loop_: for entry in IterRange::new(m.begin(), m.end()) {
        let entry: *const (Key, i32) = &(*entry.iter_deref());
        sum += (((*entry).0.k) * ((*entry).1));
    }
    ok = ((ok as i32) & (((sum) == (((3) * (31)) + ((4) * (41)))) as i32)) != 0;
    ok = ((ok as i32)
        & ((((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (7))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)) == (3)))
            as i32))
        != 0;
    (unsafe {
        libc::printf(
            c"%d %d %d\n".as_ptr() as *const libc::c_char,
            (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut made_2)),
        )
    });
    return if ok { 0 } else { 1 };
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copies_0);
    std::cell::LazyCell::force(&*&raw const moves_1);
    std::cell::LazyCell::force(&*&raw const made_2);
}
