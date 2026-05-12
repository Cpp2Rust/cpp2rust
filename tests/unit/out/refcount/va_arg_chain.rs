extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn extract_nth_0(n: i32, ap: VaList) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let ap: Value<VaList> = Rc::new(RefCell::new(ap));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((((*i.borrow()) < (*n.borrow())) as i32) != 0) {
        (*ap.borrow_mut()).arg::<i32>();
        (*i.borrow_mut()).postfix_inc();
    }
    return ((*ap.borrow_mut()).arg::<i32>()).clone();
}
pub fn middle_layer_1(n: i32, ap: VaList) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let ap: Value<VaList> = Rc::new(RefCell::new(ap));
    return ({
        let _n: i32 = (*n.borrow());
        let _ap: VaList = (*ap.borrow()).clone();
        extract_nth_0(_n, _ap)
    });
}
pub fn top_level_2(n: i32, __args: &[VaArg]) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
    (*ap.borrow_mut()) = VaList::new(__args);
    let result: Value<i32> = Rc::new(RefCell::new(
        ({
            let _n: i32 = (*n.borrow());
            let _ap: VaList = (*ap.borrow()).clone();
            middle_layer_1(_n, _ap)
        }),
    ));
    return (*result.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (((({
            let _n: i32 = 2;
            top_level_2(_n, &[100.into(), 200.into(), 300.into(), 400.into()])
        }) == 300) as i32)
            != 0)
    );
    assert!(
        (((({
            let _n: i32 = 0;
            top_level_2(_n, &[42.into(), 99.into()])
        }) == 42) as i32)
            != 0)
    );
    assert!(
        (((({
            let _n: i32 = 3;
            top_level_2(_n, &[1.into(), 2.into(), 3.into(), 4.into()])
        }) == 4) as i32)
            != 0)
    );
    return 0;
}
