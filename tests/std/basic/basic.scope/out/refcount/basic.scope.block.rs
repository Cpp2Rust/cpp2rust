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
                match (
                    {
                        {
                            match (
                                {
                                    {
                                        match (
                                            { 0 },
                                            &mut (*useCount_3.with(Value::clone).borrow_mut()),
                                        ) {
                                            (__tmp_4, __tmp_5) => {
                                                (*__tmp_5) = __tmp_4;
                                                (*__tmp_5).clone()
                                            }
                                        }
                                    }
                                },
                                &mut (*dtorCount_2.with(Value::clone).borrow_mut()),
                            ) {
                                (__tmp_2, __tmp_3) => {
                                    (*__tmp_3) = __tmp_2;
                                    (*__tmp_3).clone()
                                }
                            }
                        }
                    },
                    &mut (*copyCount_1.with(Value::clone).borrow_mut()),
                ) {
                    (__tmp_0, __tmp_1) => {
                        (*__tmp_1) = __tmp_0;
                        (*__tmp_1).clone()
                    }
                }
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
    pub static then_runs_4: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static else_runs_5: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static dtorCount_6: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct p1_1_condition_variable_visible_in_else_ConvertsToBool {
    #[offset(0)]
    pub value: i32,
}
pub fn test_7() {
    {
        let condition: Value<p1_1_condition_variable_visible_in_else_ConvertsToBool> = Rc::new(
            RefCell::new(p1_1_condition_variable_visible_in_else_ConvertsToBool { value: 0 }),
        );
        let _dtor_condition =
            std::mem::DropGuard::new(condition.clone(), |__p| __p.as_pointer().destructor());
        if ({
            p1_1_condition_variable_visible_in_else_ConvertsToBoolImpl::to_bool(
                &condition.as_pointer(),
            )
        }) {
            (*then_runs_4.with(Value::clone).borrow_mut()).prefix_inc();
        } else {
            (*else_runs_5.with(Value::clone).borrow_mut()).prefix_inc();
            assert!(({ ({ (*condition.borrow()).value } == 0) }));
            assert!((dtorCount_6.with(|rc| *rc.borrow()) == 0));
        }
    }
    assert!((then_runs_4.with(|rc| *rc.borrow()) == 0));
    assert!((else_runs_5.with(|rc| *rc.borrow()) == 1));
    assert!((dtorCount_6.with(|rc| *rc.borrow()) == 1));
    {
        let condition: Value<p1_1_condition_variable_visible_in_else_ConvertsToBool> = Rc::new(
            RefCell::new(p1_1_condition_variable_visible_in_else_ConvertsToBool { value: 6 }),
        );
        let _dtor_condition =
            std::mem::DropGuard::new(condition.clone(), |__p| __p.as_pointer().destructor());
        if ({
            p1_1_condition_variable_visible_in_else_ConvertsToBoolImpl::to_bool(
                &condition.as_pointer(),
            )
        }) {
            (*then_runs_4.with(Value::clone).borrow_mut()).prefix_inc();
            assert!(({ ({ (*condition.borrow()).value } == 6) }));
            assert!((dtorCount_6.with(|rc| *rc.borrow()) == 1));
        } else {
            (*else_runs_5.with(Value::clone).borrow_mut()).prefix_inc();
        }
    }
    assert!((then_runs_4.with(|rc| *rc.borrow()) == 1));
    assert!((else_runs_5.with(|rc| *rc.borrow()) == 1));
    assert!((dtorCount_6.with(|rc| *rc.borrow()) == 2));
}
pub fn test_8() {
    let mut index: i32 = 42;
    let mut iterations: i32 = 0;
    let mut sum: i32 = 0;
    {
        let mut index: i32 = 0;
        'loop_: while (index < 3) {
            iterations.prefix_inc();
            sum += index;
            index.prefix_inc();
        }
    }
    assert!((iterations == 3));
    assert!((sum == 3));
    assert!((index == 42));
}
pub fn test_9() {
    ({ LifetimeTracked::reset() });
    let scoped: Value<LifetimeTracked> = Rc::new(RefCell::new(LifetimeTracked::new()));
    let _dtor_scoped =
        std::mem::DropGuard::new(scoped.clone(), |__p| __p.as_pointer().destructor());
    assert!((ctorCount_0.with(|rc| *rc.borrow()) == 1));
    {
        let mut index: i32 = 0;
        'loop_: while (index < 3) {
            let scoped: Value<LifetimeTracked> = Rc::new(RefCell::new(LifetimeTracked::new()));
            let _dtor_scoped =
                std::mem::DropGuard::new(scoped.clone(), |__p| __p.as_pointer().destructor());
            index.prefix_inc();
        }
    }
    assert!((ctorCount_0.with(|rc| *rc.borrow()) == 4));
    assert!((dtorCount_2.with(|rc| *rc.borrow()) == 3));
    if (ctorCount_0.with(|rc| *rc.borrow()) == 4) {
        let scoped: Value<LifetimeTracked> = Rc::new(RefCell::new(LifetimeTracked::new()));
        let _dtor_scoped =
            std::mem::DropGuard::new(scoped.clone(), |__p| __p.as_pointer().destructor());
    }
    assert!((ctorCount_0.with(|rc| *rc.borrow()) == 5));
    assert!((dtorCount_2.with(|rc| *rc.borrow()) == 4));
    if (ctorCount_0.with(|rc| *rc.borrow()) == 0) {
    } else {
        let scoped: Value<LifetimeTracked> = Rc::new(RefCell::new(LifetimeTracked::new()));
        let _dtor_scoped =
            std::mem::DropGuard::new(scoped.clone(), |__p| __p.as_pointer().destructor());
    }
    assert!((ctorCount_0.with(|rc| *rc.borrow()) == 6));
    assert!((dtorCount_2.with(|rc| *rc.borrow()) == 5));
    ({ LifetimeTrackedImpl::use_(&scoped.as_pointer()) });
    assert!(({ ({ (*scoped.borrow()).uses } == 1) }));
}
pub fn test_10() {
    let depth: Value<i32> = Rc::new(RefCell::new(1));
    let mut outermost: Ptr<i32> = (depth.as_pointer());
    {
        let mut depth: i32 = 2;
        {
            outermost.with_mut(|__v| *__v = *__v + 10)
        };
        assert!((depth == 2));
        {
            let mut depth: i32 = 3;
            {
                outermost.with_mut(|__v| *__v = *__v + 100)
            };
            assert!((depth == 3));
        }
        assert!((depth == 2));
    }
    assert!(({ ((*depth.borrow()) == 111) }));
}
pub fn test_11() {
    ({ LifetimeTracked::reset() });
    let tracked: Value<LifetimeTracked> = Rc::new(RefCell::new(LifetimeTracked::new()));
    let _dtor_tracked =
        std::mem::DropGuard::new(tracked.clone(), |__p| __p.as_pointer().destructor());
    {
        let tracked: Value<LifetimeTracked> = Rc::new(RefCell::new(LifetimeTracked::new()));
        let _dtor_tracked =
            std::mem::DropGuard::new(tracked.clone(), |__p| __p.as_pointer().destructor());
        assert!((ctorCount_0.with(|rc| *rc.borrow()) == 2));
        ({ LifetimeTrackedImpl::use_(&tracked.as_pointer()) });
        assert!(({ ({ (*tracked.borrow()).uses } == 1) }));
        assert!((dtorCount_2.with(|rc| *rc.borrow()) == 0));
    }
    assert!((dtorCount_2.with(|rc| *rc.borrow()) == 1));
    assert!(({ ({ (*tracked.borrow()).uses } == 0) }));
    ({ LifetimeTrackedImpl::use_(&tracked.as_pointer()) });
    assert!(({ ({ (*tracked.borrow()).uses } == 1) }));
    assert!((useCount_3.with(|rc| *rc.borrow()) == 2));
}
pub fn test_12() {
    let mut outer_iterations: i32 = 0;
    let mut inner_iterations: i32 = 0;
    let mut sum: i32 = 0;
    {
        let mut index: i32 = 0;
        'loop_: while (index < 2) {
            {
                let mut index: i32 = 10;
                'loop_: while (index < 13) {
                    inner_iterations.prefix_inc();
                    sum += index;
                    index.prefix_inc();
                }
            }
            {
                index.prefix_inc();
                outer_iterations.prefix_inc()
            };
        }
    }
    assert!((outer_iterations == 2));
    assert!((inner_iterations == 6));
    assert!((sum == 66));
}
thread_local!(
    pub static inner_then_runs_13: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static inner_else_runs_14: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static outer_else_runs_15: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn test_16() {
    {
        let mut choice: i32 = 4;
        if (choice != 0) {
            {
                let mut choice: i32 = 0;
                if (choice != 0) {
                    (*inner_then_runs_13.with(Value::clone).borrow_mut()).prefix_inc();
                } else {
                    (*inner_else_runs_14.with(Value::clone).borrow_mut()).prefix_inc();
                    assert!((choice == 0));
                }
            }
        } else {
            (*outer_else_runs_15.with(Value::clone).borrow_mut()).prefix_inc();
            assert!((choice == 0));
        }
    }
    assert!((inner_then_runs_13.with(|rc| *rc.borrow()) == 0));
    assert!((inner_else_runs_14.with(|rc| *rc.borrow()) == 1));
    assert!((outer_else_runs_15.with(|rc| *rc.borrow()) == 0));
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_7() });
    ({ test_8() });
    ({ test_9() });
    ({ test_10() });
    ({ test_11() });
    ({ test_12() });
    ({ test_16() });
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
pub trait p1_1_condition_variable_visible_in_else_ConvertsToBoolImpl {
    fn to_bool(&self) -> bool;
    fn destructor(&self);
}
impl p1_1_condition_variable_visible_in_else_ConvertsToBoolImpl
    for Ptr<p1_1_condition_variable_visible_in_else_ConvertsToBool>
{
    fn to_bool(&self) -> bool {
        return ((*self).with(|__s| __s.value) != 0);
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
    let _ = then_runs_4.with(|_| ());
    let _ = else_runs_5.with(|_| ());
    let _ = dtorCount_6.with(|_| ());
    let _ = inner_then_runs_13.with(|_| ());
    let _ = inner_else_runs_14.with(|_| ());
    let _ = outer_else_runs_15.with(|_| ());
}
