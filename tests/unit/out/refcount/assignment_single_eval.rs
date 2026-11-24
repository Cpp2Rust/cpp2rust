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
pub fn test_comma_3() {
    let x: Value<u32> = Rc::new(RefCell::new(10_u32));
    let mut i: i32 = 0;
    {
        match (
            { 5_u32 },
            &({
                i.prefix_inc();
                x.as_pointer()
            }),
        ) {
            (__tmp_16, __tmp_17) => __tmp_17.with_mut(|__tmp_17| {
                (*__tmp_17) = (((*__tmp_17) as u32).wrapping_add((__tmp_16 as u32))) as u32;
            }),
        }
    };
    printf!(b"direct void %d %d\n", i, ((*x.borrow()) as i32));
    let mut result: u32 = ({
        match (
            { 2_u32 },
            &({
                i.prefix_inc();
                x.as_pointer()
            }),
        ) {
            (__tmp_18, __tmp_19) => __tmp_19.with_mut(|__tmp_19| {
                (*__tmp_19) = (((*__tmp_19) as u32).wrapping_add((__tmp_18 as u32))) as u32;
                (*__tmp_19).clone()
            }),
        }
    });
    printf!(
        b"direct compound %d %d %d\n",
        i,
        (result as i32),
        ((*x.borrow()) as i32)
    );
    result = ({
        match (
            { 99_u32 },
            &({
                i.prefix_inc();
                x.as_pointer()
            }),
        ) {
            (__tmp_20, __tmp_21) => __tmp_21.with_mut(|__tmp_21| {
                (*__tmp_21) = __tmp_20;
                (*__tmp_21).clone()
            }),
        }
    });
    printf!(
        b"direct simple %d %d %d\n",
        i,
        (result as i32),
        ((*x.borrow()) as i32)
    );
    let mut p: Ptr<u32> = (x.as_pointer());
    {
        match (
            { 5_u32 },
            &({
                i.prefix_inc();
                (p).clone()
            }),
        ) {
            (__tmp_22, __tmp_23) => __tmp_23.with_mut(|__tmp_23| {
                (*__tmp_23) = (((*__tmp_23) as u32).wrapping_add((__tmp_22 as u32))) as u32;
            }),
        }
    };
    printf!(b"indirect void %d %d\n", i, ((p.read()) as i32));
    let __rhs = ({
        match (
            { 2_u32 },
            &({
                i.prefix_inc();
                (p).clone()
            }),
        ) {
            (__tmp_24, __tmp_25) => __tmp_25.with_mut(|__tmp_25| {
                (*__tmp_25) = (((*__tmp_25) as u32).wrapping_add((__tmp_24 as u32))) as u32;
                (*__tmp_25).clone()
            }),
        }
    });
    result = __rhs;
    printf!(
        b"indirect compound %d %d %d\n",
        i,
        (result as i32),
        ((p.read()) as i32)
    );
    let __rhs = ({
        match (
            { 99_u32 },
            &({
                i.prefix_inc();
                (p).clone()
            }),
        ) {
            (__tmp_26, __tmp_27) => __tmp_27.with_mut(|__tmp_27| {
                (*__tmp_27) = __tmp_26;
                (*__tmp_27).clone()
            }),
        }
    });
    result = __rhs;
    printf!(
        b"indirect simple %d %d %d\n",
        i,
        (result as i32),
        ((p.read()) as i32)
    );
}
pub fn test_comma_4() {
    let x: Value<i16> = Rc::new(RefCell::new(10_i16));
    let mut i: i32 = 0;
    {
        match (
            { 5 },
            &({
                i.prefix_inc();
                x.as_pointer()
            }),
        ) {
            (__tmp_28, __tmp_29) => __tmp_29.with_mut(|__tmp_29| {
                (*__tmp_29) = (((*__tmp_29) as i32) + (__tmp_28 as i32)) as i16;
            }),
        }
    };
    printf!(b"direct void %d %d\n", i, ((*x.borrow()) as i32));
    let mut result: i16 = ({
        match (
            { 2 },
            &({
                i.prefix_inc();
                x.as_pointer()
            }),
        ) {
            (__tmp_30, __tmp_31) => __tmp_31.with_mut(|__tmp_31| {
                (*__tmp_31) = (((*__tmp_31) as i32) + (__tmp_30 as i32)) as i16;
                (*__tmp_31).clone()
            }),
        }
    });
    printf!(
        b"direct compound %d %d %d\n",
        i,
        (result as i32),
        ((*x.borrow()) as i32)
    );
    result = ({
        match (
            { 99_i16 },
            &({
                i.prefix_inc();
                x.as_pointer()
            }),
        ) {
            (__tmp_32, __tmp_33) => __tmp_33.with_mut(|__tmp_33| {
                (*__tmp_33) = __tmp_32;
                (*__tmp_33).clone()
            }),
        }
    });
    printf!(
        b"direct simple %d %d %d\n",
        i,
        (result as i32),
        ((*x.borrow()) as i32)
    );
    let mut p: Ptr<i16> = (x.as_pointer());
    {
        match (
            { 5 },
            &({
                i.prefix_inc();
                (p).clone()
            }),
        ) {
            (__tmp_34, __tmp_35) => __tmp_35.with_mut(|__tmp_35| {
                (*__tmp_35) = (((*__tmp_35) as i32) + (__tmp_34 as i32)) as i16;
            }),
        }
    };
    printf!(b"indirect void %d %d\n", i, ((p.read()) as i32));
    let __rhs = ({
        match (
            { 2 },
            &({
                i.prefix_inc();
                (p).clone()
            }),
        ) {
            (__tmp_36, __tmp_37) => __tmp_37.with_mut(|__tmp_37| {
                (*__tmp_37) = (((*__tmp_37) as i32) + (__tmp_36 as i32)) as i16;
                (*__tmp_37).clone()
            }),
        }
    });
    result = __rhs;
    printf!(
        b"indirect compound %d %d %d\n",
        i,
        (result as i32),
        ((p.read()) as i32)
    );
    let __rhs = ({
        match (
            { 99_i16 },
            &({
                i.prefix_inc();
                (p).clone()
            }),
        ) {
            (__tmp_38, __tmp_39) => __tmp_39.with_mut(|__tmp_39| {
                (*__tmp_39) = __tmp_38;
                (*__tmp_39).clone()
            }),
        }
    });
    result = __rhs;
    printf!(
        b"indirect simple %d %d %d\n",
        i,
        (result as i32),
        ((p.read()) as i32)
    );
}
thread_local!(
    pub static global_5: Value<i32> = Rc::new(RefCell::new(10));
);
pub fn observe_global_6(mut assigned: i32) -> i32 {
    return (assigned + global_5.with(|rc| *rc.borrow()));
}
pub fn observe_7(mut assigned: i32, mut stored: Ptr<i32>) -> i32 {
    return ({ assigned } + { (stored.read()) });
}
pub fn test_borrows_8() {
    printf!(
        b"%d\n",
        ({
            observe_global_6({
                match ({ 99 }, &mut (*global_5.with(Value::clone).borrow_mut())) {
                    (__tmp_40, __tmp_41) => {
                        (*__tmp_41) = __tmp_40;
                        (*__tmp_41).clone()
                    }
                }
            })
        })
    );
    printf!(
        b"%d\n",
        ({
            observe_global_6({
                match ({ 5 }, &mut (*global_5.with(Value::clone).borrow_mut())) {
                    (__tmp_42, __tmp_43) => {
                        (*__tmp_43) = (((*__tmp_43) as i32) + (__tmp_42 as i32)) as i32;
                        (*__tmp_43).clone()
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
                    (__tmp_44, __tmp_45) => {
                        (*__tmp_45) = __tmp_44;
                        (*__tmp_45).clone()
                    }
                }
            };
            let _stored: Ptr<i32> = (local.as_pointer());
            observe_7(_assigned, _stored)
        })
    );
    printf!(
        b"%d\n",
        ({
            let _assigned: i32 = {
                match ({ 5 }, &mut (*local.borrow_mut())) {
                    (__tmp_46, __tmp_47) => {
                        (*__tmp_47) = (((*__tmp_47) as i32) + (__tmp_46 as i32)) as i32;
                        (*__tmp_47).clone()
                    }
                }
            };
            let _stored: Ptr<i32> = (local.as_pointer());
            observe_7(_assigned, _stored)
        })
    );
    let record: Value<Record_9> = Rc::new(RefCell::new(Record_9 { value: 10 }));
    printf!(
        b"%d\n",
        ({
            let _assigned: i32 = {
                match ({ 99 }, &mut ((*record.borrow_mut()).value)) {
                    (__tmp_48, __tmp_49) => {
                        (*__tmp_49) = __tmp_48;
                        (*__tmp_49).clone()
                    }
                }
            };
            let _stored: Ptr<i32> = (field_ptr!(record.as_pointer(), value));
            observe_7(_assigned, _stored)
        })
    );
    printf!(
        b"%d\n",
        ({
            let _assigned: i32 = {
                match ({ 5 }, &mut ((*record.borrow_mut()).value)) {
                    (__tmp_50, __tmp_51) => {
                        (*__tmp_51) = (((*__tmp_51) as i32) + (__tmp_50 as i32)) as i32;
                        (*__tmp_51).clone()
                    }
                }
            };
            let _stored: Ptr<i32> = (field_ptr!(record.as_pointer(), value));
            observe_7(_assigned, _stored)
        })
    );
    let values: Value<Vec<i32>> = Rc::new(RefCell::new(vec![10; 1_usize]));
    printf!(
        b"%d\n",
        ({
            let _assigned: i32 = {
                match ({ 99 }, &mut ((*values.borrow_mut())[0_usize])) {
                    (__tmp_52, __tmp_53) => {
                        (*__tmp_53) = __tmp_52;
                        (*__tmp_53).clone()
                    }
                }
            };
            let _stored: Ptr<i32> = (values.decay().offset(0_usize));
            observe_7(_assigned, _stored)
        })
    );
    printf!(
        b"%d\n",
        ({
            let _assigned: i32 = {
                match ({ 5 }, &mut ((*values.borrow_mut())[0_usize])) {
                    (__tmp_54, __tmp_55) => {
                        (*__tmp_55) = (((*__tmp_55) as i32) + (__tmp_54 as i32)) as i32;
                        (*__tmp_55).clone()
                    }
                }
            };
            let _stored: Ptr<i32> = (values.decay().offset(0_usize));
            observe_7(_assigned, _stored)
        })
    );
}
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Record_9 {
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
    ({ test_comma_3() });
    ({ test_comma_4() });
    let values: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([10_u32, 20_u32])));
    let ref_: Ptr<u32> = values.decay();
    let mut i: i32 = 0;
    let mut result: u32 = ({
        match (
            { 5_u32 },
            &(elem!(
                {
                    i.prefix_inc();
                    (ref_).clone()
                },
                0
            )),
        ) {
            (__tmp_56, __tmp_57) => __tmp_57.with_mut(|__tmp_57| {
                (*__tmp_57) = (((*__tmp_57) as u32).wrapping_add((__tmp_56 as u32))) as u32;
                (*__tmp_57).clone()
            }),
        }
    });
    printf!(
        b"array reference %d %u %u\n",
        i,
        result,
        (*values.borrow())[(0) as usize]
    );
    ({ test_borrows_8() });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = global_5.with(|_| ());
}
