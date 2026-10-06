extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct node_t {
    #[offset(0)]
    #[byte_size(8)]
    pub left: Ptr<node_t>,
    #[offset(8)]
    #[byte_size(8)]
    pub right: Ptr<node_t>,
    #[offset(16)]
    pub value: i32,
}
pub fn find_0(mut node: Ptr<node_t>, mut value: i32) -> Ptr<node_t> {
    if ({ value } < { node.with(|__s| __s.value) })
        && (!((node.with(|__s| __s.left.clone())).is_null()))
    {
        return ({ find_0(node.with(|__s| __s.left.clone()), value) });
    } else if ({ value } > { node.with(|__s| __s.value) })
        && (!((node.with(|__s| __s.right.clone())).is_null()))
    {
        return ({ find_0(node.with(|__s| __s.right.clone()), value) });
    } else if ({ value } == { node.with(|__s| __s.value) }) {
        return node;
    }
    return Ptr::<node_t>::null();
}
pub fn insert_1(mut node: Ptr<node_t>, mut new_node: Ptr<node_t>) -> Ptr<node_t> {
    if (node).is_null() {
        return new_node;
    }
    if ({ new_node.with(|__s| __s.value) } < { node.with(|__s| __s.value) }) {
        let __rhs = ({ insert_1(node.with(|__s| __s.left.clone()), (new_node).clone()) });
        field!(node, left).write(__rhs);
    } else if ({ new_node.with(|__s| __s.value) } > { node.with(|__s| __s.value) }) {
        let __rhs = ({ insert_1(node.with(|__s| __s.right.clone()), (new_node).clone()) });
        field!(node, right).write(__rhs);
    }
    return node;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut tree: Option<Value<node_t>> = Some(Rc::new(RefCell::new(node_t {
        left: Ptr::<node_t>::null(),
        right: Ptr::<node_t>::null(),
        value: 0,
    })));
    let mut n1: Option<Value<node_t>> = Some(Rc::new(RefCell::new(node_t {
        left: Ptr::<node_t>::null(),
        right: Ptr::<node_t>::null(),
        value: 1,
    })));
    let mut n2: Option<Value<node_t>> = Some(Rc::new(RefCell::new(node_t {
        left: Ptr::<node_t>::null(),
        right: Ptr::<node_t>::null(),
        value: 2,
    })));
    let mut n3: Option<Value<node_t>> = Some(Rc::new(RefCell::new(node_t {
        left: Ptr::<node_t>::null(),
        right: Ptr::<node_t>::null(),
        value: 3,
    })));
    let mut n4: Option<Value<node_t>> = Some(Rc::new(RefCell::new(node_t {
        left: Ptr::<node_t>::null(),
        right: Ptr::<node_t>::null(),
        value: 4,
    })));
    let mut ptr1: Ptr<node_t> = (tree.as_pointer());
    let __rhs = ({ insert_1((ptr1).clone(), (n1.as_pointer())) });
    ptr1 = __rhs;
    let __rhs = ({ insert_1((ptr1).clone(), (n2.as_pointer())) });
    ptr1 = __rhs;
    let __rhs = ({ insert_1((ptr1).clone(), (n3.as_pointer())) });
    ptr1 = __rhs;
    let __rhs = ({ insert_1((ptr1).clone(), (n4.as_pointer())) });
    ptr1 = __rhs;
    assert!(
        (((((({ find_0((ptr1).clone(), 0,) }).with(|__s| __s.value) == 0)
            && (({ find_0((ptr1).clone(), 1,) }).with(|__s| __s.value) == 1))
            && (({ find_0((ptr1).clone(), 2,) }).with(|__s| __s.value) == 2))
            && (({ find_0((ptr1).clone(), 3,) }).with(|__s| __s.value) == 3))
            && (({ find_0((ptr1).clone(), 4,) }).with(|__s| __s.value) == 4))
            && (({ find_0((ptr1).clone(), 5,) }).is_null())
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
