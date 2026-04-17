extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn bar_0() -> i32 {
    return 1;
}
pub fn foo_1(x: Ptr<i32>) -> Ptr<i32> {
    return (x).clone();
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(5));
    let y: Value<i32> = Rc::new(RefCell::new(
        (({
            let _x: Ptr<i32> = x.as_pointer();
            foo_1(_x)
        })
        .read()),
    ));
    let z: Ptr<i32> = ({
        let _x: Ptr<i32> = x.as_pointer();
        foo_1(_x)
    });
    return {
        let _lhs = {
            let _lhs = ((({
                let _x: Ptr<i32> = x.as_pointer();
                foo_1(_x)
            })
            .read())
                + (({
                    let _x: Ptr<i32> = y.as_pointer();
                    foo_1(_x)
                })
                .read()));
            _lhs + (({
                let _x: Ptr<i32> = (z).clone();
                foo_1(_x)
            })
            .read())
        };
        _lhs + ({ bar_0() })
    };
}
