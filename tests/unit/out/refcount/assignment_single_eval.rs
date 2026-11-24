extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() {
    let mut a: [u32; 3] = [10_u32, 20_u32, 30_u32];
    let mut i: i32 = 0;
    {
        match ({ 5_u32 }, &mut (a[(i.postfix_inc()) as usize])) {
            (__tmp_0, __tmp_1) => {
                (*__tmp_1) = (((*__tmp_1) as u32).wrapping_add((__tmp_0 as u32))) as u32;
            }
        }
    };
    printf!(
        b"void %d %d %d\n",
        i,
        (a[(0) as usize] as i32),
        (a[(1) as usize] as i32)
    );
    i = 0;
    let mut sum: u32 = ({
        match ({ 2_u32 }, &mut (a[(i.postfix_inc()) as usize])) {
            (__tmp_2, __tmp_3) => {
                (*__tmp_3) = (((*__tmp_3) as u32).wrapping_add((__tmp_2 as u32))) as u32;
                (*__tmp_3).clone()
            }
        }
    });
    printf!(
        b"compound %d %d %d %d\n",
        i,
        (sum as i32),
        (a[(0) as usize] as i32),
        (a[(1) as usize] as i32)
    );
    i = 0;
    let mut assigned: u32 = ({
        match ({ 99_u32 }, &mut (a[(i.postfix_inc()) as usize])) {
            (__tmp_4, __tmp_5) => {
                (*__tmp_5) = __tmp_4;
                (*__tmp_5).clone()
            }
        }
    });
    printf!(
        b"simple %d %d %d %d\n",
        i,
        (assigned as i32),
        (a[(0) as usize] as i32),
        (a[(1) as usize] as i32)
    );
}
pub fn test_1() {
    let mut a: [i16; 3] = [10_i16, 20_i16, 30_i16];
    let mut i: i32 = 0;
    {
        match ({ 5 }, &mut (a[(i.postfix_inc()) as usize])) {
            (__tmp_6, __tmp_7) => {
                (*__tmp_7) = (((*__tmp_7) as i32) + (__tmp_6 as i32)) as i16;
            }
        }
    };
    printf!(
        b"void %d %d %d\n",
        i,
        (a[(0) as usize] as i32),
        (a[(1) as usize] as i32)
    );
    i = 0;
    let mut sum: i16 = ({
        match ({ 2 }, &mut (a[(i.postfix_inc()) as usize])) {
            (__tmp_8, __tmp_9) => {
                (*__tmp_9) = (((*__tmp_9) as i32) + (__tmp_8 as i32)) as i16;
                (*__tmp_9).clone()
            }
        }
    });
    printf!(
        b"compound %d %d %d %d\n",
        i,
        (sum as i32),
        (a[(0) as usize] as i32),
        (a[(1) as usize] as i32)
    );
    i = 0;
    let mut assigned: i16 = ({
        match ({ 99_i16 }, &mut (a[(i.postfix_inc()) as usize])) {
            (__tmp_10, __tmp_11) => {
                (*__tmp_11) = __tmp_10;
                (*__tmp_11).clone()
            }
        }
    });
    printf!(
        b"simple %d %d %d %d\n",
        i,
        (assigned as i32),
        (a[(0) as usize] as i32),
        (a[(1) as usize] as i32)
    );
}
pub fn test_2() {
    let mut a: [i32; 3] = [10, 20, 30];
    let mut i: i32 = 0;
    a[(i.postfix_inc()) as usize] += 5;
    printf!(b"void %d %d %d\n", i, a[(0) as usize], a[(1) as usize]);
    i = 0;
    let mut sum: i32 = ({
        match ({ 2 }, &mut (a[(i.postfix_inc()) as usize])) {
            (__tmp_12, __tmp_13) => {
                (*__tmp_13) = (((*__tmp_13) as i32) + (__tmp_12 as i32)) as i32;
                (*__tmp_13).clone()
            }
        }
    });
    printf!(
        b"compound %d %d %d %d\n",
        i,
        sum,
        a[(0) as usize],
        a[(1) as usize]
    );
    i = 0;
    let mut assigned: i32 = ({
        match ({ 99 }, &mut (a[(i.postfix_inc()) as usize])) {
            (__tmp_14, __tmp_15) => {
                (*__tmp_15) = __tmp_14;
                (*__tmp_15).clone()
            }
        }
    });
    printf!(
        b"simple %d %d %d %d\n",
        i,
        assigned,
        a[(0) as usize],
        a[(1) as usize]
    );
}
thread_local!(
    pub static global_3: Value<i32> = Rc::new(RefCell::new(10));
);
pub fn observe_global_4(mut assigned: i32) -> i32 {
    return (assigned + global_3.with(|rc| *rc.borrow()));
}
pub fn observe_5(mut assigned: i32, mut stored: Ptr<i32>) -> i32 {
    return ({ assigned } + { (stored.read()) });
}
pub fn test_borrows_6() {
    printf!(
        b"%d\n",
        ({
            observe_global_4({
                match ({ 99 }, &mut (*global_3.with(Value::clone).borrow_mut())) {
                    (__tmp_16, __tmp_17) => {
                        (*__tmp_17) = __tmp_16;
                        (*__tmp_17).clone()
                    }
                }
            })
        })
    );
    printf!(
        b"%d\n",
        ({
            observe_global_4({
                match ({ 5 }, &mut (*global_3.with(Value::clone).borrow_mut())) {
                    (__tmp_18, __tmp_19) => {
                        (*__tmp_19) = (((*__tmp_19) as i32) + (__tmp_18 as i32)) as i32;
                        (*__tmp_19).clone()
                    }
                }
            })
        })
    );
    let local: Value<i32> = Rc::new(RefCell::new(10));
    printf!(
        b"%d\n",
        ({
            let _assigned: i32 = {
                match ({ 99 }, &mut (*local.borrow_mut())) {
                    (__tmp_20, __tmp_21) => {
                        (*__tmp_21) = __tmp_20;
                        (*__tmp_21).clone()
                    }
                }
            };
            let _stored: Ptr<i32> = (local.as_pointer());
            observe_5(_assigned, _stored)
        })
    );
    printf!(
        b"%d\n",
        ({
            let _assigned: i32 = {
                match ({ 5 }, &mut (*local.borrow_mut())) {
                    (__tmp_22, __tmp_23) => {
                        (*__tmp_23) = (((*__tmp_23) as i32) + (__tmp_22 as i32)) as i32;
                        (*__tmp_23).clone()
                    }
                }
            };
            let _stored: Ptr<i32> = (local.as_pointer());
            observe_5(_assigned, _stored)
        })
    );
    let record: Value<Record_7> = Rc::new(RefCell::new(Record_7 { value: 10 }));
    printf!(
        b"%d\n",
        ({
            let _assigned: i32 = {
                match ({ 99 }, &mut ((*record.borrow_mut()).value)) {
                    (__tmp_24, __tmp_25) => {
                        (*__tmp_25) = __tmp_24;
                        (*__tmp_25).clone()
                    }
                }
            };
            let _stored: Ptr<i32> = (field_ptr!(record.as_pointer(), value));
            observe_5(_assigned, _stored)
        })
    );
    printf!(
        b"%d\n",
        ({
            let _assigned: i32 = {
                match ({ 5 }, &mut ((*record.borrow_mut()).value)) {
                    (__tmp_26, __tmp_27) => {
                        (*__tmp_27) = (((*__tmp_27) as i32) + (__tmp_26 as i32)) as i32;
                        (*__tmp_27).clone()
                    }
                }
            };
            let _stored: Ptr<i32> = (field_ptr!(record.as_pointer(), value));
            observe_5(_assigned, _stored)
        })
    );
    let values: Value<Vec<i32>> = Rc::new(RefCell::new(vec![10; 1_usize]));
    printf!(
        b"%d\n",
        ({
            let _assigned: i32 = {
                match ({ 99 }, &mut ((*values.borrow_mut())[0_usize])) {
                    (__tmp_28, __tmp_29) => {
                        (*__tmp_29) = __tmp_28;
                        (*__tmp_29).clone()
                    }
                }
            };
            let _stored: Ptr<i32> = (values.decay().offset(0_usize));
            observe_5(_assigned, _stored)
        })
    );
    printf!(
        b"%d\n",
        ({
            let _assigned: i32 = {
                match ({ 5 }, &mut ((*values.borrow_mut())[0_usize])) {
                    (__tmp_30, __tmp_31) => {
                        (*__tmp_31) = (((*__tmp_31) as i32) + (__tmp_30 as i32)) as i32;
                        (*__tmp_31).clone()
                    }
                }
            };
            let _stored: Ptr<i32> = (values.decay().offset(0_usize));
            observe_5(_assigned, _stored)
        })
    );
}
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Record_7 {
    #[offset(0)]
    pub value: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_0() });
    ({ test_1() });
    ({ test_2() });
    ({ test_borrows_6() });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = global_3.with(|_| ());
}
