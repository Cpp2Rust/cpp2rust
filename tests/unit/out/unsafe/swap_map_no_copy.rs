extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut copies_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, CopyAssignUnsafe, MoveAssignUnsafe)]
#[cpp_from(i32, new_1, value)]
pub struct Tracked {
    pub v: i32,
}
impl Tracked {
    pub unsafe fn new_2() -> Self {
        let mut this = Self { v: 0 };
        this
    }
    pub unsafe fn new_1(mut v: i32) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn copy_from(o: *const Tracked) -> Self {
        let mut this = Self { v: (*o).v };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
        this
    }
    pub unsafe fn move_from(o: *mut Tracked) -> Self {
        let mut this = Self { v: (*o).v };
        this
    }
    pub unsafe fn copy_assign(&mut self, o: *const Tracked) -> *mut Tracked {
        self.v = (*o).v;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
        return &mut (*(self as *mut Tracked));
    }
    pub unsafe fn move_assign(&mut self, o: *mut Tracked) -> *mut Tracked {
        self.v = (*o).v;
        return &mut (*(self as *mut Tracked));
    }
}
impl Clone for Tracked {
    fn clone(&self) -> Self {
        unsafe { Tracked::copy_from(self as *const Tracked) }
    }
}
impl Default for Tracked {
    fn default() -> Self {
        unsafe { Tracked::new_2() }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe, MoveAssignUnsafe, Default)]
pub struct Pair {
    pub x: Tracked,
    pub y: Tracked,
}
impl Pair {
    pub unsafe fn move_from(_a0: *mut Pair) -> Self {
        let mut this = Self {
            x: Tracked::move_from({ &mut (*_a0).x }),
            y: Tracked::move_from({ &mut (*_a0).y }),
        };
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const Pair) -> *mut Pair {
        (unsafe {
            let _o: *const Tracked = &(*_a0).x;
            Tracked::copy_assign(&mut self.x, _o)
        });
        (unsafe {
            let _o: *const Tracked = &(*_a0).y;
            Tracked::copy_assign(&mut self.y, _o)
        });
        return &mut (*(self as *mut Pair));
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Pair) -> *mut Pair {
        (unsafe {
            let _o: *mut Tracked = &mut (*_a0).x;
            Tracked::move_assign(&mut self.x, _o)
        });
        (unsafe {
            let _o: *mut Tracked = &mut (*_a0).y;
            Tracked::move_assign(&mut self.y, _o)
        });
        return &mut (*(self as *mut Pair));
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe, MoveAssignUnsafe, Default)]
pub struct Arrays {
    pub a: [Tracked; 2],
    pub b: [Tracked; 2],
}
impl Arrays {
    pub unsafe fn move_from(_a0: *mut Arrays) -> Self {
        let mut this = Self {
            a: std::array::from_fn::<_, 2, _>(|__i: usize| {
                Tracked::move_from({ &mut (*_a0).a[(__i)] })
            }),
            b: std::array::from_fn::<_, 2, _>(|__i: usize| {
                Tracked::move_from({ &mut (*_a0).b[(__i)] })
            }),
        };
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const Arrays) -> *mut Arrays {
        let mut __i0: usize = 0_usize;
        'loop_: while ((__i0) != (2_usize)) {
            (unsafe {
                let _o: *const Tracked = &(*_a0).a[(__i0)];
                Tracked::copy_assign(&mut self.a[(__i0)], _o)
            });
            __i0.prefix_inc();
        }
        let mut __i0: usize = 0_usize;
        'loop_: while ((__i0) != (2_usize)) {
            (unsafe {
                let _o: *const Tracked = &(*_a0).b[(__i0)];
                Tracked::copy_assign(&mut self.b[(__i0)], _o)
            });
            __i0.prefix_inc();
        }
        return &mut (*(self as *mut Arrays));
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Arrays) -> *mut Arrays {
        let mut __i0: usize = 0_usize;
        'loop_: while ((__i0) != (2_usize)) {
            (unsafe {
                let _o: *mut Tracked = &mut (*_a0).a[(__i0)];
                Tracked::move_assign(&mut self.a[(__i0)], _o)
            });
            __i0.prefix_inc();
        }
        let mut __i0: usize = 0_usize;
        'loop_: while ((__i0) != (2_usize)) {
            (unsafe {
                let _o: *mut Tracked = &mut (*_a0).b[(__i0)];
                Tracked::move_assign(&mut self.b[(__i0)], _o)
            });
            __i0.prefix_inc();
        }
        return &mut (*(self as *mut Arrays));
    }
}
pub static mut copy_only_copies_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut copy_only_dtors_2: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CppFromUnsafe, CopyAssignUnsafe, DestructorUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct CopyOnly {
    pub v: i32,
}
impl CopyOnly {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn copy_from(o: *const CopyOnly) -> Self {
        let mut this = Self { v: (*o).v };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_only_copies_1)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_only_dtors_2)).prefix_inc();
    }
    pub unsafe fn copy_assign(&mut self, o: *const CopyOnly) -> *mut CopyOnly {
        self.v = (*o).v;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_only_copies_1)).prefix_inc();
        return &mut (*(self as *mut CopyOnly));
    }
}
impl Clone for CopyOnly {
    fn clone(&self) -> Self {
        unsafe { CopyOnly::copy_from(self as *const CopyOnly) }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe, Default)]
