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
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let N: i32 = 10000;
    let sentinel: i32 = ((N) / (2));
    let mut m: BTreeMap<i32, Box<(i32, i32)>> = BTreeMap::new();
    (*&mut m
        .entry(sentinel.clone())
        .or_insert_with(|| Box::new((sentinel, i32::default())))
        .1) = sentinel;
    let mut it: UnsafeMapIterator<i32, i32> =
        UnsafeMapIterator::find_key(&m as *const BTreeMap<i32, Box<(i32, i32)>>, &sentinel);
    let mut p: *mut i32 = (&mut *&raw mut (*it.deref()).1 as *mut i32);
    assert!(
        ((*&raw mut (*it.deref()).1) == (sentinel))
            && (!(c"iterator does not have correct value before insert".as_ptr()).is_null())
    );
    assert!(
        ((*p) == (sentinel))
            && (!(c"pointer does not have correct value before insert".as_ptr()).is_null())
    );
    let mut i: i32 = 0;
    'loop_: while ((i) < (sentinel)) {
        (*&mut m
            .entry(i.clone())
            .or_insert_with(|| Box::new((i, i32::default())))
            .1) = i;
        i.prefix_inc();
    }
    let mut i: i32 = ((sentinel) + (1));
    'loop_: while ((i) <= (N)) {
        (*&mut m
            .entry(i.clone())
            .or_insert_with(|| Box::new((i, i32::default())))
            .1) = i;
        i.prefix_inc();
    }
    assert!(
        ((*&raw mut (*it.deref()).1) != (0))
            && (!(c"in refcount, iterator points to index 0 instead of sentinel".as_ptr())
                .is_null())
    );
    assert!(
        ((*&raw mut (*it.deref()).1) == (sentinel))
            && (!(c"iterator does not have correct value after insert".as_ptr()).is_null())
    );
    assert!(
        ((*p) == (sentinel))
            && (!(c"pointer does not have correct value after insert".as_ptr()).is_null())
    );
    *&raw mut (*it.deref()).1 = 57005;
    assert!(
        ((*&mut m
            .entry(sentinel.clone())
            .or_insert_with(|| Box::new((sentinel, i32::default())))
            .1)
            == (57005))
    );
    assert!(((*p) == (57005)));
    assert!(((m.len()) == ((((N) + (1)) as u32) as usize)));
    let mut prev: i32 = -1_i32;
    'loop_: for __begin1 in UnsafeMapIterator::begin(&m as *const BTreeMap<i32, Box<(i32, i32)>>) {
        let pair: *mut (i32, i32) = &mut (*__begin1.deref());
        assert!((((*pair).0) > (prev)));
        prev = (*pair).0;
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
