extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
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
    let y: Value<i32> = Rc::new(RefCell::new((({ foo_1(x.as_pointer()) }).read())));
    let z: Ptr<i32> = ({ foo_1(x.as_pointer()) });
    return {
        let _lhs = {
            let _lhs =
                ((({ foo_1(x.as_pointer()) }).read()) + (({ foo_1(y.as_pointer()) }).read()));
            _lhs + (({ foo_1((z).clone()) }).read())
        };
        _lhs + ({ bar_0() })
    };
}
