extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut global_0: i32 = unsafe { 0 };
#[repr(C)]
#[derive(Clone, Default)]
pub struct S {}
impl S {
    pub unsafe fn destructor(&mut self) {
        global_0.postfix_inc();
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct Defaulted {
    pub s: S,
}
impl Defaulted {
    pub unsafe fn destructor(&mut self) {
        S::destructor(&mut self.s);
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct Middle {
    pub s: S,
}
impl Middle {
    pub unsafe fn destructor(&mut self) {
        S::destructor(&mut self.s);
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct Outer {
    pub m: Middle,
}
impl Outer {
    pub unsafe fn destructor(&mut self) {
        Middle::destructor(&mut self.m);
    }
}
#[repr(C)]
#[derive(Clone)]
pub struct ArrayMember {
    pub items: [S; 3],
}
impl ArrayMember {
    pub unsafe fn destructor(&mut self) {
        for __e in self.items.iter_mut() {
            S::destructor(__e);
        }
    }
}
impl Default for ArrayMember {
    fn default() -> Self {
        ArrayMember {
            items: std::array::from_fn::<_, 3, _>(|_| <S>::default()),
        }
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct EmptyBody {
    pub s: S,
}
impl EmptyBody {
    pub unsafe fn destructor(&mut self) {
        S::destructor(&mut self.s);
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct Templated_char_ {
    pub v: libc::c_char,
}
impl Templated_char_ {
    pub unsafe fn destructor(&mut self) {
        global_0 = ((global_0 as usize)
            .wrapping_add((::std::mem::size_of::<libc::c_char>() as usize)))
            as i32;
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct Templated_int_ {
    pub v: i32,
}
impl Templated_int_ {
    pub unsafe fn destructor(&mut self) {
        global_0 =
            ((global_0 as usize).wrapping_add((::std::mem::size_of::<i32>() as usize))) as i32;
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct Copied {
    pub v: i32,
}
impl Copied {
    pub unsafe fn destructor(&mut self) {
        global_0.postfix_inc();
    }
}
pub static mut order_1: [i32; 3] = unsafe { [0_i32; 3] };
pub static mut order_count_2: i32 = unsafe { 0 };
#[repr(C)]
#[derive(Clone, Default)]
pub struct Tagged {
    pub tag: i32,
}
impl Tagged {
    pub unsafe fn destructor(&mut self) {
        order_1[(order_count_2.postfix_inc()) as usize] = self.tag;
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct Ordered {
    pub first: Tagged,
    pub dummy1: i32,
    pub second: Tagged,
    pub dummy2: i32,
    pub third: Tagged,
}
impl Ordered {
    pub unsafe fn destructor(&mut self) {
        Tagged::destructor(&mut self.third);
        Tagged::destructor(&mut self.second);
        Tagged::destructor(&mut self.first);
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    {
        let mut s: S = S {};
        let _dtor_s = ScopedDestructorUnsafe::new(&raw mut s, S::destructor);
    }
    assert!(((global_0) == (1)));
    {
        let mut s: S = S {};
        let _dtor_s = ScopedDestructorUnsafe::new(&raw mut s, S::destructor);
    }
    assert!(((global_0) == (2)));
    {
        let mut d: Defaulted = Defaulted { s: S {} };
        let _dtor_d = ScopedDestructorUnsafe::new(&raw mut d, Defaulted::destructor);
    }
    assert!(((global_0) == (3)));
    {
        let mut o: Outer = Outer {
            m: Middle { s: S {} },
        };
        let _dtor_o = ScopedDestructorUnsafe::new(&raw mut o, Outer::destructor);
    }
    assert!(((global_0) == (4)));
    {
        let mut am: ArrayMember = ArrayMember {
            items: [S {}, S {}, S {}],
        };
        let _dtor_am = ScopedDestructorUnsafe::new(&raw mut am, ArrayMember::destructor);
    }
    assert!(((global_0) == (7)));
    {
        let mut e: EmptyBody = EmptyBody { s: S {} };
        let _dtor_e = ScopedDestructorUnsafe::new(&raw mut e, EmptyBody::destructor);
    }
    assert!(((global_0) == (8)));
    {
        let mut tc: Templated_char_ = Templated_char_ {
            v: (0 as libc::c_char),
        };
        let _dtor_tc = ScopedDestructorUnsafe::new(&raw mut tc, Templated_char_::destructor);
        let mut ti: Templated_int_ = Templated_int_ { v: 0_i32 };
        let _dtor_ti = ScopedDestructorUnsafe::new(&raw mut ti, Templated_int_::destructor);
    }
    assert!(((global_0) == (13)));
    {
        let mut a: Copied = Copied { v: 5 };
        let _dtor_a = ScopedDestructorUnsafe::new(&raw mut a, Copied::destructor);
        let mut b: Copied = a.clone();
        let _dtor_b = ScopedDestructorUnsafe::new(&raw mut b, Copied::destructor);
        assert!(((b.v) == (5)));
    }
    assert!(((global_0) == (15)));
    {
        let mut o: Ordered = Ordered {
            first: Tagged { tag: 1 },
            dummy1: 0,
            second: Tagged { tag: 2 },
            dummy2: 0,
            third: Tagged { tag: 3 },
        };
        let _dtor_o = ScopedDestructorUnsafe::new(&raw mut o, Ordered::destructor);
    }
    assert!(((order_count_2) == (3)));
    assert!(((order_1[(0) as usize]) == (3)));
    assert!(((order_1[(1) as usize]) == (2)));
    assert!(((order_1[(2) as usize]) == (1)));
    return 0;
}
