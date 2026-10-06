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
pub struct node {
    #[offset(0)]
    pub value: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub next: Ptr<node>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct pair_t {
    #[offset(0)]
    pub a: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub n: Ptr<node>,
}
thread_local!(
    pub static global_node_0: Value<Ptr<node>> = Rc::new(RefCell::new(Ptr::<node>::null()));
);
pub fn id_1(mut n: Ptr<node>) -> Ptr<node> {
    return n;
}
pub fn pick_2(a: Ptr<node>, b: Ptr<node>) -> Ptr<node> {
    let a: Value<Ptr<node>> = Rc::new(RefCell::new(a));
    let b: Value<Ptr<node>> = Rc::new(RefCell::new(b));
    return if !(*a.borrow()).is_null() {
        (*a.borrow()).clone()
    } else {
        (*b.borrow()).clone()
    };
}
pub fn local_ptr_3(mut n: Ptr<node>) -> Ptr<node> {
    let mut p: Ptr<node> = n.with(|__s| __s.next.clone());
    return p;
}
pub fn call_once_4(mut n: Ptr<node>, mut m: Ptr<node>) -> Ptr<node> {
    return ({ pick_2(n, m) });
}
pub fn call_twice_5(mut n: Ptr<node>) -> Ptr<node> {
    return ({
        let _a: Ptr<node> = (n).clone();
        let _b: Ptr<node> = (n).clone();
        pick_2(_a, _b)
    });
}
pub fn next_of_6(mut n: Ptr<node>) -> Ptr<node> {
    return n.with(|__s| __s.next.clone());
}
pub fn ret_global_7() -> Ptr<node> {
    return (*global_node_0.with(Value::clone).borrow()).clone();
}
pub fn address_taken_8(n: Ptr<node>) -> Ptr<node> {
    let n: Value<Ptr<node>> = Rc::new(RefCell::new(n));
    let mut pp: Ptr<Ptr<node>> = (n.as_pointer());
    return if !(pp.read()).is_null() {
        (*n.borrow()).clone()
    } else {
        Ptr::<node>::null()
    };
}
pub fn ret_struct_9(mut a: i32, mut n: Ptr<node>) -> pair_t {
    let mut p: pair_t = <pair_t>::default();
    p.a = a;
    p.n = (n).clone();
    return p;
}
pub fn ret_struct_param_10(mut p: pair_t) -> pair_t {
    return p;
}
pub fn ret_struct_ref_11(p: Ptr<pair_t>) -> pair_t {
    return (*p.upgrade().deref()).clone();
}
pub fn ret_vec_12(v: Vec<i32>) -> Vec<i32> {
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(v));
    return std::mem::take(&mut (*v.borrow_mut()));
}
pub fn ret_vec_local_13() -> Vec<i32> {
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = 1;
        (*v.borrow_mut()).push(__a1)
    };
    return std::mem::take(&mut (*v.borrow_mut()));
}
pub fn ret_loop_14(mut n: Ptr<node>) -> Ptr<node> {
    'loop_: while !(n.with(|__s| __s.next.clone())).is_null() {
        if (n.with(|__s| __s.value) == 2) {
            return n;
        }
        n = { n.with(|__s| __s.next.clone()) };
    }
    return n;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let c: Value<node> = Rc::new(RefCell::new(node {
        value: 3,
        next: Ptr::<node>::null(),
    }));
    let b: Value<node> = Rc::new(RefCell::new(node {
        value: 2,
        next: (c.as_pointer()),
    }));
    let a: Value<node> = Rc::new(RefCell::new(node {
        value: 1,
        next: (b.as_pointer()),
    }));
    global_node_0.with(|rc| *rc.borrow_mut() = (a.as_pointer()));
    assert!((({ id_1((a.as_pointer()),) }) == (a.as_pointer())));
    assert!((({ local_ptr_3((a.as_pointer()),) }) == (b.as_pointer())));
    assert!((({ call_once_4(Ptr::<node>::null(), (b.as_pointer()),) }) == (b.as_pointer())));
    assert!((({ call_twice_5((c.as_pointer()),) }) == (c.as_pointer())));
    assert!((({ next_of_6((b.as_pointer()),) }) == (c.as_pointer())));
    assert!((({ ret_global_7() }) == (a.as_pointer())));
    assert!((({ address_taken_8((a.as_pointer()),) }) == (a.as_pointer())));
    let mut p: pair_t = ({ ret_struct_9(4, (a.as_pointer())) });
    assert!((p.a == 4) && ({ (p.n).clone() } == { (a.as_pointer()) }));
    let q: Value<pair_t> = Rc::new(RefCell::new(({ ret_struct_param_10((p).clone()) })));
    assert!(
        ({ (*q.borrow()).a } == 4) && ({ { (*q.borrow()).n.clone() } } == { (a.as_pointer()) })
    );
    let mut r: pair_t = ({ ret_struct_ref_11(q.as_pointer()) });
    assert!((r.a == 4) && ({ (r.n).clone() } == { (a.as_pointer()) }));
    assert!((({ ret_vec_12(({ ret_vec_local_13() }),) }).len() == 1_usize));
    assert!((({ ret_loop_14((a.as_pointer()),) }) == (b.as_pointer())));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = global_node_0.with(|_| ());
}
