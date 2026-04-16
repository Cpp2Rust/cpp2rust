extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(Default)]
pub struct SafePointer {
    pub ptr: Option<Box<i32>>,
}
impl SafePointer {
    pub unsafe fn inc(&mut self) {
        (*self.ptr.as_deref_mut().unwrap()).prefix_inc();
    }
}
#[derive(Copy, Clone, Default)]
pub struct Pair {
    pub x: i32,
    pub y: i32,
}
impl Pair {
    pub unsafe fn inc(&mut self, mut k: i32) {
        self.x += k;
        self.y += k;
    }
}
pub unsafe fn DoStuffWithSafePointer_0(safe_ptr: *mut Option<Box<SafePointer>>) {
    let mut x1: Option<Box<i32>> = Some(Box::new(0));
    let mut x2: Option<Box<i32>> = Some(Box::new(0));
    (*x2.as_deref_mut().unwrap()) = 1;
    x1 = x2;
    let mut raw_ptr1: *mut i32 = (&mut (*x1.as_deref_mut().unwrap()) as *mut i32);
    (*raw_ptr1).prefix_inc();
    (*(*safe_ptr).as_deref_mut().unwrap()).ptr = x1;
    (unsafe { (*(*safe_ptr).as_deref_mut().unwrap()).inc() });
    (unsafe { (*(*safe_ptr).as_deref_mut().unwrap()).inc() });
    let mut x3: Option<Box<i32>> = Some(Box::new(10));
    let mut x4: Option<Box<i32>> = Some(Box::new(20));
    (*x3.as_deref_mut().unwrap()) = ((*x3.as_deref_mut().unwrap()) + (*x4.as_deref_mut().unwrap()));
    x4 = x3;
    let mut raw_ptr2: *mut i32 = (&mut (*x4.as_deref_mut().unwrap()) as *mut i32);
    (*raw_ptr2) += 1;
    let mut pair: Option<Box<Pair>> = Some(Box::new(Pair {
        x: (*raw_ptr2),
        y: 5,
    }));
    (unsafe {
        let _k: i32 = 10;
        (*pair.as_deref_mut().unwrap()).inc(_k)
    });
    (*(*(*safe_ptr).as_deref_mut().unwrap())
        .ptr
        .as_deref_mut()
        .unwrap()) = (((*(*(*safe_ptr).as_deref_mut().unwrap())
        .ptr
        .as_deref_mut()
        .unwrap())
        + ((*pair.as_deref_mut().unwrap()).x))
        + ((*pair.as_deref_mut().unwrap()).y));
}
pub unsafe fn Consume_1(mut safe_ptr: Option<Box<SafePointer>>) -> i32 {
    let mut x: Option<Box<SafePointer>> = safe_ptr;
    let mut p: Option<Box<Pair>> = Some(Box::from_raw(
        (Box::leak(Box::new(<Pair>::default())) as *mut Pair),
    ));
    return ((*(*x.as_deref_mut().unwrap()).ptr.as_deref_mut().unwrap())
        + ((*p.as_deref_mut().unwrap()).x));
}
pub unsafe fn RndStuff_2() {
    let mut x1: Option<Box<[i32]>> = None;
    let mut x2: Option<Box<[i32]>> = Some(Box::from_raw(Box::leak(
        (0..100_u64).map(|_| 0_i32).collect::<Box<[i32]>>(),
    )));
    let mut i: i32 = 0;
    'loop_: while ((i) < (100)) {
        x2.as_mut().unwrap()[(i as u64) as usize] = 1;
        i.prefix_inc();
    }
    x2 = Some(Box::from_raw(Box::leak(
        (0..200_u64).map(|_| 0_i32).collect::<Box<[i32]>>(),
    )));
    let mut i: i32 = 0;
    'loop_: while ((i) < (200)) {
        x2.as_mut().unwrap()[(i as u64) as usize] = 2;
        i.prefix_inc();
    }
    let mut p2: *mut i32 = x2
        .as_deref_mut()
        .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr());
    let mut i: i32 = 0;
    'loop_: while ((i) < (200)) {
        assert!(((*p2.offset((i) as isize)) == (2)));
        i.prefix_inc();
    }
    let mut x3: Option<Box<[Pair]>> =
        Some((0..10_u64).map(|_| <Pair>::default()).collect::<Box<[_]>>());
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        x3.as_mut().unwrap()[(i as u64) as usize] = Pair { x: 1, y: 2 };
        i.prefix_inc();
    }
    let mut p3_0: *mut Pair = x3
        .as_deref_mut()
        .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr());
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        assert!((((*p3_0.offset((i) as isize)).x) == (1)));
        assert!((((*p3_0.offset((i) as isize)).y) == (2)));
        (unsafe {
            let _k: i32 = 10;
            x3.as_mut().unwrap()[(i as u64) as usize].inc(_k)
        });
        assert!((((*p3_0.offset((i) as isize)).x) == (11)));
        assert!((((*p3_0.offset((i) as isize)).y) == (12)));
        i.prefix_inc();
    }
    x3 = Some(Box::from_raw(Box::leak(
        (0..50_u64)
            .map(|_| <Pair>::default())
            .collect::<Box<[Pair]>>(),
    )));
    let mut i: i32 = 0;
    'loop_: while ((i) < (50)) {
        x3.as_mut().unwrap()[(i as u64) as usize] = Pair {
            x: -1_i32,
            y: -2_i32,
        };
        i.prefix_inc();
    }
    let mut p3_1: *mut Pair = x3
        .as_deref_mut()
        .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr());
    assert!(((p3_0) != (p3_1)));
    let mut i: i32 = 0;
    'loop_: while ((i) < (50)) {
        assert!((((*p3_1.offset((i) as isize)).x) == (-1_i32)));
        assert!((((*p3_1.offset((i) as isize)).y) == (-2_i32)));
        (unsafe {
            let _k: i32 = -10_i32;
            x3.as_mut().unwrap()[(i as u64) as usize].inc(_k)
        });
        assert!((((*p3_1.offset((i) as isize)).x) == (-11_i32)));
        assert!((((*p3_1.offset((i) as isize)).y) == (-12_i32)));
        i.prefix_inc();
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: Option<Box<i32>> = Some(Box::new(0));
    let mut safe_ptr: Option<Box<SafePointer>> = Some(Box::new(SafePointer { ptr: x }));
    (unsafe {
        let _safe_ptr: *mut Option<Box<SafePointer>> =
            &mut safe_ptr as *mut Option<Box<SafePointer>>;
        DoStuffWithSafePointer_0(_safe_ptr)
    });
    return (unsafe {
        let _safe_ptr: Option<Box<SafePointer>> = safe_ptr;
        Consume_1(_safe_ptr)
    });
}
