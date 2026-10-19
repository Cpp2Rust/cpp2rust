extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct Elem {
    pub id: i32,
    pub hist: [i32; 4],
    pub coeffs: Vec<i16>,
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct State {
    pub count: i32,
    pub grid: [[i32; 3]; 2],
    pub arr: Vec<i32>,
    pub elems: Vec<Elem>,
    pub rows: Vec<Vec<i32>>,
}
impl Default for State {
    fn default() -> Self {
        unsafe {
            State {
                count: 0_i32,
                grid: [[0_i32; 3]; 2],
                arr: std::array::from_fn::<_, 3_u64, _>(|_| <i32>::default()).to_vec(),
                elems: Default::default(),
                rows: Default::default(),
            }
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
pub unsafe fn fill_1(mut s: *mut State) {
    let mut i: i32 = 0;
    'loop_: while ((i) < (2)) {
        let mut j: i32 = 0;
        'loop_: while ((j) < (3)) {
            (*s).grid[(i) as usize][(j) as usize] = (((i) * (3)) + (j));
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    (&mut (*s)).arr[(1_usize)] = 7;
    {
        let __a0 = 3_usize as usize;
        (*s).elems.resize_with(__a0, || <Elem>::default())
    };
    let mut i: i32 = 0;
    'loop_: while ((i) < (3)) {
        (&mut (*s)).elems[(i as usize)].id = i;
        (&mut (*s)).elems[(i as usize)].hist[(i) as usize] = ((10) * (i));
        {
            let __a0 = 4_usize as usize;
            (&mut (*s)).elems[(i as usize)]
                .coeffs
                .resize(__a0, (i as i16))
        };
        i.prefix_inc();
    }
    {
        let __a1 = vec![1, 2, 3];
        (*s).rows.push(__a1)
    };
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: State = <State>::default();
    (unsafe { fill_1((&mut s as *mut State)) });
    assert!(
        ((s.grid[(1) as usize][(2) as usize]) == (5))
            && ((unsafe { sum_0((s.grid[(1) as usize].as_mut_ptr()).cast_const(), 3,) }) == (12))
    );
    assert!(((s.arr[(1_usize)]) == (7)));
    let mut e: *mut Elem = (&mut s.elems[(2_usize)] as *mut Elem);
    let mut h: *mut i32 = (*e).hist.as_mut_ptr();
    let mut c: *mut i16 = (&mut (&mut (*e)).coeffs[(1_usize)] as *mut i16);
    let mut id: *mut i32 = (&mut s.elems[(1_usize)].id as *mut i32);
    let mut k: i32 = 0;
    'loop_: while ((k) < (4)) {
        (*h.offset((k) as isize)) += k;
        k.prefix_inc();
    }
    (*c) = (((*c) as i32) + 5) as i16;
    (*id) = 11;
    s.count = 3;
    assert!(
        ((unsafe { sum_0((s.elems[(2_usize)].hist.as_mut_ptr()).cast_const(), 4,) }) == (26))
            && ((s.elems[(2_usize)].coeffs[(1_usize)] as i32) == (7))
    );
    assert!(
        ((s.elems[(1_usize)].id) == (11))
            && ((h.offset((3) as isize))
                == (&mut s.elems[(2_usize)].hist[(3) as usize] as *mut i32))
    );
    let mut row: *mut i32 = (&mut s.rows[(0_usize)][(0_usize)] as *mut i32);
    (*row.offset((2) as isize)) = 9;
    assert!(((s.rows[(0_usize)][(2_usize)]) == (9)));
    let mut t: State = s.clone();
    t.elems[(2_usize)].hist[(0) as usize] = 100;
    t.grid[(0) as usize][(0) as usize] = 100;
    t.rows[(0_usize)][(0_usize)] = 100;
    assert!(
        (((s.elems[(2_usize)].hist[(0) as usize]) == (0))
            && ((s.grid[(0) as usize][(0) as usize]) == (0)))
            && ((s.rows[(0_usize)][(0_usize)]) == (1))
    );
    assert!(((t.elems[(2_usize)].coeffs[(1_usize)] as i32) == (7)));
    let mut p: *mut State = (&mut t as *mut State);
    (&mut (*p)).elems[(0_usize)].hist[(3) as usize] =
        (((*p).grid[(1) as usize][(1) as usize]) + ((&mut (*p)).arr[(1_usize)]));
    assert!(((t.elems[(0_usize)].hist[(3) as usize]) == (11)));
    {
        let a0_clone = (&mut (*p)).elems[(2_usize)].clone();
        (*p).elems.push(a0_clone)
    };
    assert!(
        (((*p).elems.len()) == (4_usize))
            && (((&mut (*p)).elems[(3_usize)].hist[(0) as usize]) == (100))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
