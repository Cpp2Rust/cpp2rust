extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct HasThreeInts {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
    #[offset(8)]
    pub third: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let members: Value<Box<[MemberPtr<HasThreeInts, i32>]>> = Rc::new(RefCell::new(Box::new([
        member_ptr!(HasThreeInts, third),
        member_ptr!(HasThreeInts, first),
        member_ptr!(HasThreeInts, second),
    ])));
    let obj: Value<HasThreeInts> = Rc::new(RefCell::new(HasThreeInts {
        first: 4,
        second: 5,
        third: 6,
    }));
    assert!(
        ({
            (((obj.as_pointer())
                .member((*members.borrow())[(0) as usize])
                .read())
                == 6)
        })
    );
    assert!(
        ({
            (((obj.as_pointer())
                .member((*members.borrow())[(1) as usize])
                .read())
                == 4)
        })
    );
    assert!(
        ({
            (((obj.as_pointer())
                .member((*members.borrow())[(2) as usize])
                .read())
                == 5)
        })
    );
    let mut element: Ptr<MemberPtr<HasThreeInts, i32>> = members.decay();
    assert!(({ element.offset((1) as isize) } == { (members.decay().offset(1)) }));
    assert!((((obj.as_pointer()).member((elem!(element, 1).read())).read()) == 4));
    element.prefix_inc();
    element.prefix_inc();
    assert!(({ (element).clone() } == { (members.decay().offset(2)) }));
    assert!((((obj.as_pointer()).member((element.read())).read()) == 5));
    (*members.borrow_mut())[(2) as usize] = member_ptr!(HasThreeInts, third);
    assert!((((obj.as_pointer()).member((element.read())).read()) == 6));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
