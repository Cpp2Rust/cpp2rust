extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct p2_pointer_to_array_member_HasIntArray {
    #[offset(0)]
    pub before: i32,
    #[offset(4)]
    #[byte_size(12)]
    pub elements: Box<[i32]>,
}
impl Default for p2_pointer_to_array_member_HasIntArray {
    fn default() -> Self {
        {
            p2_pointer_to_array_member_HasIntArray {
                before: 0_i32,
                elements: (0..3).map(|_| 0_i32).collect::<Box<[i32]>>(),
            }
        }
    }
}
pub fn test_0() {
    let mut pm: isize =
        (offset_of_field!(p2_pointer_to_array_member_HasIntArray, elements) as isize);
    let obj: Value<p2_pointer_to_array_member_HasIntArray> =
        Rc::new(RefCell::new(p2_pointer_to_array_member_HasIntArray {
            before: 1,
            elements: Box::new([2, 3, 4]),
        }));
    assert!((((obj.as_pointer()).member::<Box<[i32]>>(pm).read())[(0) as usize] == 2));
    assert!((((obj.as_pointer()).member::<Box<[i32]>>(pm).read())[(2) as usize] == 4));
    index!((obj.as_pointer()).member::<Box<[i32]>>(pm), 1).write(7);
    assert!(((array_elem!(obj.as_pointer(), elements, 1).read()) == 7));
    assert!(({ ({ (*obj.borrow()).before } == 1) }));
}
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct p2_pointer_to_data_member_HasTwoInts {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
pub fn test_1() {
    let mut pm: isize = (offset_of_field!(p2_pointer_to_data_member_HasTwoInts, first) as isize);
    let obj: Value<p2_pointer_to_data_member_HasTwoInts> =
        Rc::new(RefCell::new(p2_pointer_to_data_member_HasTwoInts {
            first: 3,
            second: 4,
        }));
    assert!((((obj.as_pointer()).member::<i32>(pm).read()) == 3));
    (obj.as_pointer()).member::<i32>(pm).write(5);
    assert!(({ ({ (*obj.borrow()).first } == 5) }));
    assert!(({ ({ (*obj.borrow()).second } == 4) }));
    pm = (offset_of_field!(p2_pointer_to_data_member_HasTwoInts, second) as isize);
    assert!((((obj.as_pointer()).member::<i32>(pm).read()) == 4));
    (obj.as_pointer()).member::<i32>(pm).write(6);
    assert!(({ ({ (*obj.borrow()).first } == 5) }));
    assert!(({ ({ (*obj.borrow()).second } == 6) }));
}
thread_local!(
    pub static pm_2: Value<isize> = Rc::new(RefCell::new(-1_isize));
);
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct p2_pointer_to_member_of_incomplete_class_CompletedLater {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
pub fn test_3() {
    assert!(((pm_2.with(|rc| *rc.borrow())) == -1_isize));
    pm_2.with(|rc| {
        *rc.borrow_mut() = (offset_of_field!(
            p2_pointer_to_member_of_incomplete_class_CompletedLater,
            second
        ) as isize)
    });
    let obj: Value<p2_pointer_to_member_of_incomplete_class_CompletedLater> = Rc::new(
        RefCell::new(p2_pointer_to_member_of_incomplete_class_CompletedLater {
            first: 3,
            second: 4,
        }),
    );
    assert!(
        (((obj.as_pointer())
            .member::<i32>(pm_2.with(|rc| *rc.borrow()))
            .read())
            == 4)
    );
}
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct p2_pointer_to_member_of_pointer_to_member_type_HasTwoInts {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct p2_pointer_to_member_of_pointer_to_member_type_HoldsSelector {
    #[offset(0)]
    #[byte_size(8)]
    pub selector: isize,
}
impl Default for p2_pointer_to_member_of_pointer_to_member_type_HoldsSelector {
    fn default() -> Self {
        { p2_pointer_to_member_of_pointer_to_member_type_HoldsSelector { selector: -1_isize } }
    }
}
pub fn test_4() {
    let mut pm: isize = (offset_of_field!(
        p2_pointer_to_member_of_pointer_to_member_type_HoldsSelector,
        selector
    ) as isize);
    let holder: Value<p2_pointer_to_member_of_pointer_to_member_type_HoldsSelector> =
        Rc::new(RefCell::new(
            p2_pointer_to_member_of_pointer_to_member_type_HoldsSelector {
                selector: (offset_of_field!(
                    p2_pointer_to_member_of_pointer_to_member_type_HasTwoInts,
                    second
                ) as isize),
            },
        ));
    let obj: Value<p2_pointer_to_member_of_pointer_to_member_type_HasTwoInts> = Rc::new(
        RefCell::new(p2_pointer_to_member_of_pointer_to_member_type_HasTwoInts {
            first: 3,
            second: 4,
        }),
    );
    assert!(
        (((obj.as_pointer())
            .member::<i32>(((holder.as_pointer()).member::<isize>(pm).read()))
            .read())
            == 4)
    );
    (holder.as_pointer()).member::<isize>(pm).write(
        (offset_of_field!(
            p2_pointer_to_member_of_pointer_to_member_type_HasTwoInts,
            first
        ) as isize),
    );
    assert!(
        ({
            (((obj.as_pointer())
                .member::<i32>(({ (*holder.borrow()).selector }))
                .read())
                == 3)
        })
    );
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_0() });
    ({ test_1() });
    ({ test_3() });
    ({ test_4() });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = pm_2.with(|_| ());
}
