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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct AssertsDtorCountAtExit {
    #[offset(0)]
    pub expected: i32,
}
pub fn count_4() -> i32 {
    return (*(*n_6.with(Value::clone).borrow_mut()).prefix_inc());
}
pub fn count_7() -> i32 {
    return (*(*n_9.with(Value::clone).borrow_mut()).prefix_inc());
}
pub fn count_10() -> i32 {
    return (*(*n_12.with(Value::clone).borrow_mut()).prefix_inc());
}
thread_local!(
    static t_5: Value<LifetimeTracked> = {
        let __static = Rc::new(RefCell::new(LifetimeTracked::new()));
        libcc2rs::register_static_destructor(|| t_5.with(Value::clone).as_pointer().destructor());
        __static
    };
);
thread_local!(
    static n_6: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    static t_8: Value<LifetimeTracked> = {
        let __static = Rc::new(RefCell::new(LifetimeTracked::new()));
        libcc2rs::register_static_destructor(|| t_8.with(Value::clone).as_pointer().destructor());
        __static
    };
);
thread_local!(
    static n_9: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    static t_11: Value<LifetimeTracked> = {
        let __static = Rc::new(RefCell::new(LifetimeTracked::new()));
        libcc2rs::register_static_destructor(|| t_11.with(Value::clone).as_pointer().destructor());
        __static
    };
);
thread_local!(
    static n_12: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static check_13: Value<AssertsDtorCountAtExit> = {
        let __static = Rc::new(RefCell::new(AssertsDtorCountAtExit { expected: 4 }));
        libcc2rs::register_static_destructor(|| {
            check_13.with(Value::clone).as_pointer().destructor()
        });
        __static
    };
);
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    assert!((ctorCount_0.with(|rc| *rc.borrow()) == 0));
    assert!((({ count_4() }) == 1));
    assert!((({ b_count_local_15() }) == 1));
    assert!((({ count_4() }) == 2));
    assert!((({ count_7() }) == 1));
    assert!((({ b_count_int_16() }) == 2));
    assert!((({ count_7() }) == 3));
    assert!((ctorCount_0.with(|rc| *rc.borrow()) == 3));
    assert!((({ count_10() }) == 1));
    assert!((({ b_count_long_17() }) == 2));
    assert!((ctorCount_0.with(|rc| *rc.borrow()) == 4));
    return 0;
}
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct S_14 {}
pub fn count_18() -> i32 {
    return (*(*n_20.with(Value::clone).borrow_mut()).prefix_inc());
}
thread_local!(
    static t_19: Value<LifetimeTracked> = {
        let __static = Rc::new(RefCell::new(LifetimeTracked::new()));
        libcc2rs::register_static_destructor(|| t_19.with(Value::clone).as_pointer().destructor());
        __static
    };
);
thread_local!(
    static n_20: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn b_count_int_16() -> i32 {
    return ({ count_7() });
}
pub fn b_count_long_17() -> i32 {
    return ({ count_10() });
}
pub fn b_count_local_15() -> i32 {
    return ({ count_18() });
}
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct S_21 {}
pub trait AssertsDtorCountAtExitImpl {
    fn destructor(&self);
}
impl AssertsDtorCountAtExitImpl for Ptr<AssertsDtorCountAtExit> {
    fn destructor(&self) {
        assert!((dtorCount_2.with(|rc| *rc.borrow()) == (*self).with(|__s| __s.expected)));
    }
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
pub fn __cpp2rust_init_globals() {
    let _ = ctorCount_0.with(|_| ());
    let _ = copyCount_1.with(|_| ());
    let _ = dtorCount_2.with(|_| ());
    let _ = useCount_3.with(|_| ());
    let _ = check_13.with(|_| ());
}
