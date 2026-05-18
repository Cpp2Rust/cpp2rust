extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fatorial_0(n: i32) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    if ((*n.borrow()) == 0) {
        return 1;
    }
    return ((*n.borrow())
        * ({
            let _n: i32 = ((*n.borrow()) - 1);
            fatorial_0(_n)
        }));
}
pub fn fatorial_by_ref_1(n: Ptr<i32>) {
    if ((n.read()) == 1) {
        {
            let __ptr = n.clone();
            let __tmp = __ptr.read() * 1;
            __ptr.write(__tmp)
        };
        return;
    }
    let n_1: Value<i32> = Rc::new(RefCell::new(((n.read()) - 1)));
    ({
        let _n: Ptr<i32> = n_1.as_pointer();
        fatorial_by_ref_1(_n)
    });
    let __rhs = (*n_1.borrow());
    {
        let __ptr = n.clone();
        let __tmp = __ptr.read() * __rhs;
        __ptr.write(__tmp)
    };
}
pub fn fatorial_by_ptr_2(n: Ptr<i32>) {
    let n: Value<Ptr<i32>> = Rc::new(RefCell::new(n));
    if (((*n.borrow()).read()) == 1) {
        {
            let __ptr = (*n.borrow()).clone();
            let __tmp = __ptr.read() * 1;
            __ptr.write(__tmp)
        };
        return;
    }
    let n_1: Value<i32> = Rc::new(RefCell::new((((*n.borrow()).read()) - 1)));
    ({
        let _n: Ptr<i32> = (n_1.as_pointer());
        fatorial_by_ptr_2(_n)
    });
    let __rhs = (*n_1.borrow());
    {
        let __ptr = (*n.borrow()).clone();
        let __tmp = __ptr.read() * __rhs;
        __ptr.write(__tmp)
    };
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let n1: Value<i32> = Rc::new(RefCell::new(2));
    ({
        let _n: Ptr<i32> = (n1.as_pointer());
        fatorial_by_ptr_2(_n)
    });
    let n: Value<i32> = Rc::new(RefCell::new(((*n1.borrow()) + 1)));
    ({
        let _n: Ptr<i32> = n.as_pointer();
        fatorial_by_ref_1(_n)
    });
    return ({
        let _n: i32 = (*n.borrow());
        fatorial_0(_n)
    });
}
