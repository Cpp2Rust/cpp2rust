extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fill_row_0(row: Ptr<u8>, c: u8) {
    let row: Value<Ptr<u8>> = Rc::new(RefCell::new(row));
    let c: Value<u8> = Rc::new(RefCell::new(c));
    let __rhs = (*c.borrow());
    (*row.borrow()).offset((0) as isize).write(__rhs);
    (*row.borrow())
        .offset((1) as isize)
        .write((('\0' as i32) as u8));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let grid: Value<Box<[Value<Box<[u8]>>]>> = Rc::new(RefCell::new(
        (0..3)
            .map(|_| {
                Rc::new(RefCell::new(
                    (0..6).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
                ))
            })
            .collect::<Box<[Value<Box<[u8]>>]>>(),
    ));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((((*i.borrow()) < 3) as i32) != 0) {
        ({
            let _row: Ptr<u8> = (((grid.as_pointer() as Ptr<Value<Box<[u8]>>>)
                .offset((*i.borrow()))
                .read()
                .as_pointer()) as Ptr<u8>);
            let _c: u8 = ((('a' as i32) + (*i.borrow())) as u8);
            fill_row_0(_row, _c)
        });
        (*i.borrow_mut()).postfix_inc();
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
    (*grid.borrow())[(2) as usize].borrow_mut()[(5) as usize] = (('z' as i32) as u8);
    assert!(
        (((((*grid.borrow())[(2) as usize].borrow()[(5) as usize] as i32) == ('z' as i32)) as i32)
            != 0)
    );
    return 0;
}
