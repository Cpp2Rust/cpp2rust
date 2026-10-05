extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn call_with_ulong_0(mut g: FnPtr<fn(u64) -> u64>) -> u64 {
    return ({ g.call(3_u64) }).wrapping_add(1_u64);
}
pub fn same_type_1(mut a: u64) -> u64 {
    return a;
}
pub fn via_size_t_param_2(mut b: usize) -> u64 {
    return (b as u64);
}
pub fn via_size_t_return_3(mut b: u64) -> usize {
    return (b as usize);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct pair {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: i32,
}
pub fn pair_scaled_4(p: pair, mut n: usize) -> u64 {
    let p: Value<pair> = Rc::new(RefCell::new(p));
    return ((({ (*p.borrow()).a } as usize).wrapping_mul(n))
        .wrapping_add(({ (*p.borrow()).b } as usize)) as u64);
}
pub fn make_pair_5(mut n: usize) -> pair {
    let p: Value<pair> = Rc::new(RefCell::new(pair {
        a: (n as i32),
        b: ((n as i32) * 2),
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
                (FnPtr::<fn(usize) -> u64>::new(via_size_t_param_2)).cast::<fn(u64) -> u64>(),
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
    let mut original: FnPtr<fn(usize) -> u64> = FnPtr::<fn(usize) -> u64>::new(via_size_t_param_2);
    let mut adapted: FnPtr<fn(u64) -> u64> = original.cast::<fn(u64) -> u64>();
    let mut back: FnPtr<fn(usize) -> u64> = adapted.cast::<fn(usize) -> u64>();
    assert!(((({ (back).clone() } == { (original).clone() }) as i32) != 0));
    assert!((((({ back.call(5_usize,) }) == 5_u64) as i32) != 0));
    let mut scaled: FnPtr<fn(pair, u64) -> u64> =
        FnPtr::<fn(pair, usize) -> u64>::new(pair_scaled_4).cast::<fn(pair, u64) -> u64>();
    let p: Value<pair> = Rc::new(RefCell::new(pair { a: 3, b: 4 }));
    assert!((((({ scaled.call((*p.borrow()).clone(), 10_u64,) }) == 34_u64) as i32) != 0));
    let mut make: FnPtr<fn(u64) -> pair> =
        FnPtr::<fn(usize) -> pair>::new(make_pair_5).cast::<fn(u64) -> pair>();
    let q: Value<pair> = Rc::new(RefCell::new(({ make.call(5_u64) })));
    assert!(((({ (*q.borrow()).a } == 5) as i32) != 0));
    assert!(((({ (*q.borrow()).b } == 10) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
