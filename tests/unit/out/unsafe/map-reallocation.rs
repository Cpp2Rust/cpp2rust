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
    let mut m: UnsafeMap<i32, (i32, i32)> = UnsafeMap::new();
    (*&mut (*m.get_or_insert_with(sentinel.clone(), || (sentinel, <i32>::default()))).1) = sentinel;
    let mut it: UnsafeBidirectionalIterator<(i32, i32)> = m.find(&sentinel);
    let mut p: *mut i32 = (&mut (*it.iter_deref()).1 as *mut i32);
    assert!(
        (((*it.iter_deref()).1) == (sentinel))
            && (!(c"iterator does not have correct value before insert".as_ptr()).is_null())
    );
    assert!(
        ((*p) == (sentinel))
            && (!(c"pointer does not have correct value before insert".as_ptr()).is_null())
    );
    let mut i: i32 = 0;
    'loop_: while ((i) < (sentinel)) {
        (*&mut (*m.get_or_insert_with(i.clone(), || (i, <i32>::default()))).1) = i;
        i.prefix_inc();
    }
    let mut i: i32 = ((sentinel) + (1));
    'loop_: while ((i) <= (N)) {
        (*&mut (*m.get_or_insert_with(i.clone(), || (i, <i32>::default()))).1) = i;
        i.prefix_inc();
    }
    assert!(
        (((*it.iter_deref()).1) != (0))
            && (!(c"in refcount, iterator points to index 0 instead of sentinel".as_ptr())
                .is_null())
    );
    assert!(
        (((*it.iter_deref()).1) == (sentinel))
            && (!(c"iterator does not have correct value after insert".as_ptr()).is_null())
    );
    assert!(
        ((*p) == (sentinel))
            && (!(c"pointer does not have correct value after insert".as_ptr()).is_null())
    );
    (*it.iter_deref()).1 = 57005;
    assert!(
        ((*&mut (*m.get_or_insert_with(sentinel.clone(), || (sentinel, <i32>::default()))).1)
            == (57005))
    );
    assert!(((*p) == (57005)));
    assert!(((m.len()) == ((((N) + (1)) as u32) as usize)));
    let mut prev: i32 = -1_i32;
    'loop_: for pair in IterRange::new(m.begin(), m.end()) {
        let pair: *mut (i32, i32) = &mut (*pair.iter_deref());
        assert!((((*pair).0) > (prev)));
        prev = (*pair).0;
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
