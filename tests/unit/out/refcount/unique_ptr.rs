extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
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
    (*x2.borrow()).as_pointer().write(1);
    x1.as_pointer().write((*x2.borrow_mut()).take());
    let mut raw_ptr1: Ptr<i32> = ((*x1.borrow()).as_pointer());
    raw_ptr1.with_mut(|__v| __v.prefix_inc());
    field_ptr!(((*safe_ptr.upgrade().deref()).as_pointer()), ptr).write((*x1.borrow_mut()).take());
    ({ SafePointerImpl::inc(&((*safe_ptr.upgrade().deref()).as_pointer())) });
    ({ SafePointerImpl::inc(&((*safe_ptr.upgrade().deref()).as_pointer())) });
    let x3: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(10)))));
    let x4: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(20)))));
    let __rhs = (((*x3.borrow()).as_pointer().read()) + ((*x4.borrow()).as_pointer().read()));
    (*x3.borrow()).as_pointer().write(__rhs);
    x4.as_pointer().write((*x3.borrow_mut()).take());
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
            ((*safe_ptr.upgrade().deref())
                .as_pointer()
                .with(|__s| (__s).ptr.clone())
                .as_pointer()
                .read())
        } + { pair.as_pointer().with(|__s| (__s).x) })
    } + { pair.as_pointer().with(|__s| (__s).y) });
    (*safe_ptr.upgrade().deref())
        .as_pointer()
        .with(|__s| (__s).ptr.clone())
        .as_pointer()
        .write(__rhs);
}
pub fn Consume_1(safe_ptr: Option<Value<SafePointer>>) -> i32 {
    let safe_ptr: Value<Option<Value<SafePointer>>> = Rc::new(RefCell::new(safe_ptr));
    let mut x: Option<Value<SafePointer>> = (*safe_ptr.borrow_mut()).take();
    let mut p: Option<Value<Pair>> = Ptr::alloc(<Pair>::default()).to_owned_opt();
    return ((x
        .as_pointer()
        .with(|__s| (__s).ptr.clone())
        .as_pointer()
        .read())
        + p.as_pointer().with(|__s| __s.x));
}
pub fn RndStuff_2() {
    let mut x1: Option<Value<Box<[i32]>>> = None;
    let x2: Value<Option<Value<Box<[i32]>>>> = Rc::new(RefCell::new(
        Ptr::alloc_array((0..100_usize).map(|_| 0_i32).collect::<Box<[i32]>>()).to_owned_opt(),
    ));
    let mut i: i32 = 0;
    'loop_: while (i < 100) {
        ((*x2.borrow()).as_pointer() as Ptr<i32>)
            .offset((i as usize) as isize)
            .write(1);
        i.prefix_inc();
    }
    x2.as_pointer().write(
        Ptr::alloc_array((0..200_usize).map(|_| 0_i32).collect::<Box<[i32]>>()).to_owned_opt(),
    );
    let mut i: i32 = 0;
    'loop_: while (i < 200) {
        ((*x2.borrow()).as_pointer() as Ptr<i32>)
            .offset((i as usize) as isize)
            .write(2);
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
        ((*x3.borrow()).as_pointer() as Ptr<Pair>)
            .offset((i as usize) as isize)
            .write(Pair { x: 1, y: 2 });
        i.prefix_inc();
    }
    let mut p3_0: Ptr<Pair> = (*x3.borrow()).as_pointer();
    let mut i: i32 = 0;
    'loop_: while (i < 10) {
        assert!((elem!(p3_0, i).with(|__s| __s.x) == 1));
        assert!((elem!(p3_0, i).with(|__s| __s.y) == 2));
        ({
            PairImpl::inc(
                &((*x3.borrow()).as_pointer() as Ptr<Pair>).offset((i as usize) as isize),
                10,
            )
        });
        assert!((elem!(p3_0, i).with(|__s| __s.x) == 11));
        assert!((elem!(p3_0, i).with(|__s| __s.y) == 12));
        i.prefix_inc();
    }
    x3.as_pointer().write(
        Ptr::alloc_array(
            (0..50_usize)
                .map(|_| <Pair>::default())
                .collect::<Box<[Pair]>>(),
        )
        .to_owned_opt(),
    );
    let mut i: i32 = 0;
    'loop_: while (i < 50) {
        ((*x3.borrow()).as_pointer() as Ptr<Pair>)
            .offset((i as usize) as isize)
            .write(Pair {
                x: -1_i32,
                y: -2_i32,
            });
        i.prefix_inc();
    }
    let mut p3_1: Ptr<Pair> = (*x3.borrow()).as_pointer();
    assert!(({ (p3_0).clone() } != { (p3_1).clone() }));
    let mut i: i32 = 0;
    'loop_: while (i < 50) {
        assert!((elem!(p3_1, i).with(|__s| __s.x) == -1_i32));
        assert!((elem!(p3_1, i).with(|__s| __s.y) == -2_i32));
        ({
            PairImpl::inc(
                &((*x3.borrow()).as_pointer() as Ptr<Pair>).offset((i as usize) as isize),
                -10_i32,
            )
        });
        assert!((elem!(p3_1, i).with(|__s| __s.x) == -11_i32));
        assert!((elem!(p3_1, i).with(|__s| __s.y) == -12_i32));
        i.prefix_inc();
    }
}
pub fn unique_ptr_that_wraps_container_3() {
    let mut v: Option<Value<Box<[Vec<i8>]>>> = Some(Rc::new(RefCell::new(
        (0..2_usize).map(|_| vec![0]).collect::<Box<[_]>>(),
    )));
    assert!(
        (((*(v.as_pointer() as Ptr<Vec<i8>>)
            .offset(0_usize as isize)
            .upgrade()
            .deref())
        .len()
            - 1)
            == 0_usize)
    );
    assert!(
        (((*(v.as_pointer() as Ptr<Vec<i8>>)
            .offset(1_usize as isize)
            .upgrade()
            .deref())
        .len()
            - 1)
            == 0_usize)
    );
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(0)))));
    let safe_ptr: Value<Option<Value<SafePointer>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(SafePointer {
            ptr: (*x.borrow_mut()).take(),
        })))));
    ({ DoStuffWithSafePointer_0(safe_ptr.as_pointer()) });
    assert!((({ Consume_1((*safe_ptr.borrow_mut()).take(),) }) == 60));
    let p: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Ptr::alloc(1).to_owned_opt()));
    {
        let _p: Ptr<_> = Ptr::alloc(2);
        p.as_pointer().write(_p.to_owned_opt())
    };
    assert!((((*p.borrow()).as_pointer().read()) == 2));
    p.as_pointer().write(None);
    assert!((*p.borrow()).is_none());
    ({ unique_ptr_that_wraps_container_3() });
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
        (*self)
            .with(|__s| __s.ptr.clone())
            .as_pointer()
            .with_mut(|__v| __v.prefix_inc());
    }
    fn move_assign(&self, _a0: Ptr<SafePointer>) -> Ptr<SafePointer> {
        field_ptr!((*self), ptr)
            .write(field!(_a0, ptr).with_mut(|__v: &mut Option<Value<i32>>| __v.take()));
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
