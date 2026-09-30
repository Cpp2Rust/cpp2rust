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
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub unsafe fn sum(&self) -> i32 {
        return ((self.x) + (self.y));
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct Holder {
    pub values: Vec<i32>,
    pub points: Vec<Point>,
}
pub unsafe fn push_and_index_0(v: *mut Vec<i32>) -> i32 {
    (*v).push(42);
    return (((*v).len() as i32) - (1));
}
pub unsafe fn sum_ref_1(v: *const Vec<i32>) -> i32 {
    let mut s: i32 = 0;
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*v).len())) {
        s += (&(*v))[(i)];
        i.prefix_inc();
    }
    return s;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v: Vec<i32> = vec![1, 2, 3];
    v[(0_usize)] = 10;
    assert!(((v[(0_usize)]) == (10)));
    v[(1_usize)] += 5;
    v[(2_usize)].postfix_inc();
    assert!(((v[(1_usize)]) == (7)) && ((v[(2_usize)]) == (4)));
    v[(0_usize)] = v[(1_usize)];
    assert!(((v[(0_usize)]) == (7)));
    v[(1_usize)] = 0;
    assert!(((v[(v[(1_usize)] as usize)]) == (7)));
    let mut i: i32 = 0;
    v[(i.postfix_inc() as usize)] = 3;
    assert!(((i) == (1)) && ((v[(0_usize)]) == (3)));
    assert!(((v[((unsafe { push_and_index_0(&mut v,) }) as usize)]) == (42)));
    v[((unsafe { push_and_index_0(&mut v) }) as usize)] = 5;
    assert!(((v.len()) == (5_usize)) && ((v[(4_usize)]) == (5)));
    let mut p: *mut i32 = (&mut v[(2_usize)] as *mut i32);
    (*p) = 9;
    assert!(((v[(2_usize)]) == (9)));
    assert!(((unsafe { sum_ref_1(&v,) }) == (((((3) + (0)) + (9)) + (42)) + (5))));
    let mut h: Holder = <Holder>::default();
    {
        let __a0 = 2_usize as usize;
        h.values.resize_with(__a0, || <i32>::default())
    };
    h.values[(1_usize)] = 6;
    h.points.push(Point { x: 1, y: 2 });
    h.points[(0_usize)].y = 5;
    assert!(((h.values[(1_usize)]) == (6)));
    assert!(((unsafe { Point::sum(&h.points[(0_usize)],) }) == (6)));
    let mut hp: *mut Holder = (&mut h as *mut Holder);
    (&mut (*hp)).values[(0_usize)] = (((&mut (*hp)).values[(1_usize)]) + (1));
    (&mut (*hp)).points[(0_usize)].x = (&mut (*hp)).values[(0_usize)];
    assert!(((h.values[(0_usize)]) == (7)) && ((h.points[(0_usize)].x) == (7)));
    let mut q: Point = h.points[(0_usize)];
    q.x = 0;
    assert!(((h.points[(0_usize)].x) == (7)));
    let mut grid: Vec<Vec<i32>> = vec![vec![0; 3_usize as usize]; 2_usize as usize];
    grid[(1_usize)][(2_usize)] = 8;
    assert!(((grid[(1_usize)][(2_usize)]) == (8)) && ((grid[(0_usize)][(2_usize)]) == (0)));
    let mut a: Vec<i32> = vec![4, 5, 6];
    a[(1_usize)] = ((a[(0_usize)]) + (a[(2_usize)]));
    assert!(((a[(1_usize)]) == (10)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
