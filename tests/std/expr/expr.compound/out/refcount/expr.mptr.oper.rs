extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct p3_arrow_star_HasTwoInts {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
pub fn test_0() {
    let mut pm: isize = (offset_of_field!(p3_arrow_star_HasTwoInts, second) as isize);
    let obj: Value<p3_arrow_star_HasTwoInts> = Rc::new(RefCell::new(p3_arrow_star_HasTwoInts {
        first: 3,
        second: 4,
    }));
    let mut p: Ptr<p3_arrow_star_HasTwoInts> = (obj.as_pointer());
    assert!((((p).member::<i32>(pm).read()) == 4));
    (p).member::<i32>(pm).write(5);
    assert!(({ ({ (*obj.borrow()).second } == 5) }));
    assert!(({ ({ (*obj.borrow()).first } == 3) }));
    assert!((((p).member::<i32>(pm)) == ((p).member::<i32>(pm))));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct p4_object_expression_sequenced_first_HasValue {
    #[offset(0)]
    pub value: i32,
}
thread_local!(
    pub static sequence_1: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static object_stamp_2: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static member_stamp_3: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static object_4: Value<p4_object_expression_sequenced_first_HasValue> = Rc::new(
        RefCell::new(p4_object_expression_sequenced_first_HasValue { value: 7 }),
    );
);
pub fn select_object_5() -> Ptr<p4_object_expression_sequenced_first_HasValue> {
    object_stamp_2.with(|rc| {
        *rc.borrow_mut() = (*(*sequence_1.with(Value::clone).borrow_mut()).prefix_inc())
    });
    return object_4.with(|v| v.as_pointer());
}
pub fn select_member_6() -> isize {
    member_stamp_3.with(|rc| {
        *rc.borrow_mut() = (*(*sequence_1.with(Value::clone).borrow_mut()).prefix_inc())
    });
    return (offset_of_field!(p4_object_expression_sequenced_first_HasValue, value) as isize);
}
pub fn test_7() {
    assert!(
        ((({ select_object_5() })
            .member::<i32>(({ select_member_6() }))
            .read())
            == 7)
    );
    assert!((object_stamp_2.with(|rc| *rc.borrow()) == 1));
    assert!((member_stamp_3.with(|rc| *rc.borrow()) == 2));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct p6_data_member_of_lvalue_is_lvalue_HasTwoInts {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
pub fn test_8() {
    let mut pm: isize =
        (offset_of_field!(p6_data_member_of_lvalue_is_lvalue_HasTwoInts, second) as isize);
    let obj: Value<p6_data_member_of_lvalue_is_lvalue_HasTwoInts> = Rc::new(RefCell::new(
        p6_data_member_of_lvalue_is_lvalue_HasTwoInts {
            first: 3,
            second: 4,
        },
    ));
    assert!((((obj.as_pointer()).member::<i32>(pm)) == (field_ptr!(obj.as_pointer(), second))));
    let ref_: Ptr<i32> = (obj.as_pointer()).member::<i32>(pm);
    ref_.write(5);
    assert!(({ ({ (*obj.borrow()).second } == 5) }));
    assert!(({ ({ (*obj.borrow()).first } == 3) }));
}
thread_local!(
    pub static copyCount_9: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static moveCount_10: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(1)]
pub struct p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves {}
impl p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves {
    pub fn copy_from(_a0: Ptr<p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves>) -> Self {
        let __this: p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves = Self {};
        (*copyCount_9.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn move_from(_a0: Ptr<p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves>) -> Self {
        let __this: p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves = Self {};
        (*moveCount_10.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves {
    fn clone(&self) -> Self {
        let __src: Value<p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves> = Rc::new(
            RefCell::new(p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves {}),
        );
        p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves::copy_from(__src.as_pointer())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(1)]
pub struct p6_data_member_of_rvalue_is_xvalue_HoldsCounted {
    #[offset(0)]
    #[byte_size(1)]
    pub member: p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves,
}
impl p6_data_member_of_rvalue_is_xvalue_HoldsCounted {
    pub fn move_from(_a0: Ptr<p6_data_member_of_rvalue_is_xvalue_HoldsCounted>) -> Self {
        Self {
            member: p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves::move_from({
                field_ptr!(_a0, member)
            }),
        }
    }
}
pub fn test_11() {
    let mut pm: isize =
        (offset_of_field!(p6_data_member_of_rvalue_is_xvalue_HoldsCounted, member) as isize);
    let holder: Value<p6_data_member_of_rvalue_is_xvalue_HoldsCounted> = Rc::new(RefCell::new(
        <p6_data_member_of_rvalue_is_xvalue_HoldsCounted>::default(),
    ));
    let mut from_lvalue: p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves =
        p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves::copy_from({
            (holder.as_pointer())
                .member::<p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves>(pm)
        });
    assert!((copyCount_9.with(|rc| *rc.borrow()) == 1));
    assert!((moveCount_10.with(|rc| *rc.borrow()) == 0));
    let mut from_xvalue: p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves =
        p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves::move_from({
            (holder.as_pointer())
                .member::<p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves>(pm)
        });
    assert!((copyCount_9.with(|rc| *rc.borrow()) == 1));
    assert!((moveCount_10.with(|rc| *rc.borrow()) == 1));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_0() });
    ({ test_7() });
    ({ test_8() });
    ({ test_11() });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = sequence_1.with(|_| ());
    let _ = object_stamp_2.with(|_| ());
    let _ = member_stamp_3.with(|_| ());
    let _ = object_4.with(|_| ());
    let _ = copyCount_9.with(|_| ());
    let _ = moveCount_10.with(|_| ());
}
