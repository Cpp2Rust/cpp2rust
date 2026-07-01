extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Node {
    pub val: i32,
    pub next: *mut Node,
    pub prev: *mut Node,
}
impl Node {
    pub unsafe fn SetNext(&mut self, mut n: *mut Node) {
        self.next = n;
    }
    pub unsafe fn SetPrev(&mut self, mut p: *mut Node) {
        self.prev = p;
    }
}
pub unsafe fn Find_0(mut head: *mut Node, mut idx: i32) -> *mut Node {
    let mut curr: *mut Node = head;
    let mut i: i32 = 0;
    'loop_: while ((i) < (idx)) {
        curr = (*curr).next;
        i.postfix_inc();
    }
    return curr;
}
pub unsafe fn FindBack_1(mut tail: *mut Node, mut idx: i32) -> *mut Node {
    let mut curr: *mut Node = tail;
    let mut i: i32 = 0;
    'loop_: while ((i) < (idx)) {
        curr = (*curr).prev;
        i.postfix_inc();
    }
    return curr;
}
pub unsafe fn Append_2(head: *mut Node, new_node: *mut Node) {
    let mut curr: *mut Node = (head);
    'loop_: while !(((*curr).next).is_null()) {
        curr = (*curr).next;
    }
    (unsafe { (*curr).SetNext((new_node)) });
    (unsafe {
        let _p: *mut Node = curr;
        (*new_node).SetPrev(_p)
    });
}
pub unsafe fn Delete_3(mut head: *mut Node, mut val: i32) -> *mut Node {
    let mut curr: *mut Node = head;
    'loop_: while !((curr).is_null()) {
        if (((*curr).val) == (val)) {
            let mut prev: *mut Node = (*curr).prev;
            let mut next: *mut Node = (*curr).next;
            if !((prev).is_null()) {
                (*prev).next = next;
            }
            if !((next).is_null()) {
                (*next).prev = prev;
            }
            if !((prev).is_null()) {
                return head;
            } else {
                return next;
            }
        }
        curr = (*curr).next;
    }
    return head;
}
pub unsafe fn Tail_4(mut head: *mut Node) -> *mut Node {
    let mut curr: *mut Node = head;
    'loop_: while !(((*curr).next).is_null()) {
        curr = (*curr).next;
    }
    return curr;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut n0: Node = Node {
        val: 5,
        next: std::ptr::null_mut(),
        prev: std::ptr::null_mut(),
    };
    let mut head: *mut Node = (&mut n0 as *mut Node);
    let mut n1: Node = Node {
        val: 4,
        next: std::ptr::null_mut(),
        prev: std::ptr::null_mut(),
    };
    let mut n2: Node = Node {
        val: 3,
        next: std::ptr::null_mut(),
        prev: std::ptr::null_mut(),
    };
    let mut n3: Node = Node {
        val: 2,
        next: std::ptr::null_mut(),
        prev: std::ptr::null_mut(),
    };
    let mut n4: Node = Node {
        val: 1,
        next: std::ptr::null_mut(),
        prev: std::ptr::null_mut(),
    };
    let mut n5: Node = Node {
        val: 0,
        next: std::ptr::null_mut(),
        prev: std::ptr::null_mut(),
    };
    let mut n6: Node = Node {
        val: -1_i32,
        next: std::ptr::null_mut(),
        prev: std::ptr::null_mut(),
    };
    let mut n7: Node = Node {
        val: -2_i32,
        next: std::ptr::null_mut(),
        prev: std::ptr::null_mut(),
    };
    (unsafe {
        let _head: *mut Node = &mut (*head) as *mut Node;
        let _new_node: *mut Node = &mut n1 as *mut Node;
        Append_2(_head, _new_node)
    });
    (unsafe {
        let _head: *mut Node = &mut (*head) as *mut Node;
        let _new_node: *mut Node = &mut n2 as *mut Node;
        Append_2(_head, _new_node)
    });
    (unsafe {
        let _head: *mut Node = &mut (*head) as *mut Node;
        let _new_node: *mut Node = &mut n3 as *mut Node;
        Append_2(_head, _new_node)
    });
    (unsafe {
        let _head: *mut Node = &mut (*head) as *mut Node;
        let _new_node: *mut Node = &mut n4 as *mut Node;
        Append_2(_head, _new_node)
    });
    (unsafe {
        let _head: *mut Node = &mut (*head) as *mut Node;
        let _new_node: *mut Node = &mut n5 as *mut Node;
        Append_2(_head, _new_node)
    });
    (unsafe {
        let _head: *mut Node = &mut (*head) as *mut Node;
        let _new_node: *mut Node = &mut n6 as *mut Node;
        Append_2(_head, _new_node)
    });
    (unsafe {
        let _head: *mut Node = &mut (*head) as *mut Node;
        let _new_node: *mut Node = &mut n7 as *mut Node;
        Append_2(_head, _new_node)
    });
    head = (unsafe { Delete_3(head, 5) });
    head = (unsafe { Delete_3(head, 0) });
    head = (unsafe { Delete_3(head, -2_i32) });
    let mut tail: *mut Node = (unsafe { Tail_4(head) });
    assert!((((*(unsafe { Find_0(head, 0,) })).val) == (4)));
    assert!((((*(unsafe { Find_0(head, 1,) })).val) == (3)));
    assert!((((*(unsafe { Find_0(head, 2,) })).val) == (2)));
    assert!((((*(unsafe { Find_0(head, 3,) })).val) == (1)));
    assert!((((*(unsafe { Find_0(head, 4,) })).val) == (-1_i32)));
    assert!((unsafe { Find_0(head, 5,) }).is_null());
    assert!((((*(unsafe { FindBack_1(tail, 0,) })).val) == (-1_i32)));
    assert!((((*(unsafe { FindBack_1(tail, 1,) })).val) == (1)));
    assert!((((*(unsafe { FindBack_1(tail, 2,) })).val) == (2)));
    assert!((((*(unsafe { FindBack_1(tail, 3,) })).val) == (3)));
    assert!((((*(unsafe { FindBack_1(tail, 4,) })).val) == (4)));
    assert!(((*(unsafe { FindBack_1(tail, 4,) })).prev).is_null());
    assert!((((*(*(unsafe { Find_0(head, 0,) })).next).val) == (3)));
    assert!((((*(*(*(unsafe { Find_0(head, 1,) })).next).next).val) == (1)));
    assert!((((*(*(unsafe { Find_0(head, 2,) })).prev).val) == (3)));
    assert!(((*(unsafe { Find_0(head, 4,) })).next).is_null());
    assert!((((*(*(*(unsafe { FindBack_1(tail, 1,) })).prev).prev).val) == (3)));
    (*(*(unsafe { Find_0(head, 0) })).next).val = 30;
    assert!((((*(unsafe { Find_0(head, 1,) })).val) == (30)));
    (*(*(unsafe { Find_0(head, 1) })).next).val =
        (((*(unsafe { Find_0(head, 0) })).val) + ((*(unsafe { Find_0(head, 3) })).val));
    assert!((((*(unsafe { Find_0(head, 2,) })).val) == ((4) + (1))));
    let mut sum: i32 = ((((((*(unsafe { Find_0(head, 0) })).val)
        + ((*(unsafe { Find_0(head, 1) })).val))
        + ((*(unsafe { Find_0(head, 2) })).val))
        + ((*(unsafe { Find_0(head, 3) })).val))
        + ((*(unsafe { Find_0(head, 4) })).val));
    assert!(((sum) == (((((4) + (30)) + (5)) + (1)) + (-1_i32))));
    assert!(
        ((((*(unsafe { Find_0(head, 0,) })).val) + ((*(unsafe { FindBack_1(tail, 0,) })).val))
            == ((4) + (-1_i32)))
    );
    assert!(
        (((*(*(unsafe { Find_0(head, 2,) })).next).val)
            == ((*(unsafe { FindBack_1(tail, 1,) })).val))
    );
    assert!(
        (((*(unsafe { Find_0(head, 0,) })).prev) == ((*(unsafe { FindBack_1(tail, 4,) })).prev))
    );
    return 0;
}
