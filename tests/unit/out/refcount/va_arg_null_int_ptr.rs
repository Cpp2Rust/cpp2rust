extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn first_nonnull_0(count: i32, __args: &[VaArg]) -> i32 {
    let count: Value<i32> = Rc::new(RefCell::new(count));
    let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
    (*ap.borrow_mut()) = VaList::new(__args);
    let mut result: i32 = -1_i32;
    let mut i: i32 = 0;
    'loop_: while (((i < (*count.borrow())) as i32) != 0) {
        let mut p: Ptr<i32> = (*ap.borrow_mut()).arg::<Ptr<i32>>();
        if (((!((p).is_null())) as i32) != 0) {
            result = { (p.read()) };
            break;
        }
        i.postfix_inc();
    }
    return result;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(42));
    assert!(
        (((({ first_nonnull_0(2, &[(AnyPtr::default()).into(), (x.as_pointer()).into(),]) }) == 42)
            as i32)
            != 0)
    );
    assert!(
        (((({
            first_nonnull_0(
                3,
                &[
                    (AnyPtr::default()).into(),
                    (AnyPtr::default()).into(),
                    (x.as_pointer()).into(),
                ],
            )
        }) == 42) as i32)
            != 0)
    );
    assert!((((({ first_nonnull_0(1, &[(AnyPtr::default()).into(),]) }) == -1_i32) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
