extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg)]
pub struct Shape {
    pub id: i32,
    pub coords: [i32; 4],
    pub points: [Point; 3],
    pub tail: i32,
}
impl Default for Shape {
    fn default() -> Self {
        Shape {
            id: 0_i32,
            coords: [0_i32; 4],
            points: [<Point>::default(); 3],
            tail: 0_i32,
        }
    }
}
pub unsafe fn sum_0(mut p: *const i32, mut n: i32) -> i32 {
    let mut s: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        s += (*p.offset((i) as isize));
        i.prefix_inc();
    }
    return s;
}
pub unsafe fn set_y_1(mut p: *mut Point, mut y: i32) {
    (*p).y = y;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: Shape = Shape {
        id: 1,
        coords: [10, 20, 30, 40],
        points: [
            Point { x: 1, y: 2 },
            Point { x: 3, y: 4 },
            Point { x: 5, y: 6 },
        ],
        tail: 99,
    };
    let mut c: *mut i32 = (&mut s.coords[(1) as usize] as *mut i32);
    assert!(((*c) == (20)));
    (*c) = 21;
    assert!(((s.coords[(1) as usize]) == (21)));
    c = (c).wrapping_add((2 as i32) as usize);
    assert!(((*c) == (40)));
    assert!(
        ((((c as usize - (&mut s.coords[(0) as usize] as *mut i32) as usize)
            / ::std::mem::size_of::<i32>()) as i64)
            == (3_i64))
    );
    assert!(((*c.offset((-1_i32) as isize)) == (30)));
    assert!(((unsafe { sum_0((s.coords.as_mut_ptr()).cast_const(), 4,) }) == (101)));
    assert!(
        ((unsafe { sum_0((&mut s.coords[(2) as usize] as *mut i32).cast_const(), 2,) }) == (70))
    );
    let mut p: *mut Point = (&mut s.points[(1) as usize] as *mut Point);
    assert!((((*p).x) == (3)));
    (unsafe { set_y_1(p.offset((1) as isize), 60) });
    assert!(((s.points[(2) as usize].y) == (60)));
    let mut py: *mut i32 = (&mut s.points[(0) as usize].y as *mut i32);
    (*py) = 7;
    assert!(((s.points[(0) as usize].y) == (7)));
    let mut px: *mut i32 = (&mut (*(p.offset((1) as isize))).x as *mut i32);
    (*px) += 50;
    assert!(((s.points[(2) as usize].x) == (55)));
    let mut sp: *mut Shape = (&mut s as *mut Shape);
    let mut d: *mut i32 = (*sp).coords.as_mut_ptr().offset((3) as isize);
    (*d) = 41;
    assert!(((s.coords[(3) as usize]) == (41)));
    (*sp).points[(1) as usize].y = (((*sp).coords[(0) as usize]) + ((*sp).points[(0) as usize].x));
    assert!(((s.points[(1) as usize].y) == (11)));
    let mut q: *mut Point = (*sp).points.as_mut_ptr();
    (*q.offset((2) as isize)).x = 8;
    assert!((((*sp).points[(2) as usize].x) == (8)));
    let mut t: Shape = s;
    t.coords[(0) as usize] = 0;
    t.points[(0) as usize].x = 0;
    assert!(((s.coords[(0) as usize]) == (10)) && ((s.points[(0) as usize].x) == (1)));
    assert!(
        (((t.coords[(1) as usize]) == (21)) && ((t.points[(2) as usize].y) == (60)))
            && ((t.tail) == (99))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
