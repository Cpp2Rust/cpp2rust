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
        return (node).clone();
    }
    return Ptr::<node_t>::null();
}
pub fn insert_1(mut node: Ptr<node_t>, mut value: i32) -> Ptr<node_t> {
    if (node).is_null() {
        return Ptr::alloc(node_t {
            left: Ptr::<node_t>::null(),
            right: Ptr::<node_t>::null(),
            value: value,
        });
    }
    if ({ value } < { node.with(|__s| __s.value) }) {
        let __rhs = ({ insert_1(node.with(|__s| __s.left.clone()), value) });
        field!(node, left).write(__rhs);
    } else if ({ value } > { node.with(|__s| __s.value) }) {
        let __rhs = ({ insert_1(node.with(|__s| __s.right.clone()), value) });
        field!(node, right).write(__rhs);
    }
    return (node).clone();
}
pub fn del_2(mut node: Ptr<node_t>) {
    if !((node.with(|__s| __s.left.clone())).is_null()) {
        ({ del_2(node.with(|__s| __s.left.clone())) });
    }
    if !((node.with(|__s| __s.right.clone())).is_null()) {
        ({ del_2(node.with(|__s| __s.right.clone())) });
    }
    node.delete();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut root: Ptr<node_t> = Ptr::alloc(node_t {
        left: Ptr::<node_t>::null(),
        right: Ptr::<node_t>::null(),
        value: 0,
    });
    let __rhs = ({ insert_1((root).clone(), 1) });
    root = __rhs;
    let __rhs = ({ insert_1((root).clone(), 2) });
    root = __rhs;
    let __rhs = ({ insert_1((root).clone(), 3) });
    root = __rhs;
    let __rhs = ({ insert_1((root).clone(), 4) });
    root = __rhs;
    let mut out: bool = (((((({ find_0((root).clone(), 0) }).with(|__s| __s.value) == 0)
        && (({ find_0((root).clone(), 1) }).with(|__s| __s.value) == 1))
        && (({ find_0((root).clone(), 2) }).with(|__s| __s.value) == 2))
        && (({ find_0((root).clone(), 3) }).with(|__s| __s.value) == 3))
        && (({ find_0((root).clone(), 4) }).with(|__s| __s.value) == 4))
        && (({ find_0((root).clone(), 5) }).is_null());
    ({ del_2((root).clone()) });
    assert!(out);
    return 0;
}
pub fn __cpp2rust_init_globals() {}
