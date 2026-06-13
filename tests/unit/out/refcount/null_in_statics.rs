extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static p_mut_0: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
);
thread_local!(
    pub static p_const_1: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
);
thread_local!(
    pub static cp_2: Value<Ptr<core::ffi::c_char>> =
        Rc::new(RefCell::new(Ptr::<core::ffi::c_char>::null()));
);
thread_local!(
    pub static arr_of_ptr_3: Value<Box<[Ptr<i32>]>> = Rc::new(RefCell::new(
        (0..4)
            .map(|_| Ptr::<i32>::null())
            .collect::<Box<[Ptr<i32>]>>(),
    ));
);
thread_local!(
    pub static pp_4: Value<Ptr<Ptr<i32>>> = Rc::new(RefCell::new(Ptr::<Ptr<i32>>::null()));
);
thread_local!(
    pub static const_arr_of_ptr_5: Value<Box<[Ptr<i32>]>> = Rc::new(RefCell::new(
        (0..3)
            .map(|_| Ptr::<i32>::null())
            .collect::<Box<[Ptr<i32>]>>(),
    ));
);
thread_local!(
    pub static cp_explicit_null_6: Value<Ptr<core::ffi::c_char>> =
        Rc::new(RefCell::new(Ptr::<core::ffi::c_char>::null()));
);
thread_local!(
    pub static p_zero_7: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
);
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((*p_mut_0.with(Value::clone).borrow()).is_null());
    assert!((*p_const_1.with(Value::clone).borrow()).is_null());
    assert!((*cp_2.with(Value::clone).borrow()).is_null());
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 4) {
        assert!(((*arr_of_ptr_3.with(Value::clone).borrow())[(*i.borrow()) as usize]).is_null());
        (*i.borrow_mut()).prefix_inc();
    }
    assert!((*pp_4.with(Value::clone).borrow()).is_null());
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 3) {
        assert!(
            ((*const_arr_of_ptr_5.with(Value::clone).borrow())[(*i.borrow()) as usize]).is_null()
        );
        (*i.borrow_mut()).prefix_inc();
    }
    assert!((*cp_explicit_null_6.with(Value::clone).borrow()).is_null());
    assert!((*p_zero_7.with(Value::clone).borrow()).is_null());
    return 0;
}
