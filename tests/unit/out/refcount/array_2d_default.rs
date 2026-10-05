extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fill_row_0(mut row: Ptr<i8>, mut c: i8) {
    elem!(row, 0).write({ c });
    elem!(row, 1).write((('\0' as i32) as i8));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let grid: Value<Box<[Value<Box<[i8]>>]>> = Rc::new(RefCell::new(
        (0..3)
            .map(|_| Rc::new(RefCell::new((0..6).map(|_| 0_i8).collect::<Box<[i8]>>())))
            .collect::<Box<[Value<Box<[i8]>>]>>(),
    ));
    let mut i: i32 = 0;
    'loop_: while (((i < 3) as i32) != 0) {
        ({
            let _row: Ptr<i8> = (((grid.as_pointer() as Ptr<Value<Box<[i8]>>>)
                .offset(i)
                .read()
                .as_pointer()) as Ptr<i8>);
            let _c: i8 = ((('a' as i32) + i) as i8);
            fill_row_0(_row, _c)
        });
        i.postfix_inc();
    }
    assert!(
        (((((*grid.borrow())[(0) as usize].borrow()[(0) as usize] as i32) == ('a' as i32)) as i32)
            != 0)
    );
    assert!(
        (((((*grid.borrow())[(1) as usize].borrow()[(0) as usize] as i32) == ('b' as i32)) as i32)
            != 0)
    );
    assert!(
        (((((*grid.borrow())[(2) as usize].borrow()[(0) as usize] as i32) == ('c' as i32)) as i32)
            != 0)
    );
    assert!(
        (((((*grid.borrow())[(1) as usize].borrow()[(1) as usize] as i32) == ('\0' as i32))
            as i32)
            != 0)
    );
    (*grid.borrow())[(2) as usize].borrow_mut()[(5) as usize] = (('z' as i32) as i8);
    assert!(
        (((((*grid.borrow())[(2) as usize].borrow()[(5) as usize] as i32) == ('z' as i32)) as i32)
            != 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
