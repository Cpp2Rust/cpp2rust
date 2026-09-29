extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn call_with_ulong_0(g: FnPtr<fn(u64) -> u64>) -> u64 {
    let g: Value<FnPtr<fn(u64) -> u64>> = Rc::new(RefCell::new(g));
    return ({ (*g.borrow()).call(3_u64) }).wrapping_add(1_u64);
}
pub fn same_type_1(a: u64) -> u64 {
    let a: Value<u64> = Rc::new(RefCell::new(a));
    return (*a.borrow());
}
pub fn via_size_t_param_2(b: usize) -> u64 {
    let b: Value<usize> = Rc::new(RefCell::new(b));
    return ((*b.borrow()) as u64);
}
pub fn via_size_t_return_3(b: u64) -> usize {
    let b: Value<u64> = Rc::new(RefCell::new(b));
    return ((*b.borrow()) as usize);
}
#[derive(VaArg, FnPtrArg, Default)]
pub struct pair {
    pub a: Value<i32>,
    pub b: Value<i32>,
}
impl Clone for pair {
    fn clone(&self) -> Self {
        Self {
            a: Rc::new(RefCell::new((*self.a.borrow()).clone())),
            b: Rc::new(RefCell::new((*self.b.borrow()).clone())),
        }
    }
}
impl ByteRepr for pair {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a.borrow()).to_bytes(&mut buf[0..4]);
        (*self.b.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            b: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
pub fn pair_scaled_4(p: pair, n: usize) -> u64 {
    let p: Value<pair> = Rc::new(RefCell::new(p));
    let n: Value<usize> = Rc::new(RefCell::new(n));
    return ((((*(*p.borrow()).a.borrow()) as usize).wrapping_mul((*n.borrow())))
        .wrapping_add(((*(*p.borrow()).b.borrow()) as usize)) as u64);
}
pub fn make_pair_5(n: usize) -> pair {
    let n: Value<usize> = Rc::new(RefCell::new(n));
    let p: Value<pair> = Rc::new(RefCell::new(pair {
        a: Rc::new(RefCell::new(((*n.borrow()) as i32))),
        b: Rc::new(RefCell::new((((*n.borrow()) as i32) * 2))),
    }));
    return (*p.borrow()).clone();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (((({ call_with_ulong_0(FnPtr::<fn(u64) -> u64>::new(same_type_1),) }) == 4_u64) as i32)
            != 0)
    );
    assert!(
        (((({
            call_with_ulong_0(
                FnPtr::<fn(usize) -> u64>::new(via_size_t_param_2).cast::<fn(u64) -> u64>(),
            )
        }) == 4_u64) as i32)
            != 0)
    );
    assert!(
        (((({
            call_with_ulong_0(
                FnPtr::<fn(u64) -> usize>::new(via_size_t_return_3).cast::<fn(u64) -> u64>(),
            )
        }) == 4_u64) as i32)
            != 0)
    );
    let original: Value<FnPtr<fn(usize) -> u64>> = Rc::new(RefCell::new(
        FnPtr::<fn(usize) -> u64>::new(via_size_t_param_2),
    ));
    let adapted: Value<FnPtr<fn(u64) -> u64>> =
        Rc::new(RefCell::new((*original.borrow()).cast::<fn(u64) -> u64>()));
    let back: Value<FnPtr<fn(usize) -> u64>> =
        Rc::new(RefCell::new((*adapted.borrow()).cast::<fn(usize) -> u64>()));
    assert!(
        ((({
            let _lhs = (*back.borrow()).clone();
            _lhs == (*original.borrow()).clone()
        }) as i32)
            != 0)
    );
    assert!((((({ (*back.borrow()).call(5_usize,) }) == 5_u64) as i32) != 0));
    let scaled: Value<FnPtr<fn(pair, u64) -> u64>> = Rc::new(RefCell::new(
        FnPtr::<fn(pair, usize) -> u64>::new(pair_scaled_4).cast::<fn(pair, u64) -> u64>(),
    ));
    let p: Value<pair> = Rc::new(RefCell::new(pair {
        a: Rc::new(RefCell::new(3)),
        b: Rc::new(RefCell::new(4)),
    }));
    assert!(
        (((({ (*scaled.borrow()).call((*p.borrow()).clone(), 10_u64,) }) == 34_u64) as i32) != 0)
    );
    let make: Value<FnPtr<fn(u64) -> pair>> = Rc::new(RefCell::new(
        FnPtr::<fn(usize) -> pair>::new(make_pair_5).cast::<fn(u64) -> pair>(),
    ));
    let q: Value<pair> = Rc::new(RefCell::new(({ (*make.borrow()).call(5_u64) })));
    assert!(((((*(*q.borrow()).a.borrow()) == 5) as i32) != 0));
    assert!(((((*(*q.borrow()).b.borrow()) == 10) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
