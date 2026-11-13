extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn deref_0(mut p: Ptr<i32>) -> i32 {
    return (p.read());
}
pub fn strlen_1(mut s: Ptr<i8>) -> i32 {
    let mut c: i32 = 0;
    'loop_: while ((s.postfix_inc().read()) != 0) {
        c.prefix_inc();
    }
    return c;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct Row {
    #[offset(0)]
    #[byte_size(12)]
    pub a: Box<[i32]>,
}
impl Default for Row {
    fn default() -> Self {
        {
            Row {
                a: (0..3).map(|_| 0_i32).collect::<Box<[i32]>>(),
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2])));
    let s: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        ('a' as i8),
        ('b' as i8),
        ('c' as i8),
        ('\0' as i8),
    ])));
    assert!(((({ deref_0(a.decay(),) }) + ({ strlen_1(s.decay(),) })) == 4));
    let m: Value<Box<[Box<[i32]>]>> = Rc::new(RefCell::new(Box::new([
        Box::new([1, 2, 3]),
        Box::new([4, 5, 6]),
    ])));
    let r: Value<Row> = Rc::new(RefCell::new(Row {
        a: Box::new([7, 8, 9]),
    }));
    let mut row: Ptr<Box<[i32]>> = (m.decay().offset(1));
    let mut field: Ptr<Box<[i32]>> = (field_ptr!(r.as_pointer(), a));
    let mut whole: Ptr<Box<[i32]>> = (m.decay().offset(0));
    assert!(
        (((row.read())[(2) as usize] == 6) && ((field.read())[(1) as usize] == 8))
            && ((whole.read())[(0) as usize] == 1)
    );
    assert!(
        (({ row.decay() } == { m.decay().offset(1).decay() }) && ((field.decay().read()) == 7))
            && ((elem!((array_field_ptr!(r.as_pointer(), a)), 2).read()) == 9)
    );
    let mut v: AnyPtr = (array_field_ptr!(r.as_pointer(), a)).to_any();
    assert!(
        ({ (v).clone() } == { (array_field_ptr!(r.as_pointer(), a)).to_any() })
            && ({ (v).clone() } != { (row.decay()).to_any() })
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
