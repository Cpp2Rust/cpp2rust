extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub head: i32,
    pub tail: [i32; 3],
    pub buf: [libc::c_char; 4],
}
pub static mut s_0: std::cell::LazyCell<S> = std::cell::LazyCell::new(|| unsafe {
    S {
        head: 5,
        tail: [0; 3],
        buf: [0; 4],
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct P {
    pub x: i32,
    pub y: i32,
}
pub static mut g_1: std::cell::LazyCell<[i32; 5]> =
    std::cell::LazyCell::new(|| unsafe { [0_i32; 5] });
pub static mut k_2: std::cell::LazyCell<[i32; 8]> =
    std::cell::LazyCell::new(|| unsafe { [3, 0, 4, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32] });
pub static mut counter_3: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub unsafe fn next_counter_4() -> i32 {
    return (*(*std::cell::LazyCell::force_mut(&mut *&raw mut counter_3)).prefix_inc());
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!((((*std::cell::LazyCell::force_mut(&mut *&raw mut s_0)).head) == (5)));
    {
        let mut i: i32 = 0;
        'loop_: while ((i) < (3)) {
            'for_body: {
                assert!(
                    (((*std::cell::LazyCell::force_mut(&mut *&raw mut s_0)).tail[(i) as usize])
                        == (0))
                );
            }
            i.postfix_inc();
        }
    }
    {
        let mut i: i32 = 0;
        'loop_: while ((i) < (4)) {
            'for_body: {
                assert!(
                    (((*std::cell::LazyCell::force_mut(&mut *&raw mut s_0)).buf[(i) as usize]
                        as i32)
                        == (0))
                );
            }
            i.postfix_inc();
        }
    }
    let mut a: [i32; 10] = [0_i32; 10];
    {
        let mut i: i32 = 0;
        'loop_: while ((i) < (10)) {
            'for_body: {
                assert!(((a[(i) as usize]) == (0)));
            }
            i.postfix_inc();
        }
    }
    let mut seven: [i32; 4] = [7, 0_i32, 0_i32, 0_i32];
    assert!(((seven[(0) as usize]) == (7)));
    {
        let mut i: i32 = 1;
        'loop_: while ((i) < (4)) {
            'for_body: {
                assert!(((seven[(i) as usize]) == (0)));
            }
            i.postfix_inc();
        }
    }
    let mut b: [i32; 6] = [1, 0, 2, 0_i32, 0_i32, 0_i32];
    assert!(
        ((((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (0))) && ((b[(2) as usize]) == (2)))
    );
    {
        let mut i: i32 = 3;
        'loop_: while ((i) < (6)) {
            'for_body: {
                assert!(((b[(i) as usize]) == (0)));
            }
            i.postfix_inc();
        }
    }
    let mut d: [f64; 4] = [-0_f64, 0_f64, 0_f64, 0_f64];
    assert!((((d[(0) as usize]) == (0_f64)) && ((d[(3) as usize]) == (0_f64))));
    let mut p: [P; 4] = [
        P { x: 1, y: 2 },
        <P>::default(),
        <P>::default(),
        <P>::default(),
    ];
    assert!((((p[(0) as usize].x) == (1)) && ((p[(0) as usize].y) == (2))));
    assert!((((p[(3) as usize].x) == (0)) && ((p[(3) as usize].y) == (0))));
    let mut c: [i32; 5] = [
        (unsafe { next_counter_4() }),
        (unsafe { next_counter_4() }),
        0_i32,
        0_i32,
        0_i32,
    ];
    assert!(((((c[(0) as usize]) + (c[(1) as usize])) == (3)) && ((c[(4) as usize]) == (0))));
    let mut t: [i32; 2] = [0_i32; 2];
    assert!((((t[(0) as usize]) == (0)) && ((t[(1) as usize]) == (0))));
    let mut u: [i32; 3] = [5, 6, 0_i32];
    assert!(
        ((((u[(0) as usize]) == (5)) && ((u[(1) as usize]) == (6))) && ((u[(2) as usize]) == (0)))
    );
    assert!((((*std::cell::LazyCell::force_mut(&mut *&raw mut g_1))[(4) as usize]) == (0)));
    assert!(
        ((((((*std::cell::LazyCell::force_mut(&mut *&raw mut k_2))[(0) as usize]) == (3))
            && (((*std::cell::LazyCell::force_mut(&mut *&raw mut k_2))[(1) as usize]) == (0)))
            && (((*std::cell::LazyCell::force_mut(&mut *&raw mut k_2))[(2) as usize]) == (4)))
            && (((*std::cell::LazyCell::force_mut(&mut *&raw mut k_2))[(7) as usize]) == (0)))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const s_0);
    std::cell::LazyCell::force(&*&raw const g_1);
    std::cell::LazyCell::force(&*&raw const k_2);
    std::cell::LazyCell::force(&*&raw const counter_3);
}
