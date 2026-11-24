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
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
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
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut ok: bool = true;
    let mut m: UnsafeMap<Key, (Key, i32)> = UnsafeMap::new();
    let mut a: Key = Key::new({ 1 });
    let mut b: Key = Key::new({ 2 });
    let mut c: Key = Key::new({ 3 });
    (*m.subscript(&a, || 0_i32)) = 10;
    match (
        { (((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (1)) as i32) },
        &mut (ok),
    ) {
        (__tmp_0, __tmp_1) => {
            (*__tmp_1) = (((*__tmp_1) as i32) & (__tmp_0 as i32)) != 0;
        }
    };
    (*m.subscript(&b, || 0_i32)) = 20;
    match (
        { (((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (2)) as i32) },
        &mut (ok),
    ) {
        (__tmp_2, __tmp_3) => {
            (*__tmp_3) = (((*__tmp_3) as i32) & (__tmp_2 as i32)) != 0;
        }
    };
    (*m.subscript(&a, || 0_i32)) = 11;
    match (
        { (((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (2)) as i32) },
        &mut (ok),
    ) {
        (__tmp_4, __tmp_5) => {
            (*__tmp_5) = (((*__tmp_5) as i32) & (__tmp_4 as i32)) != 0;
        }
    };
    match (
        { ((((*m.find(&a).iter_deref()).1) == (11)) as i32) },
        &mut (ok),
    ) {
        (__tmp_6, __tmp_7) => {
            (*__tmp_7) = (((*__tmp_7) as i32) & (__tmp_6 as i32)) != 0;
        }
    };
    match (
        { ((((*m.get(&b).expect("out of range!")).1) == (20)) as i32) },
        &mut (ok),
    ) {
        (__tmp_8, __tmp_9) => {
            (*__tmp_9) = (((*__tmp_9) as i32) & (__tmp_8 as i32)) != 0;
        }
    };
    match (
        { (((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (2)) as i32) },
        &mut (ok),
    ) {
        (__tmp_10, __tmp_11) => {
            (*__tmp_11) = (((*__tmp_11) as i32) & (__tmp_10 as i32)) != 0;
        }
    };
    (*m.subscript_move(
        &mut (unsafe { make_key_3(4) }),
        |__k| cpp_move_unsafe!(__k),
        || 0_i32,
    )) = 40;
    (*m.subscript_move(
        &mut (unsafe { make_key_3(4) }),
        |__k| cpp_move_unsafe!(__k),
        || 0_i32,
    )) = 41;
    match (
        {
            (((((*std::cell::LazyCell::force_mut(&mut *&raw mut made_2)) == (2))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)) == (1)))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (2)))
                as i32)
        },
        &mut (ok),
    ) {
        (__tmp_12, __tmp_13) => {
            (*__tmp_13) = (((*__tmp_13) as i32) & (__tmp_12 as i32)) != 0;
        }
    };
    (*m.subscript_move(&mut c, |__k| cpp_move_unsafe!(__k), || 0_i32)) = 30;
    (*m.subscript_move(&mut c, |__k| cpp_move_unsafe!(__k), || 0_i32)) = 31;
    match (
        {
            (((((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)) == (2))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (2)))
                && ((c.k) == (3))) as i32)
        },
        &mut (ok),
    ) {
        (__tmp_14, __tmp_15) => {
            (*__tmp_15) = (((*__tmp_15) as i32) & (__tmp_14 as i32)) != 0;
        }
    };
    match ({ (((m.len()) == (4_usize)) as i32) }, &mut (ok)) {
        (__tmp_16, __tmp_17) => {
            (*__tmp_17) = (((*__tmp_17) as i32) & (__tmp_16 as i32)) != 0;
        }
    };
    (*m.subscript(&(*m.begin().iter_deref()).0, || 0_i32)) += 1;
    match (
        {
            ((((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (2))
                && ((*m.subscript(&a, || 0_i32)) == (12))) as i32)
        },
        &mut (ok),
    ) {
        (__tmp_18, __tmp_19) => {
            (*__tmp_19) = (((*__tmp_19) as i32) & (__tmp_18 as i32)) != 0;
        }
    };
    let mut first: *const Key = (&(*m.begin().iter_deref()).0 as *const Key);
    (*m.subscript_move(&mut Key::new({ 0 }), |__k| cpp_move_unsafe!(__k), || 0_i32)) = 0;
    match (
        { (((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)) == (3)) as i32) },
        &mut (ok),
    ) {
        (__tmp_20, __tmp_21) => {
            (*__tmp_21) = (((*__tmp_21) as i32) & (__tmp_20 as i32)) != 0;
        }
    };
    match (
        { (((&(*m.find(&a).iter_deref()).0 as *const Key) == (first)) as i32) },
        &mut (ok),
    ) {
        (__tmp_22, __tmp_23) => {
            (*__tmp_23) = (((*__tmp_23) as i32) & (__tmp_22 as i32)) != 0;
        }
    };
    let mut n: UnsafeMap<Key, (Key, i32)> = m.clone();
    match (
        { (((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == ((2) + (5))) as i32) },
        &mut (ok),
    ) {
        (__tmp_24, __tmp_25) => {
            (*__tmp_25) = (((*__tmp_25) as i32) & (__tmp_24 as i32)) != 0;
        }
    };
    match (
        { ((((n.len()) == (5_usize)) && ((*n.subscript(&a, || 0_i32)) == (12))) as i32) },
        &mut (ok),
    ) {
        (__tmp_26, __tmp_27) => {
            (*__tmp_27) = (((*__tmp_27) as i32) & (__tmp_26 as i32)) != 0;
        }
    };
    m.erase(&m.find(&a));
    match ({ (((m.erase_key(&b)) == (1_usize)) as i32) }, &mut (ok)) {
        (__tmp_28, __tmp_29) => {
            (*__tmp_29) = (((*__tmp_29) as i32) & (__tmp_28 as i32)) != 0;
        }
    };
    match ({ (((m.erase_key(&b)) == (0_usize)) as i32) }, &mut (ok)) {
        (__tmp_30, __tmp_31) => {
            (*__tmp_31) = (((*__tmp_31) as i32) & (__tmp_30 as i32)) != 0;
        }
    };
    match (
        { (((m.erase_key(&(*m.begin().iter_deref()).0)) == (1_usize)) as i32) },
        &mut (ok),
    ) {
        (__tmp_32, __tmp_33) => {
            (*__tmp_33) = (((*__tmp_33) as i32) & (__tmp_32 as i32)) != 0;
        }
    };
    match (
        {
            ((((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (7))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)) == (3)))
                as i32)
        },
        &mut (ok),
    ) {
        (__tmp_34, __tmp_35) => {
            (*__tmp_35) = (((*__tmp_35) as i32) & (__tmp_34 as i32)) != 0;
        }
    };
    match (
        { ((((m.len()) == (2_usize)) && ((n.len()) == (5_usize))) as i32) },
        &mut (ok),
    ) {
        (__tmp_36, __tmp_37) => {
            (*__tmp_37) = (((*__tmp_37) as i32) & (__tmp_36 as i32)) != 0;
        }
    };
    let mut sum: i32 = 0;
    'loop_: for entry in IterRange::new(m.begin(), m.end()) {
        let entry: *const (Key, i32) = &(*entry.iter_deref());
        sum += (((*entry).0.k) * ((*entry).1));
    }
    match (
        { (((sum) == (((3) * (31)) + ((4) * (41)))) as i32) },
        &mut (ok),
    ) {
        (__tmp_38, __tmp_39) => {
            (*__tmp_39) = (((*__tmp_39) as i32) & (__tmp_38 as i32)) != 0;
        }
    };
    match (
        {
            ((((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (7))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)) == (3)))
                as i32)
        },
        &mut (ok),
    ) {
        (__tmp_40, __tmp_41) => {
            (*__tmp_41) = (((*__tmp_41) as i32) & (__tmp_40 as i32)) != 0;
        }
    };
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
