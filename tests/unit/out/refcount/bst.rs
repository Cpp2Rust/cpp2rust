extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct node_t {
    pub left: Value<Ptr<node_t>>,
    pub right: Value<Ptr<node_t>>,
    pub value: Value<i32>,
}
impl Clone for node_t {
    fn clone(&self) -> Self {
        let mut this = Self {
            left: Rc::new(RefCell::new((*self.left.borrow()).clone())),
            right: Rc::new(RefCell::new((*self.right.borrow()).clone())),
            value: Rc::new(RefCell::new((*self.value.borrow()))),
        };
        this
    }
}
impl ByteRepr for node_t {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.left.borrow()).to_bytes(&mut buf[0..8]);
        (*self.right.borrow()).to_bytes(&mut buf[8..16]);
        (*self.value.borrow()).to_bytes(&mut buf[16..20]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            left: Rc::new(RefCell::new(<Ptr<node_t>>::from_bytes(&buf[0..8]))),
            right: Rc::new(RefCell::new(<Ptr<node_t>>::from_bytes(&buf[8..16]))),
            value: Rc::new(RefCell::new(<i32>::from_bytes(&buf[16..20]))),
        }
    }
}
pub fn find_0(node: Ptr<node_t>, value: i32) -> Ptr<node_t> {
    let node: Value<Ptr<node_t>> = Rc::new(RefCell::new(node));
    let value: Value<i32> = Rc::new(RefCell::new(value));
    if ({
        let _lhs = (*value.borrow());
        _lhs < (*(*(*node.borrow()).upgrade().deref()).value.borrow())
    }) && (!((*(*(*node.borrow()).upgrade().deref()).left.borrow()).is_null()))
    {
        return ({
            find_0(
                (*(*(*node.borrow()).upgrade().deref()).left.borrow()).clone(),
                (*value.borrow()),
            )
        });
    } else if ({
        let _lhs = (*value.borrow());
        _lhs > (*(*(*node.borrow()).upgrade().deref()).value.borrow())
    }) && (!((*(*(*node.borrow()).upgrade().deref()).right.borrow()).is_null()))
    {
        return ({
            find_0(
                (*(*(*node.borrow()).upgrade().deref()).right.borrow()).clone(),
                (*value.borrow()),
            )
        });
    } else if {
        let _lhs = (*value.borrow());
        _lhs == (*(*(*node.borrow()).upgrade().deref()).value.borrow())
    } {
        return (*node.borrow()).clone();
    }
    return Ptr::<node_t>::null();
}
pub fn insert_1(node: Ptr<node_t>, new_node: Ptr<node_t>) -> Ptr<node_t> {
    let node: Value<Ptr<node_t>> = Rc::new(RefCell::new(node));
    let new_node: Value<Ptr<node_t>> = Rc::new(RefCell::new(new_node));
    if (*node.borrow()).is_null() {
        return (*new_node.borrow()).clone();
    }
    if {
        let _lhs = (*(*(*new_node.borrow()).upgrade().deref()).value.borrow());
        _lhs < (*(*(*node.borrow()).upgrade().deref()).value.borrow())
    } {
        let __rhs = ({
            insert_1(
                (*(*(*node.borrow()).upgrade().deref()).left.borrow()).clone(),
                (*new_node.borrow()).clone(),
            )
        });
        (*(*(*node.borrow()).upgrade().deref()).left.borrow_mut()) = __rhs;
    } else if {
        let _lhs = (*(*(*new_node.borrow()).upgrade().deref()).value.borrow());
        _lhs > (*(*(*node.borrow()).upgrade().deref()).value.borrow())
    } {
        let __rhs = ({
            insert_1(
                (*(*(*node.borrow()).upgrade().deref()).right.borrow()).clone(),
                (*new_node.borrow()).clone(),
            )
        });
        (*(*(*node.borrow()).upgrade().deref()).right.borrow_mut()) = __rhs;
    }
    return (*node.borrow()).clone();
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let tree: Value<Option<Value<node_t>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(node_t {
            left: Rc::new(RefCell::new(Ptr::<node_t>::null())),
            right: Rc::new(RefCell::new(Ptr::<node_t>::null())),
            value: Rc::new(RefCell::new(0)),
        })))));
    let n1: Value<Option<Value<node_t>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(node_t {
            left: Rc::new(RefCell::new(Ptr::<node_t>::null())),
            right: Rc::new(RefCell::new(Ptr::<node_t>::null())),
            value: Rc::new(RefCell::new(1)),
        })))));
    let n2: Value<Option<Value<node_t>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(node_t {
            left: Rc::new(RefCell::new(Ptr::<node_t>::null())),
            right: Rc::new(RefCell::new(Ptr::<node_t>::null())),
            value: Rc::new(RefCell::new(2)),
        })))));
    let n3: Value<Option<Value<node_t>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(node_t {
            left: Rc::new(RefCell::new(Ptr::<node_t>::null())),
            right: Rc::new(RefCell::new(Ptr::<node_t>::null())),
            value: Rc::new(RefCell::new(3)),
        })))));
    let n4: Value<Option<Value<node_t>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(node_t {
            left: Rc::new(RefCell::new(Ptr::<node_t>::null())),
            right: Rc::new(RefCell::new(Ptr::<node_t>::null())),
            value: Rc::new(RefCell::new(4)),
        })))));
    let ptr1: Value<Ptr<node_t>> = Rc::new(RefCell::new(((*tree.borrow()).as_pointer())));
    let __rhs = ({ insert_1((*ptr1.borrow()).clone(), ((*n1.borrow()).as_pointer())) });
    (*ptr1.borrow_mut()) = __rhs;
    let __rhs = ({ insert_1((*ptr1.borrow()).clone(), ((*n2.borrow()).as_pointer())) });
    (*ptr1.borrow_mut()) = __rhs;
    let __rhs = ({ insert_1((*ptr1.borrow()).clone(), ((*n3.borrow()).as_pointer())) });
    (*ptr1.borrow_mut()) = __rhs;
    let __rhs = ({ insert_1((*ptr1.borrow()).clone(), ((*n4.borrow()).as_pointer())) });
    (*ptr1.borrow_mut()) = __rhs;
    return ((((((((*(*({ find_0((*ptr1.borrow()).clone(), 0) }).upgrade().deref())
        .value
        .borrow())
        == 0)
        && ((*(*({ find_0((*ptr1.borrow()).clone(), 1) }).upgrade().deref())
            .value
            .borrow())
            == 1))
        && ((*(*({ find_0((*ptr1.borrow()).clone(), 2) }).upgrade().deref())
            .value
            .borrow())
            == 2))
        && ((*(*({ find_0((*ptr1.borrow()).clone(), 3) }).upgrade().deref())
            .value
            .borrow())
            == 3))
        && ((*(*({ find_0((*ptr1.borrow()).clone(), 4) }).upgrade().deref())
            .value
            .borrow())
            == 4))
        && (({ find_0((*ptr1.borrow()).clone(), 5) }).is_null())) as i32);
}
