extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct SafePointer {
    pub ptr: Value<Option<Value<i32>>>,
}
impl SafePointer {
    pub fn inc(&self) {
        (*(*self.ptr.borrow_mut()).as_ref().unwrap().borrow_mut()).prefix_inc();
    }
}
impl ByteRepr for SafePointer {}
#[derive(Default)]
pub struct Pair {
    pub x: Value<i32>,
    pub y: Value<i32>,
}
impl Clone for Pair {
    fn clone(&self) -> Self {
        let mut this = Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
            y: Rc::new(RefCell::new((*self.y.borrow()))),
        };
        this
    }
}
impl ByteRepr for Pair {}
impl Pair {
    pub fn inc(&self, k: i32) {
        let k: Value<i32> = Rc::new(RefCell::new(k));
        (*self.x.borrow_mut()) += (*k.borrow());
        (*self.y.borrow_mut()) += (*k.borrow());
    }
}
pub fn DoStuffWithSafePointer_0(safe_ptr: Ptr<Option<Value<SafePointer>>>) {
    let x1: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(0)))));
    let x2: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(0)))));
    (*(*x2.borrow_mut()).as_ref().unwrap().borrow_mut()) = 1;
    (*x1.borrow_mut()) = (*x2.borrow_mut()).take();
    let raw_ptr1: Value<Ptr<i32>> = Rc::new(RefCell::new(((*x1.borrow()).as_pointer())));
    (*raw_ptr1.borrow()).with_mut(|__v| __v.prefix_inc());
    (*(*(*safe_ptr.upgrade().deref()).as_ref().unwrap().borrow())
        .ptr
        .borrow_mut()) = (*x1.borrow_mut()).take();
    ({ (*(*safe_ptr.upgrade().deref()).as_ref().unwrap().borrow()).inc() });
    ({ (*(*safe_ptr.upgrade().deref()).as_ref().unwrap().borrow()).inc() });
    let x3: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(10)))));
    let x4: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(20)))));
    let __rhs = ((*(*x3.borrow()).as_ref().unwrap().borrow())
        + (*(*x4.borrow()).as_ref().unwrap().borrow()));
    (*(*x3.borrow_mut()).as_ref().unwrap().borrow_mut()) = __rhs;
    (*x4.borrow_mut()) = (*x3.borrow_mut()).take();
    let raw_ptr2: Value<Ptr<i32>> = Rc::new(RefCell::new(((*x4.borrow()).as_pointer())));
    {
        let __ptr = (*raw_ptr2.borrow()).clone();
        let __tmp = __ptr.read() + 1;
        __ptr.write(__tmp)
    };
    let pair: Value<Option<Value<Pair>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Pair {
            x: Rc::new(RefCell::new(((*raw_ptr2.borrow()).read()))),
            y: Rc::new(RefCell::new(5)),
        })))));
    ({
        let _k: i32 = 10;
        (*(*pair.borrow()).as_ref().unwrap().borrow()).inc(_k)
    });
    let __rhs = {
        let _lhs = {
            let _lhs = (*(*(*(*safe_ptr.upgrade().deref()).as_ref().unwrap().borrow())
                .ptr
                .borrow())
            .as_ref()
            .unwrap()
            .borrow());
            _lhs + (*(*(*pair.borrow()).as_ref().unwrap().borrow()).x.borrow())
        };
        _lhs + (*(*(*pair.borrow()).as_ref().unwrap().borrow()).y.borrow())
    };
    (*(*(*(*safe_ptr.upgrade().deref()).as_ref().unwrap().borrow())
        .ptr
        .borrow_mut())
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
    return ((*(*(*(*x.borrow()).as_ref().unwrap().borrow()).ptr.borrow())
        .as_ref()
        .unwrap()
        .borrow())
        + (*(*(*p.borrow()).as_ref().unwrap().borrow()).x.borrow()));
}
pub fn RndStuff_2() {
    let x1: Value<Option<Value<Box<[i32]>>>> = Rc::new(RefCell::new(None));
    let x2: Value<Option<Value<Box<[i32]>>>> = Rc::new(RefCell::new(
        Ptr::alloc_array(
            (0..100_u64)
                .map(|_| <i32>::default())
                .collect::<Box<[i32]>>(),
        )
        .to_owned_opt(),
    ));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 100) {
        (*x2.borrow()).as_ref().unwrap().borrow_mut()[((*i.borrow()) as u64) as usize] = 1;
        (*i.borrow_mut()).prefix_inc();
    }
    (*x2.borrow_mut()) = Ptr::alloc_array(
        (0..200_u64)
            .map(|_| <i32>::default())
            .collect::<Box<[i32]>>(),
    )
    .to_owned_opt();
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 200) {
        (*x2.borrow()).as_ref().unwrap().borrow_mut()[((*i.borrow()) as u64) as usize] = 2;
        (*i.borrow_mut()).prefix_inc();
    }
    let p2: Value<Ptr<i32>> = Rc::new(RefCell::new((*x2.borrow()).as_pointer()));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 200) {
        assert!((((*p2.borrow()).offset((*i.borrow()) as isize).read()) == 2));
        (*i.borrow_mut()).prefix_inc();
    }
    let x3: Value<Option<Value<Box<[Pair]>>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
        (0..10_u64).map(|_| <Pair>::default()).collect::<Box<[_]>>(),
    )))));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 10) {
        (*x3.borrow()).as_ref().unwrap().borrow_mut()[((*i.borrow()) as u64) as usize] = Pair {
            x: Rc::new(RefCell::new(1)),
            y: Rc::new(RefCell::new(2)),
        };
        (*i.borrow_mut()).prefix_inc();
    }
    let p3_0: Value<Ptr<Pair>> = Rc::new(RefCell::new((*x3.borrow()).as_pointer()));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 10) {
        assert!(
            ((*(*(*p3_0.borrow())
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .x
            .borrow())
                == 1)
        );
        assert!(
            ((*(*(*p3_0.borrow())
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .y
            .borrow())
                == 2)
        );
        ({
            let _k: i32 = 10;
            (*x3.borrow()).as_ref().unwrap().borrow()[((*i.borrow()) as u64) as usize].inc(_k)
        });
        assert!(
            ((*(*(*p3_0.borrow())
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .x
            .borrow())
                == 11)
        );
        assert!(
            ((*(*(*p3_0.borrow())
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .y
            .borrow())
                == 12)
        );
        (*i.borrow_mut()).prefix_inc();
    }
    (*x3.borrow_mut()) = Ptr::alloc_array(
        (0..50_u64)
            .map(|_| <Pair>::default())
            .collect::<Box<[Pair]>>(),
    )
    .to_owned_opt();
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 50) {
        (*x3.borrow()).as_ref().unwrap().borrow_mut()[((*i.borrow()) as u64) as usize] = Pair {
            x: Rc::new(RefCell::new(-1_i32)),
            y: Rc::new(RefCell::new(-2_i32)),
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
            ((*(*(*p3_1.borrow())
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .x
            .borrow())
                == -1_i32)
        );
        assert!(
            ((*(*(*p3_1.borrow())
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .y
            .borrow())
                == -2_i32)
        );
        ({
            let _k: i32 = -10_i32;
            (*x3.borrow()).as_ref().unwrap().borrow()[((*i.borrow()) as u64) as usize].inc(_k)
        });
        assert!(
            ((*(*(*p3_1.borrow())
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .x
            .borrow())
                == -11_i32)
        );
        assert!(
            ((*(*(*p3_1.borrow())
                .offset((*i.borrow()) as isize)
                .upgrade()
                .deref())
            .y
            .borrow())
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
            ptr: Rc::new(RefCell::new((*x.borrow_mut()).take())),
        })))));
    ({
        let _safe_ptr: Ptr<Option<Value<SafePointer>>> = safe_ptr.as_pointer();
        DoStuffWithSafePointer_0(_safe_ptr)
    });
    return ({
        let _safe_ptr: Option<Value<SafePointer>> = (*safe_ptr.borrow_mut()).take();
        Consume_1(_safe_ptr)
    });
}
