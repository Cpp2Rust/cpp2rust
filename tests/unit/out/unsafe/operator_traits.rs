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
pub struct Lt {
    pub v: i32,
}
impl Lt {
    pub unsafe fn operator_lt(&self, o: *const Lt) -> bool {
        return ((self.v) < ((*o).v));
    }
}
impl std::cmp::Ord for Lt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if Lt::operator_lt(self, other as *const Lt) {
                std::cmp::Ordering::Less
            } else if Lt::operator_lt(other, self as *const Lt) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Lt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Lt {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(Lt::operator_lt(self, other as *const Lt))
                && !(Lt::operator_lt(other, self as *const Lt))
        }
    }
}
impl std::cmp::Eq for Lt {}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Eq {
    pub v: i32,
}
impl Eq {
    pub unsafe fn operator_eq(&self, o: *const Eq) -> bool {
        return ((self.v) == ((*o).v));
    }
}
impl std::cmp::PartialEq for Eq {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Eq::operator_eq(self, other as *const Eq) }
    }
}
impl std::cmp::Eq for Eq {}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Cmp {
    pub v: i32,
}
impl Cmp {
    pub unsafe fn operator_cmp(&self, o: *const Cmp) -> std::cmp::Ordering {
        return (self.v).cmp(&((*o).v));
    }
    pub unsafe fn operator_eq(&self, o: *const Cmp) -> bool {
        return ((self.v) == ((*o).v));
    }
}
impl std::cmp::Ord for Cmp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { Cmp::operator_cmp(self, other as *const Cmp) }
    }
}
impl std::cmp::PartialOrd for Cmp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Cmp {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Cmp::operator_eq(self, other as *const Cmp) }
    }
}
impl std::cmp::Eq for Cmp {}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Free {
    pub v: i32,
}
impl std::cmp::Ord for Free {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_0(self as *const Free, other as *const Free) {
                std::cmp::Ordering::Less
            } else if operator_lt_0(other as *const Free, self as *const Free) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Free {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Free {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_1(self as *const Free, other as *const Free) }
    }
}
impl std::cmp::Eq for Free {}
pub unsafe fn operator_lt_0(a: *const Free, b: *const Free) -> bool {
    return (((*a).v) < ((*b).v));
}
pub unsafe fn operator_eq_1(a: *const Free, b: *const Free) -> bool {
    return (((*a).v) == ((*b).v));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut lts: Vec<Lt> = vec![Lt { v: 3 }, Lt { v: 1 }, Lt { v: 2 }];
    {
        let len = lts
            .as_mut_ptr()
            .add(lts.len())
            .offset_from(lts.as_mut_ptr()) as usize;
        ::std::slice::from_raw_parts_mut(lts.as_mut_ptr(), len).sort()
    };
    assert!(
        (((lts[(0_usize)].v) == (1)) && ((lts[(1_usize)].v) == (2))) && ((lts[(2_usize)].v) == (3))
    );
    let mut eqs: Vec<Eq> = vec![Eq { v: 1 }, Eq { v: 2 }, Eq { v: 3 }];
    let mut two: Eq = Eq { v: 2 };
    let mut nine: Eq = Eq { v: 9 };
    assert!(
        (({
            let mut it = eqs.as_mut_ptr();
            while it != eqs.as_mut_ptr().add(eqs.len()) && *it != two {
                it = it.add(1);
            }
            it
        }
        .offset_from(eqs.as_mut_ptr()))
            == (1_i64))
    );
    assert!(
        {
            let mut it = eqs.as_mut_ptr();
            while it != eqs.as_mut_ptr().add(eqs.len()) && *it != nine {
                it = it.add(1);
            }
            it
        } == eqs.as_mut_ptr().add(eqs.len())
    );
    let mut cmps: Vec<Cmp> = vec![Cmp { v: 3 }, Cmp { v: 1 }, Cmp { v: 2 }];
    {
        let len = cmps
            .as_mut_ptr()
            .add(cmps.len())
            .offset_from(cmps.as_mut_ptr()) as usize;
        ::std::slice::from_raw_parts_mut(cmps.as_mut_ptr(), len).sort()
    };
    assert!(((cmps[(0_usize)].v) == (1)) && ((cmps[(2_usize)].v) == (3)));
    let mut three: Cmp = Cmp { v: 3 };
    assert!(
        (({
            let mut it = cmps.as_mut_ptr();
            while it != cmps.as_mut_ptr().add(cmps.len()) && *it != three {
                it = it.add(1);
            }
            it
        }
        .offset_from(cmps.as_mut_ptr()))
            == (2_i64))
    );
    let mut frees: Vec<Free> = vec![Free { v: 2 }, Free { v: 1 }];
    {
        let len = frees
            .as_mut_ptr()
            .add(frees.len())
            .offset_from(frees.as_mut_ptr()) as usize;
        ::std::slice::from_raw_parts_mut(frees.as_mut_ptr(), len).sort()
    };
    assert!(((frees[(0_usize)].v) == (1)));
    let mut ftwo: Free = Free { v: 2 };
    assert!(
        (({
            let mut it = frees.as_mut_ptr();
            while it != frees.as_mut_ptr().add(frees.len()) && *it != ftwo {
                it = it.add(1);
            }
            it
        }
        .offset_from(frees.as_mut_ptr()))
            == (1_i64))
    );
    let mut m: BTreeMap<Lt, Box<i32>> = BTreeMap::new();
    (*m.entry(Lt { v: 2 }).or_default().as_mut()) = 20;
    (*m.entry(Lt { v: 1 }).or_default().as_mut()) = 10;
    assert!(((*UnsafeMapIterator::begin(&m as *const BTreeMap<Lt, Box<i32>>).second()) == (10)));
    assert!(((*m.entry(Lt { v: 2 }).or_default().as_mut()) == (20)));
    return 0;
}
