extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn len_0(s: Ptr<Box<[u8]>>) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((((s.to_strong().as_pointer() as Ptr<u8>)
        .offset((*n.borrow()) as isize)
        .read()) as i32)
        != (('\0' as u8) as i32))
    {
        (*n.borrow_mut()).prefix_inc();
    }
    return (*n.borrow());
}
pub fn sum_1(a: Ptr<Box<[i32]>>) -> i32 {
    return ((((a.to_strong().as_pointer() as Ptr<i32>)
        .offset((0) as isize)
        .read())
        + ((a.to_strong().as_pointer() as Ptr<i32>)
            .offset((1) as isize)
            .read()))
        + ((a.to_strong().as_pointer() as Ptr<i32>)
            .offset((2) as isize)
            .read()));
}
pub fn fill_2(a: Ptr<Box<[i32]>>, v: i32) {
    let v: Value<i32> = Rc::new(RefCell::new(v));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 3) {
        (a.to_strong().as_pointer() as Ptr<i32>)
            .offset((*i.borrow()) as isize)
            .write((*v.borrow()));
        (*i.borrow_mut()).prefix_inc();
    }
}
pub fn sum_twice_3(a: Ptr<Box<[i32]>>) -> i32 {
    return (({ sum_1(((a).clone() as Ptr<Box<[i32]>>)) })
        + ({ sum_1(((a).clone() as Ptr<Box<[i32]>>)) }));
}
pub fn fill_and_sum_4(a: Ptr<Box<[i32]>>, v: i32, out: Ptr<i32>) {
    let v: Value<i32> = Rc::new(RefCell::new(v));
    ({
        let _a: Ptr<Box<[i32]>> = ((a).clone() as Ptr<Box<[i32]>>);
        let _v: i32 = (*v.borrow());
        fill_2(_a, _v)
    });
    let __rhs = ({ sum_twice_3(((a).clone() as Ptr<Box<[i32]>>)) });
    out.write(__rhs);
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ len_0(Ptr::from_string_literal_array(b"beta"),) }) == 4));
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::from(*b"abcd\0")));
    assert!((({ len_0((buf.as_pointer() as Ptr<Box<[u8]>>),) }) == 4));
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
    assert!((({ sum_1((arr.as_pointer() as Ptr<Box<[i32]>>),) }) == 6));
    ({ fill_2((arr.as_pointer() as Ptr<Box<[i32]>>), 7) });
    assert!((({ sum_1((arr.as_pointer() as Ptr<Box<[i32]>>),) }) == 21));
    assert!((({ sum_twice_3((arr.as_pointer() as Ptr<Box<[i32]>>),) }) == 42));
    let out: Value<i32> = Rc::new(RefCell::new(0));
    ({ fill_and_sum_4((arr.as_pointer() as Ptr<Box<[i32]>>), 2, out.as_pointer()) });
    assert!(((*out.borrow()) == 12));
    assert!(((*arr.borrow())[(0) as usize] == 2));
    let lit: Ptr<Box<[u8]>> = Ptr::from_string_literal_array(b"beta");
    assert!((({ len_0(((lit).clone() as Ptr<Box<[u8]>>),) }) == 4));
    return 0;
}
