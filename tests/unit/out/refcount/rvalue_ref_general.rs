extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let i1: Value<i32> = Rc::new(RefCell::new(3));
    let i2: Value<i32> = Rc::new(RefCell::new((*i1.borrow_mut())));
    assert!(((*i1.borrow()) == 3));
    assert!(((*i2.borrow()) == 3));
    let i3: Value<i32> = 40;
    {
        let _ptr = i3.clone();
        _ptr.write(_ptr.read() + 2)
    };
    assert!(((i3.read()) == 42));
    let i4: Value<i32> = (2 + 3);
    assert!(((i4.read()) == 5));
    let i5: Ptr<i32> = 40;
    let i6: Ptr<i32> = (i3).clone();
    let i7: Ptr<i32> = (i4).clone();
    assert!({
        let _lhs = (i6.read());
        _lhs == (i3.read())
    });
    assert!({
        let _lhs = (i7.read());
        _lhs == (i4.read())
    });
    let p1: Value<Ptr<i32>> = Rc::new(RefCell::new((i1.as_pointer())));
    let p2: Value<Ptr<i32>> = Rc::new(RefCell::new((i3).clone()));
    let p3: Value<Ptr<i32>> = Rc::new(RefCell::new((i6).clone()));
    assert!({
        let _lhs = ((*p1.borrow()).read());
        _lhs == (*i1.borrow())
    });
    assert!({
        let _lhs = ((*p2.borrow()).read());
        _lhs == (i3.read())
    });
    assert!({
        let _lhs = ((*p3.borrow()).read());
        _lhs == (i6.read())
    });
    return 0;
}
