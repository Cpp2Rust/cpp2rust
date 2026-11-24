extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn read_0(value: Ptr<i32>) -> i32 {
    return (value.read());
}
thread_local!(
    pub static selected_1: Value<i32> = Rc::new(RefCell::new({
        let __tmp_0: Value<i32>;
        let __tmp_1: Value<i32>;
        ({
            read_0(
                ({
                    __tmp_0 = Rc::new(RefCell::new(0));
                    __tmp_1 = Rc::new(RefCell::new(0));
                    if CppLess::less(&__tmp_0.as_pointer(), &__tmp_1.as_pointer()) {
                        __tmp_0.as_pointer()
                    } else {
                        __tmp_1.as_pointer()
                    }
                }),
            )
        })
    }));
);
thread_local!(
    pub static x_2: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static y_3: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static assigned_4: Value<i32> = Rc::new(RefCell::new({
        let __tmp_4: Value<i32>;
        let __tmp_5: Value<i32>;
        ({
            match (
                {
                    {
                        __tmp_4 = Rc::new(RefCell::new(6));
                        __tmp_5 = Rc::new(RefCell::new(5));
                        (if CppLess::less(&__tmp_4.as_pointer(), &__tmp_5.as_pointer()) {
                            __tmp_4.as_pointer()
                        } else {
                            __tmp_5.as_pointer()
                        }
                        .read())
                    }
                },
                &mut (*x_2.with(Value::clone).borrow_mut()),
            ) {
                (__tmp_2, __tmp_3) => {
                    (*__tmp_3) = __tmp_2;
                    (*__tmp_3).clone()
                }
            }
        })
    }));
);
thread_local!(
    pub static accumulated_5: Value<i32> = Rc::new(RefCell::new({
        let __tmp_8: Value<i32>;
        let __tmp_9: Value<i32>;
        ({
            match (
                {
                    {
                        __tmp_8 = Rc::new(RefCell::new(1));
                        __tmp_9 = Rc::new(RefCell::new(2));
                        (if CppLess::less(&__tmp_8.as_pointer(), &__tmp_9.as_pointer()) {
                            __tmp_9.as_pointer()
                        } else {
                            __tmp_8.as_pointer()
                        }
                        .read())
                    }
                },
                &mut (*y_3.with(Value::clone).borrow_mut()),
            ) {
                (__tmp_6, __tmp_7) => {
                    (*__tmp_7) = (((*__tmp_7) as i32) + (__tmp_6 as i32)) as i32;
                    (*__tmp_7).clone()
                }
            }
        })
    }));
);
thread_local!(
    pub static call_assigned_6: Value<i32> = Rc::new(RefCell::new({
        let __tmp_12: Value<i32>;
        ({
            match (
                {
                    ({
                        __tmp_12 = Rc::new(RefCell::new(5));
                        read_0(__tmp_12.as_pointer())
                    })
                },
                &mut (*x_2.with(Value::clone).borrow_mut()),
            ) {
                (__tmp_10, __tmp_11) => {
                    (*__tmp_11) = __tmp_10;
                    (*__tmp_11).clone()
                }
            }
        })
    }));
);
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Item {
    #[offset(0)]
    pub value: i32,
}
pub fn read_array_7(value: Ptr<i32>) -> i32 {
    return (elem!(value, 0).read());
}
pub fn read_record_8(value: Ptr<Item>) -> i32 {
    return value.with(|__s| __s.value);
}
thread_local!(
    pub static aggregate_selected_9: Value<i32> = Rc::new(RefCell::new({
        let __tmp_14: Value<i32>;
        let __tmp_15: Value<i32>;
        let __tmp_13: Value<Box<[i32]>>;
        let __tmp_17: Value<i32>;
        let __tmp_18: Value<i32>;
        let __tmp_16: Value<Item>;
        (({
            __tmp_13 = Rc::new(RefCell::new(Box::new([{
                __tmp_14 = Rc::new(RefCell::new(4));
                __tmp_15 = Rc::new(RefCell::new(3));
                (if CppLess::less(&__tmp_14.as_pointer(), &__tmp_15.as_pointer()) {
                    __tmp_14.as_pointer()
                } else {
                    __tmp_15.as_pointer()
                }
                .read())
            }])));
            read_array_7(__tmp_13.decay())
        }) + ({
            __tmp_16 = Rc::new(RefCell::new(Item {
                value: {
                    __tmp_17 = Rc::new(RefCell::new(5));
                    __tmp_18 = Rc::new(RefCell::new(6));
                    (if CppLess::less(&__tmp_17.as_pointer(), &__tmp_18.as_pointer()) {
                        __tmp_18.as_pointer()
                    } else {
                        __tmp_17.as_pointer()
                    }
                    .read())
                },
            }));
            read_record_8(__tmp_16.as_pointer())
        }))
    }));
);
thread_local!(
    pub static left_10: Value<u64> = Rc::new(RefCell::new(3_u64));
);
thread_local!(
    pub static right_11: Value<usize> = Rc::new(RefCell::new(7_usize));
);
thread_local!(
    pub static mapped_selected_12: Value<usize> = {
        let __tmp_19: Value<u64>;
        Rc::new(RefCell::new(
            (({
                __tmp_19 = Rc::new(RefCell::new((right_11.with(|rc| *rc.borrow()) as u64)));
                (if CppLess::less(&__tmp_19.as_pointer(), &left_10.with(|v| v.as_pointer())) {
                    __tmp_19.as_pointer()
                } else {
                    left_10.with(|v| v.as_pointer())
                }
                .read())
            }) as usize),
        ))
    };
);
pub fn get_13() -> usize {
    return value_14.with(|rc| *rc.borrow());
}
thread_local!(
    static value_14: Value<usize> = {
        let __tmp_20: Value<u64>;
        Rc::new(RefCell::new(
            (({
                __tmp_20 = Rc::new(RefCell::new((right_11.with(|rc| *rc.borrow()) as u64)));
                (if CppLess::less(&left_10.with(|v| v.as_pointer()), &__tmp_20.as_pointer()) {
                    __tmp_20.as_pointer()
                } else {
                    left_10.with(|v| v.as_pointer())
                }
                .read())
            }) as usize),
        ))
    };
);
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    assert!((selected_1.with(|rc| *rc.borrow()) == 0));
    assert!((assigned_4.with(|rc| *rc.borrow()) == 5));
    assert!((accumulated_5.with(|rc| *rc.borrow()) == 2));
    assert!((call_assigned_6.with(|rc| *rc.borrow()) == 5));
    assert!((aggregate_selected_9.with(|rc| *rc.borrow()) == 9));
    assert!((aggregate_static_15.with(|rc| *rc.borrow()) == 17));
    assert!((mapped_selected_12.with(|rc| *rc.borrow()) == 3_usize));
    assert!((({ get_13() }) == 7_usize));
    return 0;
}
thread_local!(
    static aggregate_static_15: Value<i32> = Rc::new(RefCell::new({
        let __tmp_22: Value<i32>;
        let __tmp_23: Value<i32>;
        let __tmp_21: Value<Box<[i32]>>;
        let __tmp_25: Value<i32>;
        let __tmp_26: Value<i32>;
        let __tmp_24: Value<Item>;
        (({
            __tmp_21 = Rc::new(RefCell::new(Box::new([{
                __tmp_22 = Rc::new(RefCell::new(7));
                __tmp_23 = Rc::new(RefCell::new(8));
                (if CppLess::less(&__tmp_22.as_pointer(), &__tmp_23.as_pointer()) {
                    __tmp_23.as_pointer()
                } else {
                    __tmp_22.as_pointer()
                }
                .read())
            }])));
            read_array_7(__tmp_21.decay())
        }) + ({
            __tmp_24 = Rc::new(RefCell::new(Item {
                value: {
                    __tmp_25 = Rc::new(RefCell::new(10));
                    __tmp_26 = Rc::new(RefCell::new(9));
                    (if CppLess::less(&__tmp_25.as_pointer(), &__tmp_26.as_pointer()) {
                        __tmp_25.as_pointer()
                    } else {
                        __tmp_26.as_pointer()
                    }
                    .read())
                },
            }));
            read_record_8(__tmp_24.as_pointer())
        }))
    }));
);
pub fn __cpp2rust_init_globals() {
    let _ = selected_1.with(|_| ());
    let _ = x_2.with(|_| ());
    let _ = y_3.with(|_| ());
    let _ = assigned_4.with(|_| ());
    let _ = accumulated_5.with(|_| ());
    let _ = call_assigned_6.with(|_| ());
    let _ = aggregate_selected_9.with(|_| ());
    let _ = left_10.with(|_| ());
    let _ = right_11.with(|_| ());
    let _ = mapped_selected_12.with(|_| ());
}
