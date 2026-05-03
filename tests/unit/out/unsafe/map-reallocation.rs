extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let N: i32 = 10000;
    let sentinel: i32 = ((N) / (2));
    let mut m: BTreeMap<i32, Box<i32>> = BTreeMap::new();
    (*m.entry(sentinel).or_default().as_mut()) = sentinel;
    let mut it: UnsafeMapIterator<i32, i32> =
        UnsafeMapIterator::find_key(&m as *const BTreeMap<i32, Box<i32>>, &sentinel);
    let mut p: *mut i32 = (&mut *it.second() as *mut i32);
    assert!(
        ((*it.second()) == (sentinel))
            && (!(b"iterator does not have correct value before insert\0".as_ptr()).is_null())
    );
    assert!(
        ((*p) == (sentinel))
            && (!(b"pointer does not have correct value before insert\0".as_ptr()).is_null())
    );
    let mut i: i32 = 0;
    'loop_: while ((i) < (sentinel)) {
        (*m.entry(i).or_default().as_mut()) = i;
        i.prefix_inc();
    }
    let mut i: i32 = ((sentinel) + (1));
    'loop_: while ((i) <= (N)) {
        (*m.entry(i).or_default().as_mut()) = i;
        i.prefix_inc();
    }
    assert!(
        ((*it.second()) != (0))
            && (!(b"in refcount, iterator points to index 0 instead of sentinel\0".as_ptr())
                .is_null())
    );
    assert!(
        ((*it.second()) == (sentinel))
            && (!(b"iterator does not have correct value after insert\0".as_ptr()).is_null())
    );
    assert!(
        ((*p) == (sentinel))
            && (!(b"pointer does not have correct value after insert\0".as_ptr()).is_null())
    );
    *it.second() = 57005;
    assert!(((*m.entry(sentinel).or_default().as_mut()) == (57005)));
    assert!(((*p) == (57005)));
    assert!(((m.len() as u64) == ((((N) + (1)) as u32) as u64)));
    let mut prev: i32 = -1_i32;
    'loop_: for pair in UnsafeMapIterator::begin(&m as *const BTreeMap<i32, Box<i32>>) {
        assert!(((*pair.first()) > (prev)));
        prev = *pair.first();
    }
    return 0;
}
