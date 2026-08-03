extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[repr(C)]
#[derive(Default)]
pub struct SafePointer {
    pub ptr: Option<Value<i32>>,
}
pub trait SafePointerMethods {
    fn inc(&self);
}
impl SafePointerMethods for Ptr<SafePointer> {
    fn inc(&self) {
        (*self
            .with(|__v| (*__v).ptr.clone())
            .as_ref()
            .unwrap()
            .borrow_mut())
        .prefix_inc();
    }
}
impl ByteRepr for SafePointer {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.ptr.to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            ptr: <Option<Value<i32>>>::from_bytes(&buf[0..8]),
        }
    }
}
#[repr(C)]
#[derive(Default)]
pub struct Pair {
    pub x: i32,
    pub y: i32,
}
pub trait PairMethods {
    fn inc(&self, k: i32);
}
impl Clone for Pair {
    fn clone(&self) -> Self {
        let mut this = Self {
            x: self.x,
            y: self.y,
        };
        this
    }
}
impl ByteRepr for Pair {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x.to_bytes(&mut buf[0..4]);
        self.y.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: <i32>::from_bytes(&buf[0..4]),
            y: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
impl PairMethods for Ptr<Pair> {
    fn inc(&self, k: i32) {
        let k: Value<i32> = Rc::new(RefCell::new(k));
        self.with_mut(|__v| __v.x += (*k.borrow()));
        self.with_mut(|__v| __v.y += (*k.borrow()));
    }
}
pub fn DoStuffWithSafePointer_0(safe_ptr: Ptr<Option<Value<SafePointer>>>) {
    let x1: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(0)))));
    let x2: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(0)))));
    (*(*x2.borrow_mut()).as_ref().unwrap().borrow_mut()) = 1;
    (*x1.borrow_mut()) = (*x2.borrow_mut()).take();
    let raw_ptr1: Value<Ptr<i32>> = Rc::new(RefCell::new(((*x1.borrow()).as_pointer())));
    (*raw_ptr1.borrow()).with_mut(|__v| __v.prefix_inc());
    (*(safe_ptr.read()).as_ref().unwrap().borrow_mut()).ptr = (*x1.borrow_mut()).take();
    ({ ((safe_ptr.read()).as_pointer()).inc() });
    ({ ((safe_ptr.read()).as_pointer()).inc() });
    let x3: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(10)))));
    let x4: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(20)))));
    let __rhs = ((*(*x3.borrow()).as_ref().unwrap().borrow())
        + (*(*x4.borrow()).as_ref().unwrap().borrow()));
    (*(*x3.borrow_mut()).as_ref().unwrap().borrow_mut()) = __rhs;
    (*x4.borrow_mut()) = (*x3.borrow_mut()).take();
    let raw_ptr2: Value<Ptr<i32>> = Rc::new(RefCell::new(((*x4.borrow()).as_pointer())));
    {
        let _ptr = (*raw_ptr2.borrow()).clone();
        _ptr.write((_ptr.read()) + 1)
    };
    let pair: Value<Option<Value<Pair>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Pair {
            x: ((*raw_ptr2.borrow()).read()),
            y: 5,
        })))));
    ({ ((*pair.borrow()).as_pointer()).inc(10) });
    let __rhs = {
        let _lhs = {
            let _lhs = (*(*(safe_ptr.read()).as_ref().unwrap().borrow())
                .ptr
                .as_ref()
                .unwrap()
                .borrow());
            _lhs + (*(*pair.borrow()).as_ref().unwrap().borrow()).x
        };
        _lhs + (*(*pair.borrow()).as_ref().unwrap().borrow()).y
    };
    (*(*(safe_ptr.read()).as_ref().unwrap().borrow_mut())
        .ptr
        .as_ref()
        .unwrap()
        .borrow_mut()) = __rhs;
}
pub fn Consume_1(safe_ptr: Option<Value<SafePointer>>) -> i32 {
    let safe_ptr: Value<Option<Value<SafePointer>>> = Rc::new(RefCell::new(safe_ptr));
    let x: Value<Option<Value<SafePointer>>> =
        Rc::new(RefCell::new((*safe_ptr.borrow_mut()).take()));
    let p: Value<Option<Value<Pair>>> =
        Rc::new(RefCell::new(Ptr::alloc(<Pair>::default()).to_owned_opt()));
    return ((*(*(*x.borrow()).as_ref().unwrap().borrow())
        .ptr
        .as_ref()
        .unwrap()
        .borrow())
        + (*(*p.borrow()).as_ref().unwrap().borrow()).x);
}
pub fn RndStuff_2() {
    let x1: Value<Option<Value<Box<[i32]>>>> = Rc::new(RefCell::new(None));
    let x2: Value<Option<Value<Box<[i32]>>>> = Rc::new(RefCell::new(
        Ptr::alloc_array(
            (0..100_usize)
                .map(|_| <i32>::default())
                .collect::<Box<[i32]>>(),
        )
        .to_owned_opt(),
    ));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 100) {
        (*(*x2.borrow()).as_ref().unwrap().borrow_mut())[((*i.borrow()) as usize) as usize] = 1;
        (*i.borrow_mut()).prefix_inc();
    }
    (*x2.borrow_mut()) = Ptr::alloc_array(
        (0..200_usize)
            .map(|_| <i32>::default())
            .collect::<Box<[i32]>>(),
    )
    .to_owned_opt();
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 200) {
        (*(*x2.borrow()).as_ref().unwrap().borrow_mut())[((*i.borrow()) as usize) as usize] = 2;
        (*i.borrow_mut()).prefix_inc();
    }
    let p2: Value<Ptr<i32>> = Rc::new(RefCell::new((*x2.borrow()).as_pointer()));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 200) {
        assert!((((*p2.borrow()).offset(((*i.borrow()) as isize)).read()) == 2));
        (*i.borrow_mut()).prefix_inc();
    }
    let x3: Value<Option<Value<Box<[Pair]>>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
        (0..10_usize)
            .map(|_| <Pair>::default())
            .collect::<Box<[_]>>(),
    )))));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 10) {
        (*(*x3.borrow()).as_ref().unwrap().borrow_mut())[((*i.borrow()) as usize) as usize] =
            Pair { x: 1, y: 2 };
        (*i.borrow_mut()).prefix_inc();
    }
    let p3_0: Value<Ptr<Pair>> = Rc::new(RefCell::new((*x3.borrow()).as_pointer()));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 10) {
        assert!(
            ((*p3_0.borrow())
                .offset(((*i.borrow()) as isize))
                .with(|__v| (*__v).x)
                == 1)
        );
        assert!(
            ((*p3_0.borrow())
                .offset(((*i.borrow()) as isize))
                .with(|__v| (*__v).y)
                == 2)
        );
        ({
            ((*x3.borrow()).as_ref().unwrap().as_pointer() as Ptr<Pair>)
                .offset(((*i.borrow()) as usize))
                .inc(10)
        });
        assert!(
            ((*p3_0.borrow())
                .offset(((*i.borrow()) as isize))
                .with(|__v| (*__v).x)
                == 11)
        );
        assert!(
            ((*p3_0.borrow())
                .offset(((*i.borrow()) as isize))
                .with(|__v| (*__v).y)
                == 12)
        );
        (*i.borrow_mut()).prefix_inc();
    }
    (*x3.borrow_mut()) = Ptr::alloc_array(
        (0..50_usize)
            .map(|_| <Pair>::default())
            .collect::<Box<[Pair]>>(),
    )
    .to_owned_opt();
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 50) {
        (*(*x3.borrow()).as_ref().unwrap().borrow_mut())[((*i.borrow()) as usize) as usize] =
            Pair {
                x: -1_i32,
                y: -2_i32,
            };
        (*i.borrow_mut()).prefix_inc();
    }
    let p3_1: Value<Ptr<Pair>> = Rc::new(RefCell::new((*x3.borrow()).as_pointer()));
    assert!({
        let _lhs = (*p3_0.borrow()).clone();
        _lhs != (*p3_1.borrow()).clone()
    });
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 50) {
        assert!(
            ((*p3_1.borrow())
                .offset(((*i.borrow()) as isize))
                .with(|__v| (*__v).x)
                == -1_i32)
        );
        assert!(
            ((*p3_1.borrow())
                .offset(((*i.borrow()) as isize))
                .with(|__v| (*__v).y)
                == -2_i32)
        );
        ({
            ((*x3.borrow()).as_ref().unwrap().as_pointer() as Ptr<Pair>)
                .offset(((*i.borrow()) as usize))
                .inc(-10_i32)
        });
        assert!(
            ((*p3_1.borrow())
                .offset(((*i.borrow()) as isize))
                .with(|__v| (*__v).x)
                == -11_i32)
        );
        assert!(
            ((*p3_1.borrow())
                .offset(((*i.borrow()) as isize))
                .with(|__v| (*__v).y)
                == -12_i32)
        );
        (*i.borrow_mut()).prefix_inc();
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(0)))));
    let safe_ptr: Value<Option<Value<SafePointer>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(SafePointer {
            ptr: (*x.borrow_mut()).take(),
        })))));
    ({ DoStuffWithSafePointer_0(safe_ptr.as_pointer()) });
    return ({ Consume_1((*safe_ptr.borrow_mut()).take()) });
}
