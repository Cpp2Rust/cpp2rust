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
    pub static copyCount_1: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static dtorCount_2: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static useCount_3: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(4)]
pub struct LifetimeTracked {
    #[offset(0)]
    pub uses: i32,
}
impl LifetimeTracked {
    pub fn reset() {
        ctorCount_0.with(|rc| {
            *rc.borrow_mut() = {
                copyCount_1.with(|rc| {
                    *rc.borrow_mut() = {
                        dtorCount_2.with(|rc| {
                            *rc.borrow_mut() = {
                                useCount_3.with(|rc| *rc.borrow_mut() = 0);
                                useCount_3.with(|rc| *rc.borrow())
                            }
                        });
                        dtorCount_2.with(|rc| *rc.borrow())
                    }
                });
                copyCount_1.with(|rc| *rc.borrow())
            }
        });
    }
    pub fn new() -> Self {
        let __this: LifetimeTracked = Self { uses: 0 };
        (*ctorCount_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(_a0: Ptr<LifetimeTracked>) -> Self {
        let __this: LifetimeTracked = Self { uses: 0 };
        (*copyCount_1.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for LifetimeTracked {
    fn clone(&self) -> Self {
        let __src: Value<LifetimeTracked> = Rc::new(RefCell::new(LifetimeTracked {
            uses: self.uses.clone(),
        }));
        LifetimeTracked::copy_from(__src.as_pointer())
    }
}
impl Default for LifetimeTracked {
    fn default() -> Self {
        { LifetimeTracked::new() }
    }
}
thread_local!(
    pub static matching_case_runs_4: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static other_case_runs_5: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static dtorCount_6: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Destructor, Default)]
#[cpp_into(i32, to_i32)]
#[byte_size(4)]
pub struct p2_condition_variable_lives_for_whole_statement_ConvertsToInt {
    #[offset(0)]
    pub value: i32,
}
pub fn test_7() {
    {
        let selector: Value<p2_condition_variable_lives_for_whole_statement_ConvertsToInt> =
            Rc::new(RefCell::new(
                p2_condition_variable_lives_for_whole_statement_ConvertsToInt { value: 2 },
            ));
        let _dtor_selector =
            std::mem::DropGuard::new(selector.clone(), |__p| __p.as_pointer().destructor());
        'switch: {
            match {
                ({
                    p2_condition_variable_lives_for_whole_statement_ConvertsToIntImpl::to_i32(
                        &selector.as_pointer(),
                    )
                })
            } {
                __v if __v == 2 => {
                    (*matching_case_runs_4.with(Value::clone).borrow_mut()).prefix_inc();
                    assert!(({ ({ (*selector.borrow()).value } == 2) }));
                    assert!((dtorCount_6.with(|rc| *rc.borrow()) == 0));
                    break 'switch;
                }
                _ => {
                    (*other_case_runs_5.with(Value::clone).borrow_mut()).prefix_inc();
                    break 'switch;
                }
            }
        }
    };
    assert!((matching_case_runs_4.with(|rc| *rc.borrow()) == 1));
    assert!((other_case_runs_5.with(|rc| *rc.borrow()) == 0));
    assert!((dtorCount_6.with(|rc| *rc.borrow()) == 1));
}
thread_local!(
    pub static matching_case_runs_8: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static other_case_runs_9: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn test_10() {
    ({ LifetimeTracked::reset() });
    let mut selector: i32 = 9;
    {
        let guard: Value<LifetimeTracked> = Rc::new(RefCell::new(LifetimeTracked::new()));
        let _dtor_guard =
            std::mem::DropGuard::new(guard.clone(), |__p| __p.as_pointer().destructor());
        let mut selector: i32 = 2;
        'switch: {
            match { selector } {
                __v if __v == 2 => {
                    (*matching_case_runs_8.with(Value::clone).borrow_mut()).prefix_inc();
                    ({ LifetimeTrackedImpl::use_(&guard.as_pointer()) });
                    assert!(({ ({ (*guard.borrow()).uses } == 1) }));
                    assert!((selector == 2));
                    assert!((dtorCount_2.with(|rc| *rc.borrow()) == 0));
                    break 'switch;
                }
                _ => {
                    (*other_case_runs_9.with(Value::clone).borrow_mut()).prefix_inc();
                    break 'switch;
                }
            }
        }
    };
    assert!((matching_case_runs_8.with(|rc| *rc.borrow()) == 1));
    assert!((other_case_runs_9.with(|rc| *rc.borrow()) == 0));
    assert!((ctorCount_0.with(|rc| *rc.borrow()) == 1));
    assert!((dtorCount_2.with(|rc| *rc.borrow()) == 1));
    assert!((selector == 9));
}
pub fn classify_11(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        match { x } {
            __v if __v == 1 => {
                std::hint::cold_path();
                return -1_i32;
            }
            __v if __v == 2 => {
                r = 2;
                break 'switch;
            }
            __v if __v == 3 => {
                std::hint::cold_path();
                r = 3;
                break 'switch;
            }
            _ => {
                break 'switch;
            }
        }
    };
    'loop_: while (x.postfix_dec() > 0) {
        std::hint::cold_path();
        {
            r += 10;
        };
    }
    return r;
}
pub fn test_12() {
    assert!((({ classify_11(1,) }) == -1_i32));
    assert!((({ classify_11(2,) }) == 22));
    assert!((({ classify_11(3,) }) == 33));
    assert!((({ classify_11(0,) }) == 0));
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_12() });
    ({ test_7() });
    ({ test_10() });
    return 0;
}
pub trait LifetimeTrackedImpl {
    fn destructor(&self);
    fn use_(&self);
    fn copy_assign(&self, _a0: Ptr<LifetimeTracked>) -> Ptr<LifetimeTracked>;
}
impl LifetimeTrackedImpl for Ptr<LifetimeTracked> {
    fn destructor(&self) {
        (*dtorCount_2.with(Value::clone).borrow_mut()).prefix_inc();
    }
    fn use_(&self) {
        field!((*self), uses).with_mut(|__v| {
            __v.prefix_inc();
        });
        (*useCount_3.with(Value::clone).borrow_mut()).prefix_inc();
    }
    fn copy_assign(&self, _a0: Ptr<LifetimeTracked>) -> Ptr<LifetimeTracked> {
        field!((*self), uses).write({ _a0.with(|__s| __s.uses) });
        return (*self).clone();
    }
}
pub trait p2_condition_variable_lives_for_whole_statement_ConvertsToIntImpl {
    fn to_i32(&self) -> i32;
    fn destructor(&self);
}
impl p2_condition_variable_lives_for_whole_statement_ConvertsToIntImpl
    for Ptr<p2_condition_variable_lives_for_whole_statement_ConvertsToInt>
{
    fn to_i32(&self) -> i32 {
        return (*self).with(|__s| __s.value);
    }
    fn destructor(&self) {
        (*dtorCount_6.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = ctorCount_0.with(|_| ());
    let _ = copyCount_1.with(|_| ());
    let _ = dtorCount_2.with(|_| ());
    let _ = useCount_3.with(|_| ());
    let _ = matching_case_runs_4.with(|_| ());
    let _ = other_case_runs_5.with(|_| ());
    let _ = dtorCount_6.with(|_| ());
    let _ = matching_case_runs_8.with(|_| ());
    let _ = other_case_runs_9.with(|_| ());
}
