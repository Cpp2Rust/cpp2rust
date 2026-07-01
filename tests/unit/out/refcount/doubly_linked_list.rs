extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Node {
    pub val: Value<i32>,
    pub next: Value<Ptr<Node>>,
    pub prev: Value<Ptr<Node>>,
}
impl Node {
    pub fn SetNext(&self, n: Ptr<Node>) {
        let n: Value<Ptr<Node>> = Rc::new(RefCell::new(n));
        (*self.next.borrow_mut()) = (*n.borrow()).clone();
    }
    pub fn SetPrev(&self, p: Ptr<Node>) {
        let p: Value<Ptr<Node>> = Rc::new(RefCell::new(p));
        (*self.prev.borrow_mut()) = (*p.borrow()).clone();
    }
}
impl Clone for Node {
    fn clone(&self) -> Self {
        let mut this = Self {
            val: Rc::new(RefCell::new((*self.val.borrow()))),
            next: Rc::new(RefCell::new((*self.next.borrow()).clone())),
            prev: Rc::new(RefCell::new((*self.prev.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for Node {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.val.borrow()).to_bytes(&mut buf[0..4]);
        (*self.next.borrow()).to_bytes(&mut buf[8..16]);
        (*self.prev.borrow()).to_bytes(&mut buf[16..24]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            val: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            next: Rc::new(RefCell::new(<Ptr<Node>>::from_bytes(&buf[8..16]))),
            prev: Rc::new(RefCell::new(<Ptr<Node>>::from_bytes(&buf[16..24]))),
        }
    }
}
pub fn Find_0(head: Ptr<Node>, idx: i32) -> Ptr<Node> {
    let head: Value<Ptr<Node>> = Rc::new(RefCell::new(head));
    let idx: Value<i32> = Rc::new(RefCell::new(idx));
    let curr: Value<Ptr<Node>> = Rc::new(RefCell::new((*head.borrow()).clone()));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*idx.borrow())) {
        let __rhs = (*(*(*curr.borrow()).upgrade().deref()).next.borrow()).clone();
        (*curr.borrow_mut()) = __rhs;
        (*i.borrow_mut()).postfix_inc();
    }
    return (*curr.borrow()).clone();
}
pub fn FindBack_1(tail: Ptr<Node>, idx: i32) -> Ptr<Node> {
    let tail: Value<Ptr<Node>> = Rc::new(RefCell::new(tail));
    let idx: Value<i32> = Rc::new(RefCell::new(idx));
    let curr: Value<Ptr<Node>> = Rc::new(RefCell::new((*tail.borrow()).clone()));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*idx.borrow())) {
        let __rhs = (*(*(*curr.borrow()).upgrade().deref()).prev.borrow()).clone();
        (*curr.borrow_mut()) = __rhs;
        (*i.borrow_mut()).postfix_inc();
    }
    return (*curr.borrow()).clone();
}
pub fn Append_2(head: Ptr<Node>, new_node: Ptr<Node>) {
    let curr: Value<Ptr<Node>> = Rc::new(RefCell::new((head).clone()));
    'loop_: while !((*(*(*curr.borrow()).upgrade().deref()).next.borrow()).is_null()) {
        let __rhs = (*(*(*curr.borrow()).upgrade().deref()).next.borrow()).clone();
        (*curr.borrow_mut()) = __rhs;
    }
    ({ (*(*curr.borrow()).upgrade().deref()).SetNext((new_node).clone()) });
    ({
        let _p: Ptr<Node> = (*curr.borrow()).clone();
        (*new_node.upgrade().deref()).SetPrev(_p)
    });
}
pub fn Delete_3(head: Ptr<Node>, val: i32) -> Ptr<Node> {
    let head: Value<Ptr<Node>> = Rc::new(RefCell::new(head));
    let val: Value<i32> = Rc::new(RefCell::new(val));
    let curr: Value<Ptr<Node>> = Rc::new(RefCell::new((*head.borrow()).clone()));
    'loop_: while !((*curr.borrow()).is_null()) {
        if {
            let _lhs = (*(*(*curr.borrow()).upgrade().deref()).val.borrow());
            _lhs == (*val.borrow())
        } {
            let prev: Value<Ptr<Node>> = Rc::new(RefCell::new(
                (*(*(*curr.borrow()).upgrade().deref()).prev.borrow()).clone(),
            ));
            let next: Value<Ptr<Node>> = Rc::new(RefCell::new(
                (*(*(*curr.borrow()).upgrade().deref()).next.borrow()).clone(),
            ));
            if !((*prev.borrow()).is_null()) {
                (*(*(*prev.borrow()).upgrade().deref()).next.borrow_mut()) =
                    (*next.borrow()).clone();
            }
            if !((*next.borrow()).is_null()) {
                (*(*(*next.borrow()).upgrade().deref()).prev.borrow_mut()) =
                    (*prev.borrow()).clone();
            }
            if !((*prev.borrow()).is_null()) {
                return (*head.borrow()).clone();
            } else {
                return (*next.borrow()).clone();
            }
        }
        let __rhs = (*(*(*curr.borrow()).upgrade().deref()).next.borrow()).clone();
        (*curr.borrow_mut()) = __rhs;
    }
    return (*head.borrow()).clone();
}
pub fn Tail_4(head: Ptr<Node>) -> Ptr<Node> {
    let head: Value<Ptr<Node>> = Rc::new(RefCell::new(head));
    let curr: Value<Ptr<Node>> = Rc::new(RefCell::new((*head.borrow()).clone()));
    'loop_: while !((*(*(*curr.borrow()).upgrade().deref()).next.borrow()).is_null()) {
        let __rhs = (*(*(*curr.borrow()).upgrade().deref()).next.borrow()).clone();
        (*curr.borrow_mut()) = __rhs;
    }
    return (*curr.borrow()).clone();
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let n0: Value<Node> = Rc::new(RefCell::new(Node {
        val: Rc::new(RefCell::new(5)),
        next: Rc::new(RefCell::new(Ptr::<Node>::null())),
        prev: Rc::new(RefCell::new(Ptr::<Node>::null())),
    }));
    let head: Value<Ptr<Node>> = Rc::new(RefCell::new((n0.as_pointer())));
    let n1: Value<Node> = Rc::new(RefCell::new(Node {
        val: Rc::new(RefCell::new(4)),
        next: Rc::new(RefCell::new(Ptr::<Node>::null())),
        prev: Rc::new(RefCell::new(Ptr::<Node>::null())),
    }));
    let n2: Value<Node> = Rc::new(RefCell::new(Node {
        val: Rc::new(RefCell::new(3)),
        next: Rc::new(RefCell::new(Ptr::<Node>::null())),
        prev: Rc::new(RefCell::new(Ptr::<Node>::null())),
    }));
    let n3: Value<Node> = Rc::new(RefCell::new(Node {
        val: Rc::new(RefCell::new(2)),
        next: Rc::new(RefCell::new(Ptr::<Node>::null())),
        prev: Rc::new(RefCell::new(Ptr::<Node>::null())),
    }));
    let n4: Value<Node> = Rc::new(RefCell::new(Node {
        val: Rc::new(RefCell::new(1)),
        next: Rc::new(RefCell::new(Ptr::<Node>::null())),
        prev: Rc::new(RefCell::new(Ptr::<Node>::null())),
    }));
    let n5: Value<Node> = Rc::new(RefCell::new(Node {
        val: Rc::new(RefCell::new(0)),
        next: Rc::new(RefCell::new(Ptr::<Node>::null())),
        prev: Rc::new(RefCell::new(Ptr::<Node>::null())),
    }));
    let n6: Value<Node> = Rc::new(RefCell::new(Node {
        val: Rc::new(RefCell::new(-1_i32)),
        next: Rc::new(RefCell::new(Ptr::<Node>::null())),
        prev: Rc::new(RefCell::new(Ptr::<Node>::null())),
    }));
    let n7: Value<Node> = Rc::new(RefCell::new(Node {
        val: Rc::new(RefCell::new(-2_i32)),
        next: Rc::new(RefCell::new(Ptr::<Node>::null())),
        prev: Rc::new(RefCell::new(Ptr::<Node>::null())),
    }));
    ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _new_node: Ptr<Node> = n1.as_pointer();
        Append_2(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _new_node: Ptr<Node> = n2.as_pointer();
        Append_2(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _new_node: Ptr<Node> = n3.as_pointer();
        Append_2(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _new_node: Ptr<Node> = n4.as_pointer();
        Append_2(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _new_node: Ptr<Node> = n5.as_pointer();
        Append_2(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _new_node: Ptr<Node> = n6.as_pointer();
        Append_2(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _new_node: Ptr<Node> = n7.as_pointer();
        Append_2(_head, _new_node)
    });
    let __rhs = ({ Delete_3((*head.borrow()).clone(), 5) });
    (*head.borrow_mut()) = __rhs;
    let __rhs = ({ Delete_3((*head.borrow()).clone(), 0) });
    (*head.borrow_mut()) = __rhs;
    let __rhs = ({ Delete_3((*head.borrow()).clone(), -2_i32) });
    (*head.borrow_mut()) = __rhs;
    let tail: Value<Ptr<Node>> = Rc::new(RefCell::new(({ Tail_4((*head.borrow()).clone()) })));
    assert!(
        ((*(*({ Find_0((*head.borrow()).clone(), 0,) }).upgrade().deref())
            .val
            .borrow())
            == 4)
    );
    assert!(
        ((*(*({ Find_0((*head.borrow()).clone(), 1,) }).upgrade().deref())
            .val
            .borrow())
            == 3)
    );
    assert!(
        ((*(*({ Find_0((*head.borrow()).clone(), 2,) }).upgrade().deref())
            .val
            .borrow())
            == 2)
    );
    assert!(
        ((*(*({ Find_0((*head.borrow()).clone(), 3,) }).upgrade().deref())
            .val
            .borrow())
            == 1)
    );
    assert!(
        ((*(*({ Find_0((*head.borrow()).clone(), 4,) }).upgrade().deref())
            .val
            .borrow())
            == -1_i32)
    );
    assert!(({ Find_0((*head.borrow()).clone(), 5,) }).is_null());
    assert!(
        ((*(*({ FindBack_1((*tail.borrow()).clone(), 0,) })
            .upgrade()
            .deref())
        .val
        .borrow())
            == -1_i32)
    );
    assert!(
        ((*(*({ FindBack_1((*tail.borrow()).clone(), 1,) })
            .upgrade()
            .deref())
        .val
        .borrow())
            == 1)
    );
    assert!(
        ((*(*({ FindBack_1((*tail.borrow()).clone(), 2,) })
            .upgrade()
            .deref())
        .val
        .borrow())
            == 2)
    );
    assert!(
        ((*(*({ FindBack_1((*tail.borrow()).clone(), 3,) })
            .upgrade()
            .deref())
        .val
        .borrow())
            == 3)
    );
    assert!(
        ((*(*({ FindBack_1((*tail.borrow()).clone(), 4,) })
            .upgrade()
            .deref())
        .val
        .borrow())
            == 4)
    );
    assert!(
        (*(*({ FindBack_1((*tail.borrow()).clone(), 4,) })
            .upgrade()
            .deref())
        .prev
        .borrow())
        .is_null()
    );
    assert!(
        ((*(*(*(*({ Find_0((*head.borrow()).clone(), 0,) }).upgrade().deref())
            .next
            .borrow())
        .upgrade()
        .deref())
        .val
        .borrow())
            == 3)
    );
    assert!(
        ((*(*(*(*(*(*({ Find_0((*head.borrow()).clone(), 1,) }).upgrade().deref())
            .next
            .borrow())
        .upgrade()
        .deref())
        .next
        .borrow())
        .upgrade()
        .deref())
        .val
        .borrow())
            == 1)
    );
    assert!(
        ((*(*(*(*({ Find_0((*head.borrow()).clone(), 2,) }).upgrade().deref())
            .prev
            .borrow())
        .upgrade()
        .deref())
        .val
        .borrow())
            == 3)
    );
    assert!(
        (*(*({ Find_0((*head.borrow()).clone(), 4,) }).upgrade().deref())
            .next
            .borrow())
        .is_null()
    );
    assert!(
        ((*(*(*(*(*(*({ FindBack_1((*tail.borrow()).clone(), 1,) })
            .upgrade()
            .deref())
        .prev
        .borrow())
        .upgrade()
        .deref())
        .prev
        .borrow())
        .upgrade()
        .deref())
        .val
        .borrow())
            == 3)
    );
    (*(*(*(*({ Find_0((*head.borrow()).clone(), 0) }).upgrade().deref())
        .next
        .borrow())
    .upgrade()
    .deref())
    .val
    .borrow_mut()) = 30;
    assert!(
        ((*(*({ Find_0((*head.borrow()).clone(), 1,) }).upgrade().deref())
            .val
            .borrow())
            == 30)
    );
    let __rhs = ((*(*({ Find_0((*head.borrow()).clone(), 0) }).upgrade().deref())
        .val
        .borrow())
        + (*(*({ Find_0((*head.borrow()).clone(), 3) }).upgrade().deref())
            .val
            .borrow()));
    (*(*(*(*({ Find_0((*head.borrow()).clone(), 1) }).upgrade().deref())
        .next
        .borrow())
    .upgrade()
    .deref())
    .val
    .borrow_mut()) = __rhs;
    assert!(
        ((*(*({ Find_0((*head.borrow()).clone(), 2,) }).upgrade().deref())
            .val
            .borrow())
            == (4 + 1))
    );
    let sum: Value<i32> = Rc::new(RefCell::new(
        (((((*(*({ Find_0((*head.borrow()).clone(), 0) }).upgrade().deref())
            .val
            .borrow())
            + (*(*({ Find_0((*head.borrow()).clone(), 1) }).upgrade().deref())
                .val
                .borrow()))
            + (*(*({ Find_0((*head.borrow()).clone(), 2) }).upgrade().deref())
                .val
                .borrow()))
            + (*(*({ Find_0((*head.borrow()).clone(), 3) }).upgrade().deref())
                .val
                .borrow()))
            + (*(*({ Find_0((*head.borrow()).clone(), 4) }).upgrade().deref())
                .val
                .borrow())),
    ));
    assert!(((*sum.borrow()) == ((((4 + 30) + 5) + 1) + -1_i32)));
    assert!(
        ({
            let _lhs = (*(*({ Find_0((*head.borrow()).clone(), 0) }).upgrade().deref())
                .val
                .borrow());
            _lhs + (*(*({ FindBack_1((*tail.borrow()).clone(), 0) })
                .upgrade()
                .deref())
            .val
            .borrow())
        } == (4 + -1_i32))
    );
    assert!({
        let _lhs = (*(*(*(*({ Find_0((*head.borrow()).clone(), 2) }).upgrade().deref())
            .next
            .borrow())
        .upgrade()
        .deref())
        .val
        .borrow());
        _lhs == (*(*({ FindBack_1((*tail.borrow()).clone(), 1) })
            .upgrade()
            .deref())
        .val
        .borrow())
    });
    assert!({
        let _lhs = (*(*({ Find_0((*head.borrow()).clone(), 0) }).upgrade().deref())
            .prev
            .borrow())
        .clone();
        _lhs == (*(*({ FindBack_1((*tail.borrow()).clone(), 4) })
            .upgrade()
            .deref())
        .prev
        .borrow())
        .clone()
    });
    return 0;
}
