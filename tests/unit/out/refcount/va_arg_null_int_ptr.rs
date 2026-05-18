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
    let result: Value<i32> = Rc::new(RefCell::new(-1_i32));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((((*i.borrow()) < (*count.borrow())) as i32) != 0) {
        let p: Value<Ptr<i32>> =
            Rc::new(RefCell::new(((*ap.borrow_mut()).arg::<Ptr<i32>>()).clone()));
        if (((!((*p.borrow()).is_null())) as i32) != 0) {
            let __rhs = ((*p.borrow()).read());
            (*result.borrow_mut()) = __rhs;
            break;
        }
        (*i.borrow_mut()).postfix_inc();
    }
    return (*result.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(42));
    assert!(
        (((({
            let _count: i32 = 2;
            first_nonnull_0(
                _count,
                &[(AnyPtr::default()).into(), (x.as_pointer()).into()],
            )
        }) == 42) as i32)
            != 0)
    );
    assert!(
        (((({
            let _count: i32 = 3;
            first_nonnull_0(
                _count,
                &[
                    (AnyPtr::default()).into(),
                    (AnyPtr::default()).into(),
                    (x.as_pointer()).into(),
                ],
            )
        }) == 42) as i32)
            != 0)
    );
    assert!(
        (((({
            let _count: i32 = 1;
            first_nonnull_0(_count, &[(AnyPtr::default()).into()])
        }) == -1_i32) as i32)
            != 0)
    );
    return 0;
}
