extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct p2_pointer_to_data_member_bytes_round_trip_HasTwoInts {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
pub fn test_0() {
    let pm: Value<isize> = Rc::new(RefCell::new(
        (offset_of_field!(
            p2_pointer_to_data_member_bytes_round_trip_HasTwoInts,
            second
        ) as isize),
    ));
    let bytes: Value<Box<[u8]>> =
        Rc::new(RefCell::new((0..8).map(|_| 0_u8).collect::<Box<[u8]>>()));
    {
        let mut __tmp_0 = { (bytes.decay()).to_any() };
        __tmp_0.memcpy(&(pm.as_pointer()).to_any(), ::std::mem::size_of::<isize>());
        (__tmp_0).clone()
    };
    (*pm.borrow_mut()) =
        (offset_of_field!(p2_pointer_to_data_member_bytes_round_trip_HasTwoInts, first) as isize);
    {
        let mut __tmp_1 = { (pm.as_pointer()).to_any() };
        __tmp_1.memcpy(&(bytes.decay()).to_any(), ::std::mem::size_of::<isize>());
        (__tmp_1).clone()
    };
    assert!(
        ({
            ((*pm.borrow())
                == (offset_of_field!(
                    p2_pointer_to_data_member_bytes_round_trip_HasTwoInts,
                    second
                ) as isize))
        })
    );
    let obj: Value<p2_pointer_to_data_member_bytes_round_trip_HasTwoInts> = Rc::new(RefCell::new(
        p2_pointer_to_data_member_bytes_round_trip_HasTwoInts {
            first: 3,
            second: 4,
        },
    ));
    assert!(({ (((obj.as_pointer()).member::<i32>((*pm.borrow())).read()) == 4) }));
    let null_pm: Value<isize> = Rc::new(RefCell::new(-1_isize));
    {
        let mut __tmp_2 = { (bytes.decay()).to_any() };
        __tmp_2.memcpy(
            &(null_pm.as_pointer()).to_any(),
            ::std::mem::size_of::<isize>(),
        );
        (__tmp_2).clone()
    };
    (*null_pm.borrow_mut()) =
        (offset_of_field!(p2_pointer_to_data_member_bytes_round_trip_HasTwoInts, first) as isize);
    {
        let mut __tmp_3 = { (null_pm.as_pointer()).to_any() };
        __tmp_3.memcpy(&(bytes.decay()).to_any(), ::std::mem::size_of::<isize>());
        (__tmp_3).clone()
    };
    assert!(({ ((*null_pm.borrow()) == -1_isize) }));
}
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct p3_pointer_to_data_member_bytes_copied_to_other_object_HasTwoInts {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
pub fn test_1() {
    let source: Value<isize> = Rc::new(RefCell::new(
        (offset_of_field!(
            p3_pointer_to_data_member_bytes_copied_to_other_object_HasTwoInts,
            second
        ) as isize),
    ));
    let destination: Value<isize> = Rc::new(RefCell::new(
        (offset_of_field!(
            p3_pointer_to_data_member_bytes_copied_to_other_object_HasTwoInts,
            first
        ) as isize),
    ));
    {
        let mut __tmp_4 = { (destination.as_pointer()).to_any() };
        __tmp_4.memcpy(
            &(source.as_pointer()).to_any(),
            ::std::mem::size_of::<isize>(),
        );
        (__tmp_4).clone()
    };
    assert!(({ ((*destination.borrow()) == (*source.borrow())) }));
    let obj: Value<p3_pointer_to_data_member_bytes_copied_to_other_object_HasTwoInts> =
        Rc::new(RefCell::new(
            p3_pointer_to_data_member_bytes_copied_to_other_object_HasTwoInts {
                first: 3,
                second: 4,
            },
        ));
    assert!(
        ({
            (((obj.as_pointer())
                .member::<i32>((*destination.borrow()))
                .read())
                == 4)
        })
    );
    let null_source: Value<isize> = Rc::new(RefCell::new(-1_isize));
    {
        let mut __tmp_5 = { (destination.as_pointer()).to_any() };
        __tmp_5.memcpy(
            &(null_source.as_pointer()).to_any(),
            ::std::mem::size_of::<isize>(),
        );
        (__tmp_5).clone()
    };
    assert!(({ ((*destination.borrow()) == -1_isize) }));
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_0() });
    ({ test_1() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
