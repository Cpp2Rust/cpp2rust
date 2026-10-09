extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut counter_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut inc_1: std::cell::LazyCell<FnPtr<fn(i32) -> i32>> =
    std::cell::LazyCell::new(|| unsafe {
        FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
            unsafe {
                return ((x) + (1));
            }
        })
    });
pub static mut bump_2: std::cell::LazyCell<FnPtr<fn() -> i32>> =
    std::cell::LazyCell::new(|| unsafe {
        FnPtr::<fn() -> i32>::new(|| -> i32 {
            unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut counter_0)).postfix_inc();
                return (*std::cell::LazyCell::force_mut(&mut *&raw mut counter_0));
            }
        })
    });
pub unsafe fn apply_3(mut f: FnPtr<fn(i32) -> i32>, mut x: i32) -> i32 {
    return (unsafe { f.call(x) });
}
pub unsafe fn by_ref_4(v: *const i32) -> i32 {
    return ((*v) * (2));
}
pub static mut twice_next_5: std::cell::LazyCell<FnPtr<fn(i32) -> i32>> =
    std::cell::LazyCell::new(|| unsafe {
        FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
            unsafe {
                return (unsafe {
                    let mut _v: i32 = ((x) + (1));
                    by_ref_4(&mut _v)
                });
            }
        })
    });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(
        ((unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut inc_1)).call(41,) }) == (42))
    );
    (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut bump_2)).call() });
    assert!(((unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut bump_2)).call() }) == (2)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut counter_0)) == (2)));
    assert!(
        ((unsafe {
            apply_3(
                (*std::cell::LazyCell::force_mut(&mut *&raw mut inc_1)).copy_from(),
                1,
            )
        }) == (2))
    );
    let mut copy: FnPtr<fn(i32) -> i32> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut inc_1)).copy_from();
    assert!(((unsafe { copy.call(9,) }) == (10)));
    let mut fp: Option<unsafe fn(i32) -> i32> = Some(|mut x: i32| -> i32 {
        unsafe {
            return ((x) + (1));
        }
    });
    assert!(((unsafe { (fp).unwrap()(-1_i32,) }) == (0)));
    assert!(
        ((unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut twice_next_5)).call(4,) })
            == (10))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const counter_0);
    std::cell::LazyCell::force(&*&raw const inc_1);
    std::cell::LazyCell::force(&*&raw const bump_2);
    std::cell::LazyCell::force(&*&raw const twice_next_5);
}
