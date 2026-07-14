extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn logf_impl_0(fmt: Ptr<u8>, ap: VaList) -> i32 {
    let fmt: Value<Ptr<u8>> = Rc::new(RefCell::new(fmt));
    let ap: Value<VaList> = Rc::new(RefCell::new(ap));
    (*fmt.borrow()).clone();
    return {
        let _lhs = ((*ap.borrow_mut()).arg::<i32>()).clone();
        _lhs + ((*ap.borrow_mut()).arg::<i32>()).clone()
    };
}
pub fn logf_1(fmt: Ptr<u8>, __args: &[VaArg]) -> i32 {
    let fmt: Value<Ptr<u8>> = Rc::new(RefCell::new(fmt));
    let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
    (*ap.borrow_mut()) = VaList::new(__args);
    let result: Value<i32> = Rc::new(RefCell::new(
        ({ logf_impl_0((*fmt.borrow()).clone(), (*ap.borrow()).clone()) }),
    ));
    return (*result.borrow());
}
pub fn lenf_2(fmt: Ptr<u8>, __args: &[VaArg]) -> i32 {
    let fmt: Value<Ptr<u8>> = Rc::new(RefCell::new(fmt));
    let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
    (*ap.borrow_mut()) = VaList::new(__args);
    let s: Value<Ptr<u8>> = Rc::new(RefCell::new(((*ap.borrow_mut()).arg::<Ptr<u8>>()).clone()));
    let result: Value<i32> = Rc::new(RefCell::new(
        ({
            let mut __p = (*s.borrow()).clone();
            let mut __i: usize = 0;
            while __p.read() != 0 {
                __p += 1;
                __i += 1;
            }
            __i
        } as i32),
    ));
    return (*result.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let dummy: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(b"dummy")));
    assert!(
        (((({
            logf_1(
                Ptr::from_string_literal(b"hello %d %d"),
                &[
                    (10).into(),
                    ({
                        let mut __p = (*dummy.borrow()).clone();
                        let mut __i: usize = 0;
                        while __p.read() != 0 {
                            __p += 1;
                            __i += 1;
                        }
                        __i
                    })
                    .into(),
                ],
            )
        }) == 15) as i32)
            != 0)
    );
    assert!(
        (((({
            logf_1(
                Ptr::from_string_literal(b"x %d %d"),
                &[(1).into(), (2).into()],
            )
        }) == 3) as i32)
            != 0)
    );
    assert!(
        (((({
            lenf_2(
                Ptr::from_string_literal(b"%s"),
                &[((*dummy.borrow()).clone()).into()],
            )
        }) == 5) as i32)
            != 0)
    );
    assert!(
        (((({
            lenf_2(
                Ptr::from_string_literal(b"%s"),
                &[
                    (if ((((*dummy.borrow()).offset((0) as isize).read()) as i32) != 0) {
                        (*dummy.borrow()).clone()
                    } else {
                        Ptr::from_string_literal(b"")
                    })
                    .into(),
                ],
            )
        }) == 5) as i32)
            != 0)
    );
    return 0;
}
