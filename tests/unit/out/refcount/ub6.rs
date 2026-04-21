extern crate libcc2rs;
use libcc2rs::{prepostfix::*, rc::*};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
#[derive(Debug, Default)]
#[repr(C)]
pub struct Pair {
    pub x1: Reference<i32>,
    pub x2: Reference<i32>,
}
impl Clone for Pair {
    fn clone(&self) -> Self {
        Pair {
            x1: self.x1.clone(),
            x2: self.x2.clone(),
        }
    }
}
pub fn mkPair(x1: Reference<i32>, x2: Reference<i32>) -> Value<Pair> {
    return Rc::new(RefCell::new(Pair {
        x1: x1.clone(),
        x2: x2.clone(),
    }));
}
pub fn fill(arr: Reference<Box<[Value<Pointer<i32>>]>>, n1: Reference<i32>) {
    let n2: Value<i32> = Rc::new(RefCell::new(
        (*n1.upgrade().expect("ub: dangling reference").borrow()),
    ));
    let pair: Value<Pair> = Rc::new(RefCell::new(
        (*mkPair(n1.clone(), Rc::downgrade(&n2)).borrow()).clone(),
    ));
    (*(*arr.upgrade().expect("ub: dangling reference").borrow())[(0_i32 as u64) as usize]
        .borrow_mut()) = (*pair.borrow()).x1.as_pointer();
    (*(*arr.upgrade().expect("ub: dangling reference").borrow())[(1_i32 as u64) as usize]
        .borrow_mut()) = (*pair.borrow()).x2.as_pointer();
}
pub fn any(arr: Reference<Box<[Value<Pointer<i32>>]>>, n1: Reference<i32>) -> Value<bool> {
    let out: Value<bool> = Rc::new(RefCell::new(false));
    let i: Value<i32> = Rc::new(RefCell::new(0_i32));
    while (*i.borrow()) < (*n1.upgrade().expect("ub: dangling reference").borrow()) {
        let rhs_0 = (*out.borrow())
            || (*(*(*arr.upgrade().expect("ub: dangling reference").borrow())
                [((*i.borrow()) as u64) as usize]
                .borrow())
            .as_reference()
            .upgrade()
            .expect("ub: dangling pointer")
            .borrow())
                == 0_i32;
        (*out.borrow_mut()) = rhs_0;
        (*i.borrow_mut()).prefix_inc();
    }
    return Rc::new(RefCell::new((*out.borrow())));
}
pub fn main() {
    std::process::exit(*main_0().borrow() as i32);
}
pub fn main_0() -> Value<i32> {
    let n: Value<i32> = Rc::new(RefCell::new(2_i32));
    let arr: Value<Box<[Value<Pointer<i32>>]>> = Rc::new(RefCell::new(
        (0..((*n.borrow()) as u64))
            .map(|_| Rc::new(RefCell::new(Pointer::<i32>::null())))
            .collect::<Box<[_]>>(),
    ));
    fill(Rc::downgrade(&arr), Rc::downgrade(&n));
    return Rc::new(RefCell::new(
        ((*any(Rc::downgrade(&arr), Rc::downgrade(&n)).borrow()) as i32),
    ));
}
