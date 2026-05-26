extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static s_p_mut: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
);
thread_local!(
    pub static s_p_const: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
);
thread_local!(
    pub static s_cp: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::<u8>::null()));
);
thread_local!(
    pub static s_arr_of_ptr: Value<Box<[Ptr<i32>]>> = Rc::new(RefCell::new(
        (0..4)
            .map(|_| Ptr::<i32>::null())
            .collect::<Box<[Ptr<i32>]>>(),
    ));
);
thread_local!(
    pub static s_pp: Value<Ptr<Ptr<i32>>> = Rc::new(RefCell::new(Ptr::<Ptr<i32>>::null()));
);
thread_local!(
    pub static s_const_arr_of_ptr: Value<Box<[Ptr<i32>]>> = Rc::new(RefCell::new(
        (0..3)
            .map(|_| Ptr::<i32>::null())
            .collect::<Box<[Ptr<i32>]>>(),
    ));
);
thread_local!(
    pub static s_cp_explicit_null: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::<u8>::null()));
);
thread_local!(
    pub static s_p_zero: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
);
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((*s_p_mut.with(Value::clone).borrow()).is_null());
    assert!((*s_p_const.with(Value::clone).borrow()).is_null());
    assert!((*s_cp.with(Value::clone).borrow()).is_null());
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 4) {
        assert!(((*s_arr_of_ptr.with(Value::clone).borrow())[(*i.borrow()) as usize]).is_null());
        (*i.borrow_mut()).prefix_inc();
    }
    assert!((*s_pp.with(Value::clone).borrow()).is_null());
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 3) {
        assert!(
            ((*s_const_arr_of_ptr.with(Value::clone).borrow())[(*i.borrow()) as usize]).is_null()
        );
        (*i.borrow_mut()).prefix_inc();
    }
    assert!((*s_cp_explicit_null.with(Value::clone).borrow()).is_null());
    assert!((*s_p_zero.with(Value::clone).borrow()).is_null());
    return 0;
}