pub struct CopyOnlyPair {
    pub x: CopyOnly,
    pub y: CopyOnly,
}
impl CopyOnlyPair {
    pub unsafe fn copy_assign(&mut self, _a0: *const CopyOnlyPair) -> *mut CopyOnlyPair {
        (unsafe {
            let _o: *const CopyOnly = &(*_a0).x;
            CopyOnly::copy_assign(&mut self.x, _o)
        });
        (unsafe {
            let _o: *const CopyOnly = &(*_a0).y;
            CopyOnly::copy_assign(&mut self.y, _o)
        });
        return &mut (*(self as *mut CopyOnlyPair));
    }
}
impl CopyOnlyPair {
    pub unsafe fn destructor(&mut self) {
        CopyOnly::destructor(&mut self.y);
        CopyOnly::destructor(&mut self.x);
    }
}
pub static mut moves_3: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut move_assigns_4: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut moved_dtors_5: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(
    VaArg,
    FnPtrArg,
    MoveCtorUnsafe,
    CppFromUnsafe,
    CopyAssignUnsafe,
    MoveAssignUnsafe,
    DestructorUnsafe,
    Default,
)]
#[cpp_from(i32, new, value)]
pub struct Moved {
    pub v: i32,
}
impl Moved {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn copy_from(o: *const Moved) -> Self {
        let mut this = Self { v: (*o).v };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
        this
    }
    pub unsafe fn move_from(o: *mut Moved) -> Self {
        let mut this = Self { v: (*o).v };
        (*o).v = -1_i32;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_3)).prefix_inc();
        this
    }
    pub unsafe fn copy_assign(&mut self, o: *const Moved) -> *mut Moved {
        self.v = (*o).v;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
        return &mut (*(self as *mut Moved));
    }
    pub unsafe fn move_assign(&mut self, o: *mut Moved) -> *mut Moved {
        self.v = (*o).v;
        (*o).v = -1_i32;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut move_assigns_4)).prefix_inc();
        return &mut (*(self as *mut Moved));
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_dtors_5)).prefix_inc();
    }
}
impl Clone for Moved {
    fn clone(&self) -> Self {
        unsafe { Moved::copy_from(self as *const Moved) }
    }
}
#[repr(C)]
#[derive(
    Clone,
    VaArg,
    FnPtrArg,
    MoveCtorUnsafe,
    CopyAssignUnsafe,
    MoveAssignUnsafe,
    DestructorUnsafe,
    Default,
)]
pub struct MovedPair {
    pub x: Moved,
    pub y: Moved,
}
impl MovedPair {
    pub unsafe fn move_from(_a0: *mut MovedPair) -> Self {
        let mut this = Self {
            x: Moved::move_from({ &mut (*_a0).x }),
            y: Moved::move_from({ &mut (*_a0).y }),
        };
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const MovedPair) -> *mut MovedPair {
        (unsafe {
            let _o: *const Moved = &(*_a0).x;
            Moved::copy_assign(&mut self.x, _o)
        });
        (unsafe {
            let _o: *const Moved = &(*_a0).y;
            Moved::copy_assign(&mut self.y, _o)
        });
        return &mut (*(self as *mut MovedPair));
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut MovedPair) -> *mut MovedPair {
        (unsafe {
            let _o: *mut Moved = &mut (*_a0).x;
            Moved::move_assign(&mut self.x, _o)
        });
        (unsafe {
            let _o: *mut Moved = &mut (*_a0).y;
            Moved::move_assign(&mut self.y, _o)
        });
        return &mut (*(self as *mut MovedPair));
    }
}
impl MovedPair {
    pub unsafe fn destructor(&mut self) {
        Moved::destructor(&mut self.y);
        Moved::destructor(&mut self.x);
    }
}
pub unsafe fn swapped_once_6() -> bool {
    let mut ok: bool = (((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_3)) == (1))
        && ((*std::cell::LazyCell::force_mut(&mut *&raw mut move_assigns_4)) == (2)))
        && ((*std::cell::LazyCell::force_mut(&mut *&raw mut moved_dtors_5)) == (1));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_3)) = {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut move_assigns_4)) = {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_dtors_5)) = 0;
            (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_dtors_5))
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut move_assigns_4))
    };
    return ok;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Owner {
    pub p: Option<Box<i32>>,
}
impl Owner {
    pub unsafe fn move_from(_a0: *mut Owner) -> Self {
        let mut this = Self { p: (*_a0).p.take() };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Owner) -> *mut Owner {
        self.p = (*_a0).p.take();
        return &mut (*(self as *mut Owner));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: Tracked = Tracked::new_1({ 1 });
    let mut b: Tracked = Tracked::new_1({ 2 });
    cpp_swap_unsafe!(&mut a, &mut b);
    assert!(((a.v) == (2)) && ((b.v) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    let mut m1: UnsafeMap<i32, (i32, Tracked)> = UnsafeMap::new();
    (unsafe {
        let mut _o: Tracked = Tracked::new_1({ 10 });
        Tracked::move_assign(
            &mut (*m1.subscript_move(&mut 1, |__k| cpp_move_unsafe!(__k))),
            &mut _o,
        )
    });
    (unsafe {
        let mut _o: Tracked = Tracked::new_1({ 20 });
        Tracked::move_assign(
            &mut (*m1.subscript_move(&mut 2, |__k| cpp_move_unsafe!(__k))),
            &mut _o,
        )
    });
    let mut m2: UnsafeMap<i32, (i32, Tracked)> = UnsafeMap::new();
    (unsafe {
        let mut _o: Tracked = Tracked::new_1({ 30 });
        Tracked::move_assign(
            &mut (*m2.subscript_move(&mut 3, |__k| cpp_move_unsafe!(__k))),
            &mut _o,
        )
    });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    cpp_swap_unsafe!(
        &mut (*m1.subscript_move(&mut 1, |__k| cpp_move_unsafe!(__k))),
        &mut (*m1.subscript_move(&mut 2, |__k| cpp_move_unsafe!(__k)))
    );
    assert!(
        (((*m1.subscript_move(&mut 1, |__k| cpp_move_unsafe!(__k))).v) == (20))
            && (((*m1.subscript_move(&mut 2, |__k| cpp_move_unsafe!(__k))).v) == (10))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    cpp_swap_unsafe!(&mut m1, &mut m2);
    assert!(
        ((m1.len()) == (1_usize))
            && (((*m1.subscript_move(&mut 3, |__k| cpp_move_unsafe!(__k))).v) == (30))
    );
    assert!(
        (((m2.len()) == (2_usize))
            && (((*m2.subscript_move(&mut 1, |__k| cpp_move_unsafe!(__k))).v) == (20)))
            && (((*m2.subscript_move(&mut 2, |__k| cpp_move_unsafe!(__k))).v) == (10))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    let mut arr: [Tracked; 3] = [
        Tracked::new_1({ 1 }),
        Tracked::new_1({ 2 }),
        Tracked::new_1({ 3 }),
    ];
    cpp_swap_unsafe!(&mut arr[(0) as usize], &mut arr[(2) as usize]);
    assert!(((arr[(0) as usize].v) == (3)) && ((arr[(2) as usize].v) == (1)));
    cpp_swap_unsafe!(&mut arr[(1) as usize], &mut arr[(1) as usize]);
    assert!(((arr[(1) as usize].v) == (2)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    let mut vec_: Vec<Tracked> = Vec::new();
    {
        let __init = Tracked::new_1({ 4 });
        vec_.push(__init)
    };
    {
        let __init = Tracked::new_1({ 5 });
        vec_.push(__init)
    };
    cpp_swap_unsafe!(&mut vec_[(0_usize)], &mut vec_[(1_usize)]);
    assert!(((vec_[(0_usize)].v) == (5)) && ((vec_[(1_usize)].v) == (4)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    let mut pairs: [Pair; 2] = [
        Pair {
            x: Tracked::new_1({ 6 }),
            y: Tracked::new_1({ 7 }),
        },
        Pair {
            x: Tracked::new_1({ 8 }),
            y: Tracked::new_1({ 9 }),
        },
    ];
    let mut p: Pair = Pair {
        x: Tracked::new_1({ 10 }),
        y: Tracked::new_1({ 11 }),
    };
    cpp_swap_unsafe!(&mut pairs[(0) as usize].x, &mut p.y);
    assert!(((pairs[(0) as usize].x.v) == (11)) && ((p.y.v) == (6)));
    cpp_swap_unsafe!(&mut pairs[(0) as usize], &mut pairs[(1) as usize]);
    assert!(((pairs[(0) as usize].x.v) == (8)) && ((pairs[(1) as usize].x.v) == (11)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    cpp_swap_unsafe!(&mut p.x, &mut p.y);
    assert!(((p.x.v) == (6)) && ((p.y.v) == (10)));
    cpp_swap_unsafe!(&mut pairs[(0) as usize].x, &mut pairs[(1) as usize].y);
    assert!(((pairs[(0) as usize].x.v) == (7)) && ((pairs[(1) as usize].y.v) == (8)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    let mut arrays: Arrays = <Arrays>::default();
    arrays.a[(0) as usize].v = 1;
    arrays.b[(1) as usize].v = 2;
    cpp_swap_unsafe!(&mut arrays.a[(0) as usize], &mut arrays.b[(1) as usize]);
    assert!(((arrays.a[(0) as usize].v) == (2)) && ((arrays.b[(1) as usize].v) == (1)));
    cpp_swap_unsafe!(&mut arrays.a[(0) as usize], &mut arrays.a[(1) as usize]);
    assert!(((arrays.a[(0) as usize].v) == (0)) && ((arrays.a[(1) as usize].v) == (2)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    let mut grid: Vec<Vec<Tracked>> = (0..(2_usize) as usize)
        .map(|_| Default::default())
        .collect::<Vec<_>>();
    {
        let __init = Tracked::new_1({ 1 });
        grid[(0_usize)].push(__init)
    };
    {
        let __init = Tracked::new_1({ 2 });
        grid[(0_usize)].push(__init)
    };
    {
        let __init = Tracked::new_1({ 3 });
        grid[(1_usize)].push(__init)
    };
    cpp_swap_unsafe!(
        &mut grid[(0_usize)][(1_usize)],
        &mut grid[(1_usize)][(0_usize)]
    );
    assert!(((grid[(0_usize)][(1_usize)].v) == (3)) && ((grid[(1_usize)][(0_usize)].v) == (2)));
    cpp_swap_unsafe!(&mut grid[(0_usize)], &mut grid[(1_usize)]);
    assert!(((grid[(0_usize)].len()) == (1_usize)) && ((grid[(0_usize)][(0_usize)].v) == (2)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    let mut maps: [UnsafeMap<i32, (i32, Tracked)>; 2] =
        std::array::from_fn::<_, 2, _>(|_| UnsafeMap::new());
    (unsafe {
        let mut _o: Tracked = Tracked::new_1({ 1 });
        Tracked::move_assign(
            &mut (*maps[(0) as usize].subscript_move(&mut 1, |__k| cpp_move_unsafe!(__k))),
            &mut _o,
        )
    });
    (unsafe {
        let mut _o: Tracked = Tracked::new_1({ 2 });
        Tracked::move_assign(
            &mut (*maps[(1) as usize].subscript_move(&mut 2, |__k| cpp_move_unsafe!(__k))),
            &mut _o,
        )
    });
    cpp_swap_unsafe!(&mut maps[(0) as usize], &mut maps[(1) as usize]);
    assert!(
        ((maps[(0) as usize].len()) == (1_usize))
            && (((*maps[(0) as usize].subscript_move(&mut 2, |__k| cpp_move_unsafe!(__k))).v)
                == (2))
    );
    assert!(
        ((maps[(1) as usize].len()) == (1_usize))
            && (((*maps[(1) as usize].subscript_move(&mut 1, |__k| cpp_move_unsafe!(__k))).v)
                == (1))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    let mut c1: CopyOnly = CopyOnly::new({ 1 });
    let _dtor_c1 = ScopedDestructorUnsafe::new(&raw mut c1, CopyOnly::destructor);
    let mut c2: CopyOnly = CopyOnly::new({ 2 });
    let _dtor_c2 = ScopedDestructorUnsafe::new(&raw mut c2, CopyOnly::destructor);
    cpp_swap_unsafe!(&mut c1, &mut c2);
    assert!(((c1.v) == (2)) && ((c2.v) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut copy_only_copies_1)) == (3))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut copy_only_dtors_2)) == (1))
    );
    let mut cp: CopyOnlyPair = CopyOnlyPair {
        x: CopyOnly::new({ 3 }),
        y: CopyOnly::new({ 4 }),
    };
    let _dtor_cp = ScopedDestructorUnsafe::new(&raw mut cp, CopyOnlyPair::destructor);
    cpp_swap_unsafe!(&mut cp.x, &mut cp.y);
    assert!(((cp.x.v) == (4)) && ((cp.y.v) == (3)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut copy_only_copies_1)) == (6))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut copy_only_dtors_2)) == (2))
    );
    let mut carr: [CopyOnly; 2] = [CopyOnly::new({ 5 }), CopyOnly::new({ 6 })];
    cpp_swap_unsafe!(&mut carr[(0) as usize], &mut carr[(1) as usize]);
    assert!(((carr[(0) as usize].v) == (6)) && ((carr[(1) as usize].v) == (5)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut copy_only_copies_1)) == (9))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut copy_only_dtors_2)) == (3))
    );
    let mut mv1: Moved = Moved::new({ 1 });
    let _dtor_mv1 = ScopedDestructorUnsafe::new(&raw mut mv1, Moved::destructor);
    let mut mv2: Moved = Moved::new({ 2 });
    let _dtor_mv2 = ScopedDestructorUnsafe::new(&raw mut mv2, Moved::destructor);
    cpp_swap_unsafe!(&mut mv1, &mut mv2);
    assert!(((mv1.v) == (2)) && ((mv2.v) == (1)));
    assert!((unsafe { swapped_once_6() }));
    let mut mp: MovedPair = MovedPair {
        x: Moved::new({ 3 }),
        y: Moved::new({ 4 }),
    };
    let _dtor_mp = ScopedDestructorUnsafe::new(&raw mut mp, MovedPair::destructor);
    cpp_swap_unsafe!(&mut mp.x, &mut mp.y);
    assert!(((mp.x.v) == (4)) && ((mp.y.v) == (3)));
    assert!((unsafe { swapped_once_6() }));
    let mut marr: [Moved; 2] = [Moved::new({ 5 }), Moved::new({ 6 })];
    cpp_swap_unsafe!(&mut marr[(0) as usize], &mut marr[(1) as usize]);
    assert!(((marr[(0) as usize].v) == (6)) && ((marr[(1) as usize].v) == (5)));
    assert!((unsafe { swapped_once_6() }));
    cpp_swap_unsafe!(&mut mv1, &mut mv1);
    assert!(((mv1.v) == (2)));
    assert!((unsafe { swapped_once_6() }));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    let mut o1: Owner = Owner {
        p: Some(Box::new(1)),
    };
    let mut o2: Owner = Owner {
        p: Some(Box::new(2)),
    };
    cpp_swap_unsafe!(&mut o1, &mut o2);
    assert!(
        ((*o1
            .p
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (2))
            && ((*o2
                .p
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
    );
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0));
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copies_0);
    std::cell::LazyCell::force(&*&raw const copy_only_copies_1);
    std::cell::LazyCell::force(&*&raw const copy_only_dtors_2);
    std::cell::LazyCell::force(&*&raw const moves_3);
    std::cell::LazyCell::force(&*&raw const move_assigns_4);
    std::cell::LazyCell::force(&*&raw const moved_dtors_5);
}
