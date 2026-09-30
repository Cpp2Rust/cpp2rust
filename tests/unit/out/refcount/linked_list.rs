extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Node {
    #[offset(0)]
    pub val: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub next: Ptr<Node>,
}
pub fn Find_0(head: Ptr<Node>, idx: i32) -> Ptr<Node> {
    let head: Value<Ptr<Node>> = Rc::new(RefCell::new(head));
    let idx: Value<i32> = Rc::new(RefCell::new(idx));
    let curr: Value<Ptr<Node>> = Rc::new(RefCell::new((*head.borrow()).clone()));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*idx.borrow())) {
        let __rhs = (*curr.borrow()).with(|__s| __s.next.clone());
        (*curr.borrow_mut()) = __rhs;
        (*i.borrow_mut()).postfix_inc();
    }
    return (*curr.borrow()).clone();
}
pub fn Append_1(head: Ptr<Node>, new_node: Ptr<Node>) {
    let curr: Value<Ptr<Node>> = Rc::new(RefCell::new((head).clone()));
    'loop_: while !(((*curr.borrow()).with(|__s| __s.next.clone())).is_null()) {
        let __rhs = (*curr.borrow()).with(|__s| __s.next.clone());
        (*curr.borrow_mut()) = __rhs;
    }
    ({ NodeImpl::SetNext(&(*curr.borrow()), (new_node).clone()) });
}
pub fn Delete_2(head: Ptr<Node>, val: i32) -> Ptr<Node> {
    let head: Value<Ptr<Node>> = Rc::new(RefCell::new(head));
    let val: Value<i32> = Rc::new(RefCell::new(val));
    let curr: Value<Ptr<Node>> = Rc::new(RefCell::new((*head.borrow()).clone()));
    let prev: Value<Ptr<Node>> = Rc::new(RefCell::new(Ptr::<Node>::null()));
    'loop_: while !((*curr.borrow()).is_null()) {
        if {
            let _lhs = (*curr.borrow()).with(|__s| __s.val);
            _lhs == (*val.borrow())
        } {
            if !((*prev.borrow()).is_null()) {
                let __rhs = (*curr.borrow()).with(|__s| __s.next.clone());
                field!((*prev.borrow()), next).write(__rhs);
                return (*head.borrow()).clone();
            } else {
                return (*curr.borrow()).with(|__s| __s.next.clone());
            }
        }
        (*prev.borrow_mut()) = (*curr.borrow()).clone();
        let __rhs = (*curr.borrow()).with(|__s| __s.next.clone());
        (*curr.borrow_mut()) = __rhs;
    }
    return (*head.borrow()).clone();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let n0: Value<Node> = Rc::new(RefCell::new(Node {
        val: 5,
        next: Ptr::<Node>::null(),
    }));
    let head: Value<Ptr<Node>> = Rc::new(RefCell::new((n0.as_pointer())));
    let n1: Value<Node> = Rc::new(RefCell::new(Node {
        val: 4,
        next: Ptr::<Node>::null(),
    }));
    let n2: Value<Node> = Rc::new(RefCell::new(Node {
        val: 3,
        next: Ptr::<Node>::null(),
    }));
    let n3: Value<Node> = Rc::new(RefCell::new(Node {
        val: 2,
        next: Ptr::<Node>::null(),
    }));
    let n4: Value<Node> = Rc::new(RefCell::new(Node {
        val: 1,
        next: Ptr::<Node>::null(),
    }));
    let n5: Value<Node> = Rc::new(RefCell::new(Node {
        val: 0,
        next: Ptr::<Node>::null(),
    }));
    let n6: Value<Node> = Rc::new(RefCell::new(Node {
        val: -1_i32,
        next: Ptr::<Node>::null(),
    }));
    let n7: Value<Node> = Rc::new(RefCell::new(Node {
        val: -2_i32,
        next: Ptr::<Node>::null(),
    }));
    ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _new_node: Ptr<Node> = n1.as_pointer();
        Append_1(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _new_node: Ptr<Node> = n2.as_pointer();
        Append_1(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _new_node: Ptr<Node> = n3.as_pointer();
        Append_1(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _new_node: Ptr<Node> = n4.as_pointer();
        Append_1(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _new_node: Ptr<Node> = n5.as_pointer();
        Append_1(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _new_node: Ptr<Node> = n6.as_pointer();
        Append_1(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _new_node: Ptr<Node> = n7.as_pointer();
        Append_1(_head, _new_node)
    });
    let __rhs = ({ Delete_2((*head.borrow()).clone(), 5) });
    (*head.borrow_mut()) = __rhs;
    let __rhs = ({ Delete_2((*head.borrow()).clone(), 0) });
    (*head.borrow_mut()) = __rhs;
    let __rhs = ({ Delete_2((*head.borrow()).clone(), -2_i32) });
    (*head.borrow_mut()) = __rhs;
    assert!(
        (((((({ Find_0((*head.borrow()).clone(), 0,) }).with(|__s| __s.val) == 4)
            && (({ Find_0((*head.borrow()).clone(), 1,) }).with(|__s| __s.val) == 3))
            && (({ Find_0((*head.borrow()).clone(), 2,) }).with(|__s| __s.val) == 2))
            && (({ Find_0((*head.borrow()).clone(), 3,) }).with(|__s| __s.val) == 1))
            && (({ Find_0((*head.borrow()).clone(), 4,) }).with(|__s| __s.val) == -1_i32))
            && (({ Find_0((*head.borrow()).clone(), 5,) }).is_null())
    );
    return 0;
}
pub trait NodeImpl {
    fn SetNext(&self, next: Ptr<Node>);
}
impl NodeImpl for Ptr<Node> {
    fn SetNext(&self, next: Ptr<Node>) {
        let next: Value<Ptr<Node>> = Rc::new(RefCell::new(next));
        field!((*self), next).write((*next.borrow()).clone());
    }
}
pub fn __cpp2rust_init_globals() {}
