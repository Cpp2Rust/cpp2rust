extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Pair {
    pub x1: Ptr<i32>,
    pub x2: Ptr<i32>,
}
impl Clone for Pair {
    fn clone(&self) -> Self {
        let mut this = Self {
            x1: (self.x1).clone(),
            x2: (self.x2).clone(),
        };
        this
    }
}
impl ByteRepr for Pair {}
pub fn mkPair_0(x1: Ptr<i32>, x2: Ptr<i32>) -> Pair {
    return Pair {
        x1: (x1).clone(),
        x2: (x2).clone(),
    };
}
pub fn fill_1(arr: Ptr<Option<Value<Box<[Ptr<i32>]>>>>, n1: Ptr<i32>) {
    let n2: Value<i32> = Rc::new(RefCell::new((n1.read())));
    let pair: Value<Pair> = Rc::new(RefCell::new(
        ({
            let _x1: Ptr<i32> = (n1).clone();
            let _x2: Ptr<i32> = n2.as_pointer();
            mkPair_0(_x1, _x2)
        }),
    ));
    (*arr.upgrade().deref()).as_ref().unwrap().borrow_mut()[(0_u64) as usize] =
        ((*pair.borrow()).x1).clone();
    (*arr.upgrade().deref()).as_ref().unwrap().borrow_mut()[(1_u64) as usize] =
        ((*pair.borrow()).x2).clone();
}
pub fn any_2(arr: Ptr<Option<Value<Box<[Ptr<i32>]>>>>, n1: Ptr<i32>) -> bool {
    let out: Value<bool> = Rc::new(RefCell::new(false));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (n1.read())
    } {
        let __rhs = (*out.borrow())
            || (((*arr.upgrade().deref()).as_ref().unwrap().borrow()
                [((*i.borrow()) as u64) as usize]
                .read())
                == 0);
        (*out.borrow_mut()) = __rhs;
        (*i.borrow_mut()).prefix_inc();
    }
    return (*out.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(2));
    let arr: Value<Option<Value<Box<[Ptr<i32>]>>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
            (0..((*n.borrow()) as u64))
                .map(|_| <Ptr<i32>>::default())
                .collect::<Box<[_]>>(),
        )))));
    ({
        let _arr: Ptr<Option<Value<Box<[Ptr<i32>]>>>> = arr.as_pointer();
        let _n1: Ptr<i32> = n.as_pointer();
        fill_1(_arr, _n1)
    });
    return (({
        let _arr: Ptr<Option<Value<Box<[Ptr<i32>]>>>> = arr.as_pointer();
        let _n1: Ptr<i32> = n.as_pointer();
        any_2(_arr, _n1)
    }) as i32);
}
