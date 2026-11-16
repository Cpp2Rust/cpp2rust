extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static ctorCount_0: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static dtorCount_1: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Destructor, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct p2_condition_variable_created_each_iteration_ConvertsToBool {
    #[offset(0)]
    pub value: i32,
}
impl p2_condition_variable_created_each_iteration_ConvertsToBool {
    pub fn new(mut value: i32) -> Self {
        let __this: p2_condition_variable_created_each_iteration_ConvertsToBool =
            Self { value: value };
        (*ctorCount_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
pub fn test_2() {
    let mut remaining: i32 = 2;
    let mut iterations: i32 = 0;
    'loop_: loop {
        let condition: Value<p2_condition_variable_created_each_iteration_ConvertsToBool> =
            Rc::new(RefCell::new(
                p2_condition_variable_created_each_iteration_ConvertsToBool::new({ remaining }),
            ));
        let _dtor_condition =
            std::mem::DropGuard::new(condition.clone(), |__p| __p.as_pointer().destructor());
        if !({
            p2_condition_variable_created_each_iteration_ConvertsToBoolImpl::to_bool(
                &condition.as_pointer(),
            )
        }) {
            break;
        }
        iterations.prefix_inc();
        assert!(({ ({ (*condition.borrow()).value } == remaining) }));
        assert!((ctorCount_0.with(|rc| *rc.borrow()) == iterations));
        assert!((dtorCount_1.with(|rc| *rc.borrow()) == (iterations - 1)));
        remaining.prefix_dec();
    }
    assert!((iterations == 2));
    assert!((ctorCount_0.with(|rc| *rc.borrow()) == 3));
    assert!((dtorCount_1.with(|rc| *rc.borrow()) == 3));
}
thread_local!(
    pub static ctorCount_3: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static dtorCount_4: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Destructor, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct p2_continue_destroys_condition_variable_ConvertsToBool {
    #[offset(0)]
    pub value: i32,
}
impl p2_continue_destroys_condition_variable_ConvertsToBool {
    pub fn new(mut value: i32) -> Self {
        let __this: p2_continue_destroys_condition_variable_ConvertsToBool = Self { value: value };
        (*ctorCount_3.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
pub fn test_5() {
    let mut remaining: i32 = 3;
    let mut continued: i32 = 0;
    let mut completed: i32 = 0;
    'loop_: loop {
        let condition: Value<p2_continue_destroys_condition_variable_ConvertsToBool> =
            Rc::new(RefCell::new(
                p2_continue_destroys_condition_variable_ConvertsToBool::new({ remaining }),
            ));
        let _dtor_condition =
            std::mem::DropGuard::new(condition.clone(), |__p| __p.as_pointer().destructor());
        if !({
            p2_continue_destroys_condition_variable_ConvertsToBoolImpl::to_bool(
                &condition.as_pointer(),
            )
        }) {
            break;
        }
        remaining.prefix_dec();
        if ({ (*condition.borrow()).value } == 3) {
            continued.prefix_inc();
            continue 'loop_;
        }
        assert!((ctorCount_3.with(|rc| *rc.borrow()) == 2));
        assert!((dtorCount_4.with(|rc| *rc.borrow()) == 1));
        completed.prefix_inc();
        break;
    }
    assert!((continued == 1));
    assert!((completed == 1));
    assert!((remaining == 1));
    assert!((ctorCount_3.with(|rc| *rc.borrow()) == 2));
    assert!((dtorCount_4.with(|rc| *rc.borrow()) == 2));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_2() });
    ({ test_5() });
    return 0;
}
pub trait p2_condition_variable_created_each_iteration_ConvertsToBoolImpl {
    fn destructor(&self);
    fn to_bool(&self) -> bool;
}
impl p2_condition_variable_created_each_iteration_ConvertsToBoolImpl
    for Ptr<p2_condition_variable_created_each_iteration_ConvertsToBool>
{
    fn destructor(&self) {
        (*dtorCount_1.with(Value::clone).borrow_mut()).prefix_inc();
    }
    fn to_bool(&self) -> bool {
        return ((*self).with(|__s| __s.value) != 0);
    }
}
pub trait p2_continue_destroys_condition_variable_ConvertsToBoolImpl {
    fn destructor(&self);
    fn to_bool(&self) -> bool;
}
impl p2_continue_destroys_condition_variable_ConvertsToBoolImpl
    for Ptr<p2_continue_destroys_condition_variable_ConvertsToBool>
{
    fn destructor(&self) {
        (*dtorCount_4.with(Value::clone).borrow_mut()).prefix_inc();
    }
    fn to_bool(&self) -> bool {
        return ((*self).with(|__s| __s.value) != 0);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = ctorCount_0.with(|_| ());
    let _ = dtorCount_1.with(|_| ());
    let _ = ctorCount_3.with(|_| ());
    let _ = dtorCount_4.with(|_| ());
}
