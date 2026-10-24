extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct Holder {
    pub ref_: *mut Vec<i32>,
    pub vec_: Vec<i32>,
}
impl Holder {
    pub unsafe fn move_from(_a0: *mut Holder) -> Self {
        let mut this = Self {
            ref_: (*_a0).ref_,
            vec_: std::mem::take(&mut (*_a0).vec_),
        };
        this
    }
}
pub unsafe fn pick_0(a: *mut Vec<i32>, b: *mut Vec<i32>, mut first: bool) -> *mut Vec<i32> {
    return if first { (a) } else { (b) };
}
pub unsafe fn sum_1(v: *const Vec<i32>) -> i32 {
    let mut total: i32 = 0;
    let mut it: UnsafeRandomAccessIterator<i32> = (*v).begin();
    'loop_: while it != (*v).end() {
        total += (*it.iter_deref());
        it.iter_inc();
    }
    return ((total)
        + (*{
            let __idx = 0_usize;
            if __idx as usize >= (*v).len() {
                panic!("out of bounds access")
            } else {
                (*v).as_ptr().add(__idx as usize)
            }
        }));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v: Vec<i32> = Vec::from([1, 2, 3]);
    let mut h: Holder = Holder {
        ref_: &mut v,
        vec_: Vec::from([4, 5]),
    };
    assert!(((*(*h.ref_).as_mut_ptr()) == (1)));
    (*((*h.ref_).first_mut().unwrap())) = 9;
    assert!(((v[(0_usize)]) == (9)));
    (*((*h.ref_).first_mut().unwrap())) = 1;
    assert!(((*((h.vec_).last_mut().unwrap())) == (5)));
    let mut p: *mut Vec<i32> = (&mut h.vec_ as *mut Vec<i32>);
    assert!(((*(*p).as_mut_ptr()) == (4)));
    (*((*(unsafe { pick_0(&mut v, &mut h.vec_, false) }))
        .first_mut()
        .unwrap())) = 7;
    assert!(((h.vec_[(0_usize)]) == (7)));
    let mut m: UnsafeMap<i32, (i32, Vec<i32>)> = UnsafeMap::new();
    {
        let __a1 = 8;
        (*m.subscript_move(&mut 1, |__k| cpp_move_unsafe!(__k))).push(__a1)
    };
    assert!(
        ((*(*m.subscript_move(&mut 1, |__k| cpp_move_unsafe!(__k)))
            .begin()
            .iter_deref())
            == (8))
    );
    let c: *const Vec<i32> = &v;
    assert!(
        ((*{
            let __idx = 2_usize;
            if __idx as usize >= (*c).len() {
                panic!("out of bounds access")
            } else {
                (*c).as_ptr().add(__idx as usize)
            }
        }) == (3))
    );
    assert!(((unsafe { sum_1(&v,) }) == (7)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
