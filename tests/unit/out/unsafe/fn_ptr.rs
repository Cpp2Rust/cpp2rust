extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn my_foo_0(mut p: *mut ::libc::c_void) -> i32 {
    return (*(p as *mut i32));
}
pub unsafe fn foo_1(
    mut fn_: Option<unsafe fn(*mut ::libc::c_void) -> i32>,
    mut pi: *mut i32,
) -> i32 {
    return (unsafe { (fn_).unwrap()((pi as *mut ::libc::c_void)) });
}
pub unsafe fn twice_2(mut x: u64) -> u64 {
    return (x).wrapping_mul(2_u64);
}
pub unsafe fn twice_in_place_3(x: *mut u64) -> u64 {
    (*x) = (*x).wrapping_mul(2_u64);
    return (*x);
}
pub unsafe fn ret_size_4(mut v: i32) -> usize {
    return (((v) + (1)) as usize);
}
pub unsafe fn call_fn_5(mut f: Option<unsafe fn(i32) -> usize>, mut v: i32) -> usize {
    return (unsafe { (f).unwrap()(v) }).wrapping_mul(2_usize);
}
pub unsafe fn identity_hash_6(mut v: bool) -> usize {
    return (v as usize);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(Option<unsafe fn(bool ,) -> u64 > , new, ptr)]
pub struct HashHolder_unsigned_long__ptr__bool__ {
    pub h: Option<unsafe fn(bool) -> u64>,
}
impl HashHolder_unsigned_long__ptr__bool__ {
    pub unsafe fn new(h: *const Option<unsafe fn(bool) -> u64>) -> Self {
        let mut this = Self { h: (*h) };
        this
    }
}
pub static mut total_7: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub unsafe fn add_to_total_8(mut x: i32) {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut total_7)) += x;
}
pub unsafe fn pair_sum_9(mut p: (i32, i32)) -> i32 {
    return ((p.0) + (p.1));
}
pub unsafe fn string_size_10(mut s: Vec<libc::c_char>) -> usize {
    return (s.len() - 1);
}
pub unsafe fn vector_sum_11(mut v: Vec<i32>) -> i32 {
    return ((v[(0_usize)]) + (v[(1_usize)]));
}
pub unsafe fn is_less_12(mut o: std::cmp::Ordering) -> bool {
    return o == std::cmp::Ordering::Less;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut fn_: Option<unsafe fn(*mut ::libc::c_void) -> i32> = None;
    assert!((fn_).is_none());
    assert!(((fn_) != (Some(my_foo_0))));
    fn_ = Some(my_foo_0);
    assert!(!((fn_).is_none()));
    assert!(((fn_) == (Some(my_foo_0))));
    let mut a: i32 = 10;
    assert!(((unsafe { foo_1(fn_, (&mut a as *mut i32),) }) == (a)));
    let mut ul_fn: Option<unsafe fn(u64) -> u64> = (Some(twice_2));
    let mut n: usize = 21_usize;
    let mut r: usize = ((unsafe { (ul_fn).unwrap()((n as u64)) }) as usize);
    assert!(((r) == (42_usize)));
    let mut ul_ref_fn: Option<unsafe fn(*mut u64) -> u64> = (Some(twice_in_place_3));
    let mut m: usize = 21_usize;
    let mut q: usize =
        ((unsafe { (ul_ref_fn).unwrap()(&mut *(&raw mut m).cast::<u64>()) }) as usize);
    assert!(((q) == (42_usize)));
    assert!(((m) == (42_usize)));
    assert!(((unsafe { call_fn_5(Some(ret_size_4), 3,) }) == (8_usize)));
    let mut hh: HashHolder_unsigned_long__ptr__bool__ = {
        let mut __tmp_0: Option<unsafe fn(bool) -> u64> = std::mem::transmute::<
            Option<unsafe fn(bool) -> usize>,
            Option<unsafe fn(bool) -> u64>,
        >(Some(identity_hash_6));
        HashHolder_unsigned_long__ptr__bool__::new({ &mut __tmp_0 })
    };
    assert!(((unsafe { (hh.h).unwrap()(true,) }) == (1_u64)));
    let mut handler: Option<unsafe fn(i32)> = Some(add_to_total_8);
    (unsafe { (handler).unwrap()(2) });
    (unsafe { (handler).unwrap()(3) });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut total_7)) == (5)));
    let mut pf: Option<unsafe fn((i32, i32)) -> i32> = Some(pair_sum_9);
    assert!(((unsafe { (pf).unwrap()((1, 2),) }) == (3)));
    let mut sf: Option<unsafe fn(Vec<libc::c_char>) -> usize> = Some(string_size_10);
    assert!(
        ((unsafe {
            (sf).unwrap()(
                std::slice::from_raw_parts(
                    c"abc".as_ptr(),
                    (0..).take_while(|&i| *c"abc".as_ptr().add(i) != 0).count() + 1,
                )
                .to_vec(),
            )
        }) == (3_usize))
    );
    let mut vf: Option<unsafe fn(Vec<i32>) -> i32> = Some(vector_sum_11);
    assert!(((unsafe { (vf).unwrap()(Vec::from([1, 2,]),) }) == (3)));
    let mut of: Option<unsafe fn(std::cmp::Ordering) -> bool> = Some(is_less_12);
    assert!((unsafe { (of).unwrap()(std::cmp::Ord::cmp(&(1), &(2)),) }));
    assert!(!(unsafe { (of).unwrap()(std::cmp::Ord::cmp(&(2), &(1)),) }));
    let mut o: std::cmp::Ordering = std::cmp::Ord::cmp(&(1), &(2));
    let mut op: *mut std::cmp::Ordering = (&mut o as *mut std::cmp::Ordering);
    assert!((*op) == std::cmp::Ordering::Less);
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const total_7);
}
