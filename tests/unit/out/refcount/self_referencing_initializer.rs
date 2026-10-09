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
pub struct list_head {
    #[offset(0)]
    #[byte_size(8)]
    pub next: Ptr<list_head>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct node {
    #[offset(0)]
    pub value: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub value_ptr: Ptr<i32>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct node_with_defaults {
    #[offset(0)]
    #[byte_size(8)]
    pub next: Ptr<node_with_defaults>,
    #[offset(8)]
    pub value: i32,
}
impl Default for node_with_defaults {
    fn default() -> Self {
        node_with_defaults {
            next: Ptr::<node_with_defaults>::null(),
            value: 3,
        }
    }
}
pub fn init_0(mut l: Ptr<list_head>) -> Ptr<list_head> {
    field!(l, next).write(Ptr::<list_head>::null());
    return l;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let list: Value<list_head> = <Value<list_head>>::default();
    *list.borrow_mut() = list_head {
        next: (list.as_pointer()),
    };
    assert!(({ { (*list.borrow()).next.clone() } } == { (list.as_pointer()) }));
    let n: Value<node> = <Value<node>>::default();
    *n.borrow_mut() = node {
        value: 42,
        value_ptr: (field_ptr!(n.as_pointer(), value)),
    };
    assert!(({ { (*n.borrow()).value_ptr.clone() } } == { (field_ptr!(n.as_pointer(), value)) }));
    { (*n.borrow()).value_ptr.clone() }.write(7);
    assert!(({ (*n.borrow()).value } == 7));
    let arr: Value<Box<[list_head]>> = Rc::new(RefCell::new(
        (0..2)
            .map(|_| <list_head>::default())
            .collect::<Box<[list_head]>>(),
    ));
    *arr.borrow_mut() = Box::new([
        list_head {
            next: ((arr.as_pointer() as Ptr<list_head>).offset(1)),
        },
        list_head {
            next: ((arr.as_pointer() as Ptr<list_head>).offset(0)),
        },
    ]);
    assert!(
        ({ { (*arr.borrow())[(0) as usize].next.clone() } } == {
            ((arr.as_pointer() as Ptr<list_head>).offset(1))
        })
    );
    assert!(
        ({ { (*arr.borrow())[(1) as usize].next.clone() } } == {
            ((arr.as_pointer() as Ptr<list_head>).offset(0))
        })
    );
    let d: Value<node_with_defaults> = <Value<node_with_defaults>>::default();
    *d.borrow_mut() = node_with_defaults {
        next: (d.as_pointer()),
        value: 3,
    };
    assert!(({ { (*d.borrow()).next.clone() } } == { (d.as_pointer()) }));
    assert!(({ (*d.borrow()).value } == 3));
    let called: Value<list_head> = <Value<list_head>>::default();
    *called.borrow_mut() = list_head {
        next: ({ init_0((called.as_pointer())) }),
    };
    assert!(({ { (*called.borrow()).next.clone() } } == { (called.as_pointer()) }));
    let mut size: usize = ::std::mem::size_of::<usize>();
    assert!((size == ::std::mem::size_of::<usize>()));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
