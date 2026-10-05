extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static g_cursor_0: Value<Ptr<Ptr<i32>>> = Rc::new(RefCell::new(Ptr::<Ptr<i32>>::null()));
);
pub fn advance_1() -> i32 {
    (*g_cursor_0.with(Value::clone).borrow()).with_mut(|__v| __v.prefix_inc());
    return 10;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct S {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<i32>,
}
pub fn by_ref_2(r: Ptr<Ptr<i32>>) {
    let __rhs = ({ advance_1() });
    {
        let _ptr = elem!((r.read()), 1);
        _ptr.write(_ptr.read() + __rhs)
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6, 7, 8])));
    let q: Value<Ptr<i32>> = Rc::new(RefCell::new((a.as_pointer() as Ptr<i32>)));
    g_cursor_0.with(|rc| *rc.borrow_mut() = (q.as_pointer()));
    let __rhs = ({ advance_1() });
    elem!((*q.borrow()), 1).write(__rhs);
    assert!(
        ({ (*q.borrow()).clone() } == { (a.as_pointer() as Ptr::<i32>).offset((1) as isize) })
            && ((*a.borrow())[(2) as usize] == 10)
    );
    let __rhs = ({ advance_1() });
    {
        let _ptr = elem!((*q.borrow()), 1);
        _ptr.write(_ptr.read() + __rhs)
    };
    assert!(
        ({ (*q.borrow()).clone() } == { (a.as_pointer() as Ptr::<i32>).offset((2) as isize) })
            && ((*a.borrow())[(3) as usize] == 14)
    );
    let mut pq: Ptr<Ptr<i32>> = (q.as_pointer());
    let __rhs = ({ advance_1() });
    {
        let _ptr = elem!((pq.read()), 1);
        _ptr.write(_ptr.read() + __rhs)
    };
    assert!(
        ({ (*q.borrow()).clone() } == { (a.as_pointer() as Ptr::<i32>).offset((3) as isize) })
            && ((*a.borrow())[(4) as usize] == 15)
    );
    let __rhs = ({ advance_1() });
    elem!((pq.read()), 1).write(__rhs);
    assert!(
        ({ (*q.borrow()).clone() } == { (a.as_pointer() as Ptr::<i32>).offset((4) as isize) })
            && ((*a.borrow())[(5) as usize] == 10)
    );
    ({ by_ref_2(q.as_pointer()) });
    assert!(
        ({ (*q.borrow()).clone() } == { (a.as_pointer() as Ptr::<i32>).offset((5) as isize) })
            && ((*a.borrow())[(6) as usize] == 17)
    );
    let s: Value<S> = Rc::new(RefCell::new(S {
        ptr: (a.as_pointer() as Ptr<i32>),
    }));
    g_cursor_0.with(|rc| *rc.borrow_mut() = (field_ptr!(s.as_pointer(), ptr)));
    let __rhs = ({ advance_1() });
    {
        let _ptr = elem!({ (*s.borrow()).ptr.clone() }, 1);
        _ptr.write(_ptr.read() + __rhs)
    };
    assert!(
        ({ { (*s.borrow()).ptr.clone() } } == {
            (a.as_pointer() as Ptr<i32>).offset((1) as isize)
        }) && ((*a.borrow())[(2) as usize] == 20)
    );
    let mut sp: Ptr<S> = (s.as_pointer());
    let __rhs = ({ advance_1() });
    {
        let _ptr = elem!(sp.with(|__s| __s.ptr.clone()), 1);
        _ptr.write(_ptr.read() + __rhs)
    };
    assert!(
        ({ { (*s.borrow()).ptr.clone() } } == {
            (a.as_pointer() as Ptr<i32>).offset((2) as isize)
        }) && ((*a.borrow())[(3) as usize] == 24)
    );
    let mut b: Ptr<u8> = (a.as_pointer() as Ptr<i32>).reinterpret_cast::<u8>();
    elem!(b, 0).write(7_u8);
    elem!(b, 4).write({ (((elem!(b, 4).read()) as i32) + 1) as u8 });
    assert!(((*a.borrow())[(0) as usize] == 7) && ((*a.borrow())[(1) as usize] == 3));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = g_cursor_0.with(|_| ());
}
