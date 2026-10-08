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
    let arr: Value<Box<[AnyPtr]>> = Rc::new(RefCell::new(
        (0..2).map(|_| AnyPtr::default()).collect::<Box<[AnyPtr]>>(),
    ));
    *arr.borrow_mut() = Box::new([
        (((arr.as_pointer() as Ptr<AnyPtr>).offset(1)) as Ptr<AnyPtr>).to_any(),
        (((arr.as_pointer() as Ptr<AnyPtr>).offset(0)) as Ptr<AnyPtr>).to_any(),
    ]);
    assert!(
        ((*arr.borrow())[(0) as usize]
            == (((arr.as_pointer() as Ptr<AnyPtr>).offset(1)) as Ptr<AnyPtr>).to_any())
    );
    assert!(
        ((*arr.borrow())[(1) as usize]
            == (((arr.as_pointer() as Ptr<AnyPtr>).offset(0)) as Ptr<AnyPtr>).to_any())
    );
    let p: Value<AnyPtr> = Rc::new(RefCell::new(AnyPtr::default()));
    *p.borrow_mut() = ((p.as_pointer()) as Ptr<AnyPtr>).to_any();
    assert!(((*p.borrow()) == ((p.as_pointer()) as Ptr<AnyPtr>).to_any()));
    let d: Value<node_with_defaults> = <Value<node_with_defaults>>::default();
    *d.borrow_mut() = node_with_defaults {
        next: (d.as_pointer()),
        value: 3,
    };
    assert!(({ { (*d.borrow()).next.clone() } } == { (d.as_pointer()) }));
    assert!(({ (*d.borrow()).value } == 3));
    let mut heap: Ptr<list_head> =
        libcc2rs::malloc_refcount(8usize).reinterpret_cast::<list_head>();
    assert!(!((heap).is_null()));
    field!(heap, next).write({ (heap).clone() });
    assert!(({ heap.with(|__s| __s.next.clone()) } == { (heap).clone() }));
    libcc2rs::free_refcount((heap).to_any());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
