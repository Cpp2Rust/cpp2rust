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
pub struct Inner {
    pub a: i32,
    pub name: [libc::c_char; 8],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, Default)]
pub struct Header {
    pub tag: i32,
    pub size: i16,
}
#[repr(C)]
#[derive(Clone, VaArg, Default)]
pub struct Outer {
    pub x: i32,
    pub inner: Inner,
    pub items: [Inner; 3],
    pub v: Vec<i32>,
    pub cursor: *mut i32,
    pub buf: [i32; 4],
}
impl Outer {
    pub unsafe fn next(&mut self) -> i32 {
        return self.buf[(self.x.postfix_inc()) as usize];
    }
    pub unsafe fn sum(&self) -> i32 {
        return ((self.x) + (self.inner.a));
    }
    pub unsafe fn push(&mut self, mut k: i32) {
        {
            let __a1 = ((k) + (self.x));
            self.v.push(__a1)
        };
    }
}
pub unsafe fn set_0(mut p: *mut i32, mut value: i32) {
    (*p) = value;
}
pub unsafe fn bump_1(mut o: *mut Outer) -> i32 {
    (*o).x.postfix_inc();
    return 1;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut o: Outer = <Outer>::default();
    o.x = 1;
    let mut px: *mut i32 = (&mut o.x as *mut i32);
    (*px) = 2;
    assert!(((o.x) == (2)));
    let mut pa: *mut i32 = (&mut o.inner.a as *mut i32);
    (unsafe {
        let _p: *mut i32 = pa;
        set_0(_p, 3)
    });
    assert!(((o.inner.a) == (3)));
    let mut pi: *mut i32 = (&mut o.items[(1) as usize].a as *mut i32);
    (*pi) = 4;
    assert!(((o.items[(1) as usize].a) == (4)));
    assert!(((&mut o.items[(0) as usize].a as *mut i32) != (pi)));
    let mut name: *mut libc::c_char = o.inner.name.as_mut_ptr();
    let mut i: i32 = 0;
    'loop_: while ((i) < (3)) {
        (*name.offset((i) as isize)) = (((('a' as libc::c_char) as i32) + (i)) as libc::c_char);
        i.prefix_inc();
    }
    assert!(((libc::strlen((o.inner.name.as_mut_ptr()).cast_const())) == (3_usize)));
    assert!(
        ((name.offset((3) as isize)) == (&mut o.inner.name[(3) as usize] as *mut libc::c_char))
    );
    o.cursor = (&mut o.buf[(1) as usize] as *mut i32);
    (*o.cursor) = 5;
    (*o.cursor.postfix_inc()) += 1;
    assert!(
        ((o.buf[(1) as usize]) == (6)) && ((o.cursor) == (&mut o.buf[(2) as usize] as *mut i32))
    );
    o.x = 0;
    let mut first: i32 = (unsafe { Outer::next(&mut o) });
    assert!(((first) == (0)) && ((o.x) == (1)));
    o.x = ((o.inner.a) + (unsafe { bump_1((&mut o as *mut Outer)) }));
    assert!(((o.x) == (4)));
    (unsafe {
        let _k: i32 = o.inner.a;
        Outer::push(&mut o, _k)
    });
    assert!(((o.v.len()) == (1_usize)) && ((o.v[(0_usize)]) == (7)));
    assert!(((unsafe { Outer::sum(&o,) }) == (7)));
    let mut y: i32 = 0;
    {
        if ::std::mem::size_of::<i32>() != 0 {
            ::std::ptr::copy_nonoverlapping(
                ((&mut o.items[(1) as usize].a as *mut i32) as *const ::libc::c_void),
                ((&mut o.items[(2) as usize].a as *mut i32) as *mut ::libc::c_void),
                ::std::mem::size_of::<i32>() as usize,
            )
        }
        ((&mut o.items[(2) as usize].a as *mut i32) as *mut ::libc::c_void)
    };
    {
        if ::std::mem::size_of::<i32>() != 0 {
            ::std::ptr::copy_nonoverlapping(
                ((&mut o.items[(2) as usize].a as *mut i32) as *const ::libc::c_void),
                ((&mut y as *mut i32) as *mut ::libc::c_void),
                ::std::mem::size_of::<i32>() as usize,
            )
        }
        ((&mut y as *mut i32) as *mut ::libc::c_void)
    };
    assert!(((y) == (4)));
    let mut bytes: [u8; 8] = [0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8];
    let mut view: *mut Header = (bytes.as_mut_ptr() as *mut Header);
    (*view).tag = 16909060;
    assert!(((bytes[(0) as usize] as i32) == (4)) || ((bytes[(3) as usize] as i32) == (4)));
    (unsafe { set_0((&mut (*view).tag as *mut i32), 0) });
    assert!(((bytes[(0) as usize] as i32) == (0)) && ((bytes[(3) as usize] as i32) == (0)));
    (*view).size = 1_i16;
    assert!((((bytes[(4) as usize] as i32) + (bytes[(5) as usize] as i32)) == (1)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
