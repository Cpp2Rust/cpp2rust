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
pub struct Inner {
    pub x: i32,
}
#[repr(C)]
#[derive(Clone)]
pub struct Explicit {
    pub v: i32,
    pub inner: Inner,
    pub arr: [i32; 2],
}
impl Explicit {
    pub unsafe fn Explicit(mut v: i32) -> Self {
        let mut this = Self {
            v: v,
            inner: Inner { x: ((v) * (10)) },
            arr: [v, ((v) + (1))],
        };
        this
    }
    pub unsafe fn destructor(&mut self) {}
}
impl Default for Explicit {
    fn default() -> Self {
        Explicit {
            v: 0_i32,
            inner: <Inner>::default(),
            arr: [0_i32; 2],
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Implicit {
    pub v: i32,
    pub inner: Inner,
    pub arr: [i32; 2],
}
impl Default for Implicit {
    fn default() -> Self {
        Implicit {
            v: 0_i32,
            inner: <Inner>::default(),
            arr: [0_i32; 2],
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct DefaultCopyUserMove {
    pub v: i32,
}
impl DefaultCopyUserMove {
    pub unsafe fn DefaultCopyUserMove(mut v: i32) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn DefaultCopyUserMove_pmutDefaultCopyUserMove(o: *mut DefaultCopyUserMove) -> Self {
        let mut this = Self { v: (*o).v };
        (*o).v = 0;
        this
    }
    pub unsafe fn operator_assign_pmutDefaultCopyUserMove(
        &mut self,
        o: *mut DefaultCopyUserMove,
    ) -> *mut DefaultCopyUserMove {
        self.v = (*o).v;
        (*o).v = 0;
        return &mut (*(self as *mut DefaultCopyUserMove));
    }
}
#[repr(C)]
#[derive(Default)]
pub struct UserCopyDefaultMove {
    pub v: i32,
}
impl UserCopyDefaultMove {
    pub unsafe fn UserCopyDefaultMove(mut v: i32) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn UserCopyDefaultMove_pconstUserCopyDefaultMove(
        o: *const UserCopyDefaultMove,
    ) -> Self {
        let mut this = Self {
            v: (((*o).v) + (100)),
        };
        this
    }
    pub unsafe fn UserCopyDefaultMove_pmutUserCopyDefaultMove(
        _a0: *mut UserCopyDefaultMove,
    ) -> Self {
        let mut this = Self { v: (*_a0).v };
        this
    }
    pub unsafe fn operator_assign_pconstUserCopyDefaultMove(
        &mut self,
        o: *const UserCopyDefaultMove,
    ) -> *mut UserCopyDefaultMove {
        self.v = (((*o).v) + (100));
        return &mut (*(self as *mut UserCopyDefaultMove));
    }
    pub unsafe fn operator_assign_pmutUserCopyDefaultMove(
        &mut self,
        _a0: *mut UserCopyDefaultMove,
    ) -> *mut UserCopyDefaultMove {
        self.v = (*_a0).v;
        return &mut (*(self as *mut UserCopyDefaultMove));
    }
}
impl Clone for UserCopyDefaultMove {
    fn clone(&self) -> Self {
        unsafe {
            UserCopyDefaultMove::UserCopyDefaultMove_pconstUserCopyDefaultMove(
                self as *const UserCopyDefaultMove,
            )
        }
    }
}
#[repr(C)]
#[derive()]
pub struct Buffer {
    pub data: Vec<i32>,
    pub rows: Vec<Vec<i32>>,
    pub n: i32,
    pub arr: [i32; 2],
}
impl Buffer {
    pub unsafe fn Buffer(mut n: i32) -> Self {
        let mut this = Self {
            data: vec![n; (n as usize) as usize],
            rows: Vec::new(),
            n: n,
            arr: [n, ((n) + (1))],
        };
        this.rows.push(this.data.clone());
        this
    }
    pub unsafe fn Buffer_pmutBuffer(_a0: *mut Buffer) -> Self {
        let mut this = Self {
            data: std::mem::take(&mut (*_a0).data),
            rows: std::mem::take(&mut (*_a0).rows),
            n: (*_a0).n,
            arr: std::array::from_fn::<_, 2, _>(|__i: usize| (*_a0).arr[(__i)]),
        };
        this
    }
    pub unsafe fn operator_assign_pmutBuffer(&mut self, _a0: *mut Buffer) -> *mut Buffer {
        self.data = std::mem::take(&mut (*_a0).data);
        self.rows = std::mem::take(&mut (*_a0).rows);
        self.n = (*_a0).n;
        {
            if 8_usize != 0 {
                ::std::ptr::copy_nonoverlapping(
                    ((&mut (*_a0).arr as *mut [i32; 2]) as *const [i32; 2]
                        as *const ::libc::c_void),
                    ((&mut self.arr as *mut [i32; 2]) as *mut [i32; 2] as *mut ::libc::c_void),
                    8_usize as usize,
                )
            }
            ((&mut self.arr as *mut [i32; 2]) as *mut [i32; 2] as *mut ::libc::c_void)
        };
        return &mut (*(self as *mut Buffer));
    }
}
impl Default for Buffer {
    fn default() -> Self {
        Buffer {
            data: Default::default(),
            rows: Vec::new(),
            n: 0_i32,
            arr: [0_i32; 2],
        }
    }
}
pub unsafe fn same_0(a: *const Explicit, b: *const Explicit) -> bool {
    return (((((*a).v) == ((*b).v)) && (((*a).inner.x) == ((*b).inner.x)))
        && (((*a).arr[(0) as usize]) == ((*b).arr[(0) as usize])))
        && (((*a).arr[(1) as usize]) == ((*b).arr[(1) as usize]));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: Explicit = Explicit::Explicit({ 1 });
    let _dtor_a = ScopedDestructorUnsafe::new(&raw mut a, Explicit::destructor);
    let mut b: Explicit = a.clone();
    let _dtor_b = ScopedDestructorUnsafe::new(&raw mut b, Explicit::destructor);
    let mut c: Explicit = a.clone();
    let _dtor_c = ScopedDestructorUnsafe::new(&raw mut c, Explicit::destructor);
    let mut d: Explicit = a.clone();
    let _dtor_d = ScopedDestructorUnsafe::new(&raw mut d, Explicit::destructor);
    assert!(
        ((unsafe { same_0(&b, &a,) }) && (unsafe { same_0(&c, &a,) }))
            && (unsafe { same_0(&d, &a,) })
    );
    let mut e: Explicit = Explicit::Explicit({ 2 });
    let _dtor_e = ScopedDestructorUnsafe::new(&raw mut e, Explicit::destructor);
    let mut f: Explicit = Explicit::Explicit({ 3 });
    let _dtor_f = ScopedDestructorUnsafe::new(&raw mut f, Explicit::destructor);
    e = (b).clone();
    f = (c).clone();
    assert!((unsafe { same_0(&e, &b,) }) && (unsafe { same_0(&f, &c,) }));
    let mut g: Explicit = Explicit::Explicit({ 4 });
    let _dtor_g = ScopedDestructorUnsafe::new(&raw mut g, Explicit::destructor);
    g = {
        e = (f).clone();
        (e).clone()
    };
    assert!((unsafe { same_0(&g, &f,) }) && (unsafe { same_0(&e, &f,) }));
    let mut i: Implicit = Implicit {
        v: 5,
        inner: Inner { x: 50 },
        arr: [5, 6],
    };
    let mut j: Implicit = i;
    let mut k: Implicit = i;
    assert!((((j.v) == (5)) && ((j.inner.x) == (50))) && ((j.arr[(1) as usize]) == (6)));
    assert!(((i.v) == (5)) && ((k.v) == (5)));
    let mut l: Implicit = Implicit {
        v: 0,
        inner: Inner { x: 0 },
        arr: [0, 0],
    };
    l = j;
    assert!((((l.v) == (5)) && ((l.inner.x) == (50))) && ((l.arr[(0) as usize]) == (5)));
    let mut vec_: Vec<Explicit> = Vec::new();
    {
        let a0_clone = b.clone();
        vec_.push(a0_clone)
    };
    vec_.push(Explicit::Explicit({ 9 }));
    assert!(((vec_[(0_usize)].v) == (1)) && ((vec_[(1_usize)].v) == (9)));
    let mut m: DefaultCopyUserMove = DefaultCopyUserMove::DefaultCopyUserMove({ 7 });
    let mut m1: DefaultCopyUserMove = m;
    let mut m2: DefaultCopyUserMove =
        DefaultCopyUserMove::DefaultCopyUserMove_pmutDefaultCopyUserMove({ &mut m });
    assert!((((m1.v) == (7)) && ((m2.v) == (7))) && ((m.v) == (0)));
    let mut m3: DefaultCopyUserMove = DefaultCopyUserMove::DefaultCopyUserMove({ 1 });
    let mut m4: DefaultCopyUserMove = DefaultCopyUserMove::DefaultCopyUserMove({ 1 });
    m3 = m1;
    (unsafe { DefaultCopyUserMove::operator_assign_pmutDefaultCopyUserMove(&mut m4, &mut m1) });
    assert!((((m3.v) == (7)) && ((m4.v) == (7))) && ((m1.v) == (0)));
    let mut u: UserCopyDefaultMove = UserCopyDefaultMove::UserCopyDefaultMove({ 8 });
    let mut u1: UserCopyDefaultMove =
        UserCopyDefaultMove::UserCopyDefaultMove_pconstUserCopyDefaultMove({ &u });
    let mut u2: UserCopyDefaultMove =
        UserCopyDefaultMove::UserCopyDefaultMove_pmutUserCopyDefaultMove({ &mut u });
    assert!((((u1.v) == (108)) && ((u2.v) == (8))) && ((u.v) == (8)));
    let mut u3: UserCopyDefaultMove = UserCopyDefaultMove::UserCopyDefaultMove({ 1 });
    let mut u4: UserCopyDefaultMove = UserCopyDefaultMove::UserCopyDefaultMove({ 1 });
    (unsafe { UserCopyDefaultMove::operator_assign_pconstUserCopyDefaultMove(&mut u3, &u2) });
    (unsafe { UserCopyDefaultMove::operator_assign_pmutUserCopyDefaultMove(&mut u4, &mut u2) });
    assert!(((u3.v) == (108)) && ((u4.v) == (8)));
    let mut p: Buffer = Buffer::Buffer({ 3 });
    let mut q: Buffer = Buffer::Buffer_pmutBuffer({ &mut p });
    assert!(
        ((((q.n) == (3)) && ((q.data.len()) == (3_usize))) && ((q.data[(2_usize)]) == (3)))
            && (p.data.is_empty())
    );
    let mut r: Buffer = Buffer::Buffer({ 1 });
    (unsafe { Buffer::operator_assign_pmutBuffer(&mut r, &mut q) });
    assert!(
        ((((r.n) == (3)) && ((r.data.len()) == (3_usize))) && ((r.arr[(1) as usize]) == (4)))
            && (q.data.is_empty())
    );
    assert!(
        (((r.rows.len()) == (1_usize)) && ((r.rows[(0_usize)].len()) == (3_usize)))
            && (q.rows.is_empty())
    );
    let mut bufs: Vec<Buffer> = Vec::new();
    bufs.push(Buffer::Buffer_pmutBuffer({ &mut r }));
    {
        let __arg = Buffer::Buffer_pmutBuffer({ &mut bufs[(0_usize)] });
        bufs.push(__arg)
    };
    assert!(
        (((bufs[(1_usize)].n) == (3)) && ((bufs[(1_usize)].data.len()) == (3_usize)))
            && (bufs[(0_usize)].data.is_empty())
    );
    return 0;
}
