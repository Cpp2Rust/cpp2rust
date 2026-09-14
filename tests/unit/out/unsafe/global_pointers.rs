extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Entry {
    pub name: *const libc::c_char,
    pub p: *mut i32,
}
pub static mut single_entry_0: std::cell::LazyCell<Entry> = std::cell::LazyCell::new(|| unsafe {
    Entry {
        name: c"alone".as_ptr(),
        p: std::ptr::null_mut(),
    }
});
pub static mut entries_1: std::cell::LazyCell<[Entry; 2]> = std::cell::LazyCell::new(|| unsafe {
    [
        Entry {
            name: c"first".as_ptr(),
            p: std::ptr::null_mut(),
        },
        Entry {
            name: c"second".as_ptr(),
            p: std::ptr::null_mut(),
        },
    ]
});
pub static mut arr_of_pointers_2: std::cell::LazyCell<[*mut libc::c_char; 3]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        ]
    });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut single_entry_0)).p).is_null());
    let mut i: i32 = 0;
    'loop_: while ((i) < (2)) {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut entries_1))[(i) as usize].p).is_null()
        );
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut arr_of_pointers_2))[(i) as usize])
                .is_null()
        );
        i.prefix_inc();
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const single_entry_0);
    std::cell::LazyCell::force(&*&raw const entries_1);
    std::cell::LazyCell::force(&*&raw const arr_of_pointers_2);
}
