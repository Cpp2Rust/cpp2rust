extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Elem {
    pub id: i32,
    pub hist: [i32; 4],
    pub coeffs: Vec<i16>,
}
impl Elem {
    pub unsafe fn move_from(_a0: *mut Elem) -> Self {
        let mut this = Self {
            id: (*_a0).id,
            hist: std::array::from_fn::<_, 4, _>(|__i: usize| (*_a0).hist[(__i)]),
            coeffs: std::mem::take(&mut (*_a0).coeffs),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Elem) -> *mut Elem {
        self.id = (*_a0).id;
        match (
            { ((&mut self.hist as *mut [i32; 4]) as *mut ::libc::c_void) },
            { ((&mut (*_a0).hist as *mut [i32; 4]) as *const ::libc::c_void) },
        ) {
            (mut __tmp_0, mut __tmp_1) => {
                if 16_usize != 0 {
                    ::std::ptr::copy_nonoverlapping(__tmp_1, __tmp_0, 16_usize)
                }
                __tmp_0
            }
        };
        self.coeffs = std::mem::take(&mut (*_a0).coeffs);
        return &mut (*(self as *mut Elem));
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct State {
    pub count: i32,
    pub grid: [[i32; 3]; 2],
    pub arr: [i32; 3],
    pub elems: Vec<Elem>,
    pub rows: Vec<Vec<i32>>,
}
impl State {
    pub unsafe fn move_from(_a0: *mut State) -> Self {
        let mut this = Self {
            count: (*_a0).count,
            grid: std::array::from_fn::<_, 2, _>(|__i: usize| {
                std::array::from_fn::<_, 3, _>(|__i2: usize| (*_a0).grid[(__i)][(__i2)])
            }),
            arr: std::array::from_fn(|__i| (*&mut (&mut (*_a0)).arr[__i])),
            elems: std::mem::take(&mut (*_a0).elems),
            rows: std::mem::take(&mut (*_a0).rows),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut State) -> *mut State {
        self.count = (*_a0).count;
        match (
            { ((&mut self.grid as *mut [[i32; 3]; 2]) as *mut ::libc::c_void) },
            { ((&mut (*_a0).grid as *mut [[i32; 3]; 2]) as *const ::libc::c_void) },
        ) {
            (mut __tmp_2, mut __tmp_3) => {
                if 24_usize != 0 {
                    ::std::ptr::copy_nonoverlapping(__tmp_3, __tmp_2, 24_usize)
                }
                __tmp_2
            }
        };
        for __i in 0..3 {
            *&mut self.arr[__i] = (*&mut (&mut (*_a0)).arr[__i]);
        }
        self.elems = std::mem::take(&mut (*_a0).elems);
        self.rows = std::mem::take(&mut (*_a0).rows);
        return &mut (*(self as *mut State));
    }
}
impl Default for State {
    fn default() -> Self {
        unsafe {
            State {
                count: 0_i32,
                grid: [[0_i32; 3]; 2],
                arr: std::array::from_fn::<_, 3, _>(|_| 0_i32),
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
        let __a0 = 3_usize;
        (*s).elems.resize_with(__a0, || <Elem>::default())
    };
    let mut i: i32 = 0;
    'loop_: while ((i) < (3)) {
        (&mut (*s)).elems[(i as usize)].id = i;
        (&mut (*s)).elems[(i as usize)].hist[(i) as usize] = ((10) * (i));
        {
            let __a0 = 4_usize;
            (&mut (*s)).elems[(i as usize)]
                .coeffs
                .resize(__a0, (i as i16))
        };
        i.prefix_inc();
    }
    {
        let __a1 = Vec::from([1, 2, 3]);
        (*s).rows.push(__a1)
    };
}
pub static mut copies_2: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut moves_3: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut copy_assigns_4: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut move_assigns_5: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe, MoveAssignUnsafe)]
pub struct Counted {
    pub v: i32,
}
impl Counted {
    pub unsafe fn copy_from(o: *const Counted) -> Self {
        let mut this = Self { v: (*o).v };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_2)).prefix_inc();
        this
    }
    pub unsafe fn move_from(o: *mut Counted) -> Self {
        let mut this = Self { v: (*o).v };
        (*o).v = -1_i32;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_3)).prefix_inc();
        this
    }
    pub unsafe fn copy_assign(&mut self, o: *const Counted) -> *mut Counted {
        self.v = (*o).v;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_assigns_4)).prefix_inc();
        return &mut (*(self as *mut Counted));
    }
    pub unsafe fn move_assign(&mut self, o: *mut Counted) -> *mut Counted {
        self.v = (*o).v;
        (*o).v = -1_i32;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut move_assigns_5)).prefix_inc();
        return &mut (*(self as *mut Counted));
    }
}
impl Clone for Counted {
    fn clone(&self) -> Self {
        unsafe { Counted::copy_from(self as *const Counted) }
    }
}
impl Default for Counted {
    fn default() -> Self {
        unsafe { Counted { v: 0 } }
    }
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
    let mut elems: Vec<Elem> = (0..(2_usize) as usize)
        .map(|_| <Elem>::default())
        .collect::<Vec<_>>();
    let mut first: *mut Elem = (&mut elems[(0_usize)] as *mut Elem);
    {
        let __a0 = 2_usize;
        elems[(1_usize)].coeffs.resize_with(__a0, || 0_i16)
    };
    elems[(1_usize)].coeffs[(1_usize)] = 4_i16;
    elems[(1_usize)].hist[(2) as usize] = ((elems[(1_usize)].coeffs[(1_usize)] as i32) + (1));
    assert!(
        ((elems[(1_usize)].hist[(2) as usize]) == (5)) && (((*first).hist[(2) as usize]) == (0))
    );
    let mut moved: State = State::move_from({ &mut t });
    assert!(
        ((moved.grid[(0) as usize][(0) as usize]) == (100))
            && ((moved.grid[(1) as usize][(2) as usize]) == (5))
    );
    assert!((((moved.elems.len()) == (4_usize)) && (t.elems.is_empty())) && (t.rows.is_empty()));
    assert!(((moved.arr[(1_usize)]) == (7)) && ((t.arr[(1_usize)]) == (7)));
    let mut assigned: State = <State>::default();
    (unsafe { State::move_assign(&mut assigned, &mut moved) });
    assert!(
        ((assigned.grid[(0) as usize][(0) as usize]) == (100))
            && ((assigned.grid[(1) as usize][(1) as usize]) == (4))
    );
    assert!(
        ((assigned.elems[(2_usize)].coeffs[(1_usize)] as i32) == (7)) && (moved.elems.is_empty())
    );
    assert!(((assigned.arr[(1_usize)]) == (7)) && ((moved.arr[(1_usize)]) == (7)));
    let mut ca: [Counted; 3] = std::array::from_fn::<_, 3, _>(|_| <Counted>::default());
    ca[(1_usize)].v = 5;
    let mut cb: [Counted; 3] = std::array::from_fn(|__i| Counted::move_from(&mut ca[__i]));
    assert!(
        ((((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_3)) == (3))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_2)) == (0)))
            && ((cb[(1_usize)].v) == (5)))
            && ((ca[(1_usize)].v) == (-1_i32))
    );
    let mut cc: [Counted; 3] = cb.clone();
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_2)) == (3))
            && ((cc[(1_usize)].v) == (5)))
            && ((cb[(1_usize)].v) == (5))
    );
    assert!(((3) == (3_usize)));
    cc[(2_usize)].v = 8;
    for __i in 0..3 {
        (*&mut cb[__i]).copy_assign(&cc[__i]);
    }
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut copy_assigns_4)) == (3))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut move_assigns_5)) == (0)))
            && ((cb[(2_usize)].v) == (8))
    );
    for __i in 0..3 {
        (*&mut ca[__i]).move_assign(&mut cb[__i]);
    }
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut move_assigns_5)) == (3))
            && ((ca[(2_usize)].v) == (8)))
            && ((cb[(2_usize)].v) == (-1_i32))
    );
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_3)) == (3))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_2)) == (3)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut copy_assigns_4)) == (3))
    );
    let mut va: [Vec<i32>; 2] = std::array::from_fn::<_, 2, _>(|_| Default::default());
    {
        let __a1 = 1;
        va[(0_usize)].push(__a1)
    };
    let mut vb: [Vec<i32>; 2] =
        std::array::from_fn(|__i| <Vec<i32> as MoveCtorUnsafe>::move_ctor(&mut va[__i]));
    assert!(((vb[(0_usize)].len()) == (1_usize)) && (va[(0_usize)].is_empty()));
    let mut vc: [Vec<i32>; 2] = std::array::from_fn::<_, 2, _>(|_| Default::default());
    for __i in 0..2 {
        *&mut vc[__i] = <Vec<i32> as MoveCtorUnsafe>::move_ctor(&mut vb[__i]);
    }
    assert!(((vc[(0_usize)].len()) == (1_usize)) && (vb[(0_usize)].is_empty()));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copies_2);
    std::cell::LazyCell::force(&*&raw const moves_3);
    std::cell::LazyCell::force(&*&raw const copy_assigns_4);
    std::cell::LazyCell::force(&*&raw const move_assigns_5);
}
