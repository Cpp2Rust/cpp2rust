extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, Default)]
#[byte_size(8)]
pub struct SafePointer {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Option<Value<i32>>,
}
impl SafePointer {
    pub fn move_from(_a0: Ptr<SafePointer>) -> Self {
        Self {
            ptr: field!(_a0, ptr).with_mut(|__v: &mut Option<Value<i32>>| __v.take()),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Pair {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
pub fn DoStuffWithSafePointer_0(safe_ptr: Ptr<Option<Value<SafePointer>>>) {
    let x1: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(0)))));
    let x2: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(0)))));
    (*(*x2.borrow()).as_ref().unwrap().borrow_mut()) = 1;
    (x1.as_pointer() as Ptr<Option<Value<i32>>>).write((*x2.borrow_mut()).take());
    let mut raw_ptr1: Ptr<i32> = ((*x1.borrow()).as_pointer());
    raw_ptr1.with_mut(|__v| __v.prefix_inc());
    (field_ptr!(((*safe_ptr.upgrade().deref()).as_pointer()), ptr) as Ptr<Option<Value<i32>>>)
        .write((*x1.borrow_mut()).take());
    ({ SafePointerImpl::inc(&((*safe_ptr.upgrade().deref()).as_pointer())) });
    ({ SafePointerImpl::inc(&((*safe_ptr.upgrade().deref()).as_pointer())) });
    let x3: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(10)))));
    let x4: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(20)))));
    let __rhs = ((*(*x3.borrow()).as_ref().unwrap().borrow())
        + (*(*x4.borrow()).as_ref().unwrap().borrow()));
    (*(*x3.borrow()).as_ref().unwrap().borrow_mut()) = __rhs;
    (x4.as_pointer() as Ptr<Option<Value<i32>>>).write((*x3.borrow_mut()).take());
    let mut raw_ptr2: Ptr<i32> = ((*x4.borrow()).as_pointer());
    {
        raw_ptr2.with_mut(|__v| *__v = *__v + 1)
    };
    let mut pair: Option<Value<Pair>> = Some(Rc::new(RefCell::new(Pair {
        x: (raw_ptr2.read()),
        y: 5,
    })));
    ({ PairImpl::inc(&(pair.as_pointer()), 10) });
    let __rhs = ({
        ({
            (*{
                (*(*safe_ptr.upgrade().deref()).as_ref().unwrap().borrow())
                    .ptr
                    .clone()
            }
            .as_ref()
            .unwrap()
            .borrow())
        } + { { (*pair.as_ref().unwrap().borrow()).x } })
    } + { { (*pair.as_ref().unwrap().borrow()).y } });
    (*{
        (*(*safe_ptr.upgrade().deref()).as_ref().unwrap().borrow())
            .ptr
            .clone()
    }
    .as_ref()
    .unwrap()
    .borrow_mut()) = __rhs;
}
pub fn Consume_1(safe_ptr: Option<Value<SafePointer>>) -> i32 {
    let safe_ptr: Value<Option<Value<SafePointer>>> = Rc::new(RefCell::new(safe_ptr));
    let mut x: Option<Value<SafePointer>> = (*safe_ptr.borrow_mut()).take();
    let mut p: Option<Value<Pair>> = Ptr::alloc(<Pair>::default()).to_owned_opt();
    return ((*{ (*x.as_ref().unwrap().borrow()).ptr.clone() }
        .as_ref()
        .unwrap()
        .borrow())
        + { (*p.as_ref().unwrap().borrow()).x });
}
pub fn RndStuff_2() {
    let mut x1: Option<Value<Box<[i32]>>> = None;
    let x2: Value<Option<Value<Box<[i32]>>>> = Rc::new(RefCell::new(
        Ptr::alloc_array((0..100_usize).map(|_| 0_i32).collect::<Box<[i32]>>()).to_owned_opt(),
    ));
    let mut i: i32 = 0;
    'loop_: while (i < 100) {
        (*x2.borrow()).as_ref().unwrap().borrow_mut()[(i as usize) as usize] = 1;
        i.prefix_inc();
    }
    (x2.as_pointer() as Ptr<Option<Value<Box<[i32]>>>>).write(
        Ptr::alloc_array((0..200_usize).map(|_| 0_i32).collect::<Box<[i32]>>()).to_owned_opt(),
    );
    let mut i: i32 = 0;
    'loop_: while (i < 200) {
        (*x2.borrow()).as_ref().unwrap().borrow_mut()[(i as usize) as usize] = 2;
        i.prefix_inc();
    }
    let mut p2: Ptr<i32> = (*x2.borrow()).as_pointer();
    let mut i: i32 = 0;
    'loop_: while (i < 200) {
        assert!(((elem!(p2, i).read()) == 2));
        i.prefix_inc();
    }
    let x3: Value<Option<Value<Box<[Pair]>>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
        (0..10_usize)
            .map(|_| <Pair>::default())
            .collect::<Box<[_]>>(),
    )))));
    let mut i: i32 = 0;
    'loop_: while (i < 10) {
        (*x3.borrow()).as_ref().unwrap().borrow_mut()[(i as usize) as usize] = Pair { x: 1, y: 2 };
        i.prefix_inc();
    }
    let mut p3_0: Ptr<Pair> = (*x3.borrow()).as_pointer();
    let mut i: i32 = 0;
    'loop_: while (i < 10) {
        assert!(({ (*elem!(p3_0, i).upgrade().deref()).x } == 1));
        assert!(({ (*elem!(p3_0, i).upgrade().deref()).y } == 2));
        ({
            PairImpl::inc(
                &(*x3.borrow())
                    .as_ref()
                    .unwrap()
                    .as_pointer()
                    .offset((i as usize)),
                10,
            )
        });
        assert!(({ (*elem!(p3_0, i).upgrade().deref()).x } == 11));
        assert!(({ (*elem!(p3_0, i).upgrade().deref()).y } == 12));
        i.prefix_inc();
    }
    (x3.as_pointer() as Ptr<Option<Value<Box<[Pair]>>>>).write(
        Ptr::alloc_array(
            (0..50_usize)
                .map(|_| <Pair>::default())
                .collect::<Box<[Pair]>>(),
        )
        .to_owned_opt(),
    );
    let mut i: i32 = 0;
    'loop_: while (i < 50) {
        (*x3.borrow()).as_ref().unwrap().borrow_mut()[(i as usize) as usize] = Pair {
            x: -1_i32,
            y: -2_i32,
        };
        i.prefix_inc();
    }
    let mut p3_1: Ptr<Pair> = (*x3.borrow()).as_pointer();
    assert!(({ (p3_0).clone() } != { (p3_1).clone() }));
    let mut i: i32 = 0;
    'loop_: while (i < 50) {
        assert!(({ (*elem!(p3_1, i).upgrade().deref()).x } == -1_i32));
        assert!(({ (*elem!(p3_1, i).upgrade().deref()).y } == -2_i32));
        ({
            PairImpl::inc(
                &(*x3.borrow())
                    .as_ref()
                    .unwrap()
                    .as_pointer()
                    .offset((i as usize)),
                -10_i32,
            )
        });
        assert!(({ (*elem!(p3_1, i).upgrade().deref()).x } == -11_i32));
        assert!(({ (*elem!(p3_1, i).upgrade().deref()).y } == -12_i32));
        i.prefix_inc();
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(0)))));
    let safe_ptr: Value<Option<Value<SafePointer>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new({
            let __tmp_0: Value<SafePointer> = Rc::new(RefCell::new(SafePointer {
                ptr: (*x.borrow_mut()).take(),
            }));
            SafePointer::move_from({ __tmp_0.as_pointer() })
        })))));
    ({ DoStuffWithSafePointer_0(safe_ptr.as_pointer()) });
    assert!((({ Consume_1((*safe_ptr.borrow_mut()).take(),) }) == 60));
    return 0;
}
pub trait PairImpl {
    fn inc(&self, k: i32);
}
impl PairImpl for Ptr<Pair> {
    fn inc(&self, mut k: i32) {
        {
            let __rhs = k;
            field!((*self), x).with_mut(|__v| *__v = *__v + __rhs)
        };
        {
            let __rhs = k;
            field!((*self), y).with_mut(|__v| *__v = *__v + __rhs)
        };
    }
}
pub trait SafePointerImpl {
    fn inc(&self);
    fn move_assign(&self, _a0: Ptr<SafePointer>) -> Ptr<SafePointer>;
}
impl SafePointerImpl for Ptr<SafePointer> {
    fn inc(&self) {
        (*(*self)
            .with(|__s| __s.ptr.clone())
            .as_ref()
            .unwrap()
            .borrow_mut())
        .prefix_inc();
    }
    fn move_assign(&self, _a0: Ptr<SafePointer>) -> Ptr<SafePointer> {
        (field_ptr!((*self), ptr) as Ptr<Option<Value<i32>>>)
            .write(field!(_a0, ptr).with_mut(|__v: &mut Option<Value<i32>>| __v.take()));
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
