extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() {
    let mut level: i32 = 5;
    {
        let mut saved: i32 = level;
        let mut level: i32 = ((saved) + (1));
        let mut next: i32 = ((level) + (1));
        assert!(((saved) == (5)));
        assert!(((level) == (6)));
        assert!(((next) == (7)));
    }
    assert!(((level) == (5)));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p1_own_address_in_initializer_SelfLinked {
    pub next: *mut p1_own_address_in_initializer_SelfLinked,
    pub value: i32,
}
pub unsafe fn test_1() {
    let mut node: p1_own_address_in_initializer_SelfLinked =
        p1_own_address_in_initializer_SelfLinked {
            next: std::ptr::null_mut(),
            value: 3,
        };
    let mut outer: *mut p1_own_address_in_initializer_SelfLinked =
        (&mut node as *mut p1_own_address_in_initializer_SelfLinked);
    {
        let mut node: p1_own_address_in_initializer_SelfLinked =
            <p1_own_address_in_initializer_SelfLinked>::default();
        node = p1_own_address_in_initializer_SelfLinked {
            next: (&mut node as *mut p1_own_address_in_initializer_SelfLinked),
            value: 7,
        };
        assert!(((node.next) == (&mut node as *mut p1_own_address_in_initializer_SelfLinked)));
        assert!(((node.next) != (outer)));
        assert!((((*node.next).value) == (7)));
    }
    assert!((node.next).is_null());
    assert!(((node.value) == (3)));
}
pub unsafe fn test_2() {
    let length: i32 = 2;
    {
        let mut length: [i32; 2] = [7, 8];
        assert!(
            (((::std::mem::size_of::<[i32; 2]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                == (2_usize))
        );
        assert!(((length[(0) as usize]) == (7)));
        assert!(((length[(1) as usize]) == (8)));
    }
    assert!(((length) == (2)));
}
pub unsafe fn test_3() {
    let limit: i32 = 12;
    {
        assert!(((anon_4_limit as i32) == (13)));
        assert!(((anon_4_above as i32) == (14)));
    }
    assert!(((limit) == (12)));
}
pub type anon_4 = u32;
pub const anon_4_limit: anon_4 = 13;
pub const anon_4_above: anon_4 = 14;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p9_func_declared_before_function_body_RecordsConstructorName {
    pub name: *const libc::c_char,
}
impl p9_func_declared_before_function_body_RecordsConstructorName {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            name: c"RecordsConstructorName".as_ptr(),
        };
        assert!(((this.name) == (c"RecordsConstructorName".as_ptr())));
        this
    }
}
impl Default for p9_func_declared_before_function_body_RecordsConstructorName {
    fn default() -> Self {
        unsafe { p9_func_declared_before_function_body_RecordsConstructorName::new() }
    }
}
pub unsafe fn plain_function_5() -> *const libc::c_char {
    return c"plain_function".as_ptr();
}
pub unsafe fn test_6() {
    assert!(((libc::strcmp((unsafe { plain_function_5() }), c"plain_function".as_ptr())) == (0)));
    assert!(((unsafe { plain_function_5() }) == (unsafe { plain_function_5() })));
    let mut object: p9_func_declared_before_function_body_RecordsConstructorName =
        p9_func_declared_before_function_body_RecordsConstructorName::new();
    assert!(((libc::strcmp(object.name, c"RecordsConstructorName".as_ptr())) == (0)));
    let mut enclosing: *const libc::c_char = c"test".as_ptr();
    let mut lambda: FnPtr<fn() -> *const libc::c_char> =
        FnPtr::<fn() -> *const libc::c_char>::new(|| -> *const libc::c_char {
            unsafe {
                return c"operator()".as_ptr();
            }
        });
    assert!(((unsafe { lambda.call() }) != (enclosing)));
    assert!(((libc::strcmp(enclosing, c"test".as_ptr())) == (0)));
}
pub unsafe fn test_7() {
    let mut values: [i32; 3] = [2, 5, 9];
    let mut steps: i32 = 0;
    let mut sum: i32 = 0;
    'loop_: for values in IterRange::new(
        values.as_mut_ptr(),
        values.as_mut_ptr().offset((3_i64) as isize),
    ) {
        let mut values: i32 = (*values);
        values *= 2;
        steps.prefix_inc();
        sum += values;
    }
    assert!(((steps) == (3)));
    assert!(((sum) == (32)));
    assert!(((values[(0) as usize]) == (2)));
    assert!(((values[(1) as usize]) == (5)));
    assert!(((values[(2) as usize]) == (9)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_0() });
    (unsafe { test_1() });
    (unsafe { test_2() });
    (unsafe { test_3() });
    (unsafe { test_6() });
    (unsafe { test_7() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
