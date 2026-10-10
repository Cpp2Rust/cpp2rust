extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = 1;
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 2;
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 3;
        (*v.borrow_mut()).push(__a1)
    };
    let mut square: i32 = 0;
    'loop_: for mut e1 in v.as_pointer() as Ptr<i32> {
        let mut e1: i32 = e1.read();
        'loop_: for mut e2 in v.as_pointer() as Ptr<i32> {
            let mut e2: i32 = e2.read();
            square += (e1 * e2);
        }
    }
    'loop_: for mut e1 in v.as_pointer() as Ptr<i32> {
        'loop_: for mut e2 in v.as_pointer() as Ptr<i32> {
            square += { ({ (e1.read()) } * { (e2.read()) }) };
        }
    }
    'loop_: for mut e1 in v.as_pointer() as Ptr<i32> {
        'loop_: for mut e2 in v.as_pointer() as Ptr<i32> {
            square += { ({ (e1.read()) } * { (e2.read()) }) };
        }
    }
    'loop_: for mut e1 in v.as_pointer() as Ptr<i32> {
        'loop_: for mut e2 in v.as_pointer() as Ptr<i32> {
            square += { ({ (e1.read()) } * { (e2.read()) }) };
        }
    }
    let m: Value<Vec<Value<Vec<i32>>>> = Rc::new(RefCell::new(Vec::new()));
    let v1: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    (m.as_pointer() as Ptr<Vec<Value<Vec<i32>>>>).with_mut(|__v: &mut Vec<Value<Vec<i32>>>| {
        __v.push(Rc::new(RefCell::new(std::mem::take(
            &mut (*v1.borrow_mut()),
        ))))
    });
    let v2: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    (m.as_pointer() as Ptr<Vec<Value<Vec<i32>>>>).with_mut(|__v: &mut Vec<Value<Vec<i32>>>| {
        __v.push(Rc::new(RefCell::new(std::mem::take(
            &mut (*v2.borrow_mut()),
        ))))
    });
    let v3: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    (m.as_pointer() as Ptr<Vec<Value<Vec<i32>>>>).with_mut(|__v: &mut Vec<Value<Vec<i32>>>| {
        __v.push(Rc::new(RefCell::new(std::mem::take(
            &mut (*v3.borrow_mut()),
        ))))
    });
    'loop_: for mut row in m.as_pointer() as Ptr<Value<Vec<i32>>> {
        let row: Ptr<Vec<i32>> = row.upgrade().deref().as_pointer();
        'loop_: for mut col in Ptr::<Vec<i32>>::decay(&(row)) as Ptr<i32> {
            square += { (col.read()) };
        }
    }
    assert!((square == 144));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
