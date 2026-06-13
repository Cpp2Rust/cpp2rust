extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Inner {
    pub v: Value<i32>,
    pub name: Value<Ptr<::core::ffi::c_char>>,
}
impl Clone for Inner {
    fn clone(&self) -> Self {
        let mut this = Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
            name: Rc::new(RefCell::new((*self.name.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for Inner {}
#[derive()]
pub struct Outer {
    pub p1: Value<Ptr<i32>>,
    pub p2: Value<Ptr<i32>>,
    pub arr: Value<Box<[Ptr<i32>]>>,
    pub cp: Value<Ptr<::core::ffi::c_char>>,
    pub pp: Value<Ptr<Ptr<i32>>>,
    pub inner: Value<Inner>,
    pub x: Value<i32>,
    pub fn_: Value<FnPtr<fn(i32) -> i32>>,
}
impl Clone for Outer {
    fn clone(&self) -> Self {
        let mut this = Self {
            p1: Rc::new(RefCell::new((*self.p1.borrow()).clone())),
            p2: Rc::new(RefCell::new((*self.p2.borrow()).clone())),
            arr: Rc::new(RefCell::new((*self.arr.borrow()).clone())),
            cp: Rc::new(RefCell::new((*self.cp.borrow()).clone())),
            pp: Rc::new(RefCell::new((*self.pp.borrow()).clone())),
            inner: Rc::new(RefCell::new((*self.inner.borrow()).clone())),
            x: Rc::new(RefCell::new((*self.x.borrow()))),
            fn_: Rc::new(RefCell::new((*self.fn_.borrow()).clone())),
        };
        this
    }
}
impl Default for Outer {
    fn default() -> Self {
        Outer {
            p1: Rc::new(RefCell::new(Ptr::<i32>::null())),
            p2: Rc::new(RefCell::new(Ptr::<i32>::null())),
            arr: Rc::new(RefCell::new(
                (0..3)
                    .map(|_| Ptr::<i32>::null())
                    .collect::<Box<[Ptr<i32>]>>(),
            )),
            cp: Rc::new(RefCell::new(Ptr::<::core::ffi::c_char>::null())),
            pp: Rc::new(RefCell::new(Ptr::<Ptr<i32>>::null())),
            inner: <Value<Inner>>::default(),
            x: <Value<i32>>::default(),
            fn_: Rc::new(RefCell::new(FnPtr::null())),
        }
    }
}
impl ByteRepr for Outer {}
#[derive()]
pub struct Foo {
    pub s1: Value<Ptr<::core::ffi::c_char>>,
    pub s2: Value<Ptr<::core::ffi::c_char>>,
    pub fn1: Value<FnPtr<fn(i32) -> i32>>,
    pub fn2: Value<FnPtr<fn(i32) -> i32>>,
    pub n: Value<i32>,
}
impl Clone for Foo {
    fn clone(&self) -> Self {
        let mut this = Self {
            s1: Rc::new(RefCell::new((*self.s1.borrow()).clone())),
            s2: Rc::new(RefCell::new((*self.s2.borrow()).clone())),
            fn1: Rc::new(RefCell::new((*self.fn1.borrow()).clone())),
            fn2: Rc::new(RefCell::new((*self.fn2.borrow()).clone())),
            n: Rc::new(RefCell::new((*self.n.borrow()))),
        };
        this
    }
}
impl Default for Foo {
    fn default() -> Self {
        Foo {
            s1: Rc::new(RefCell::new(Ptr::<::core::ffi::c_char>::null())),
            s2: Rc::new(RefCell::new(Ptr::<::core::ffi::c_char>::null())),
            fn1: Rc::new(RefCell::new(FnPtr::null())),
            fn2: Rc::new(RefCell::new(FnPtr::null())),
            n: <Value<i32>>::default(),
        }
    }
}
impl ByteRepr for Foo {}
thread_local!(
    pub static static_fn_0: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(FnPtr::null()));
);
thread_local!(
    pub static static_outer_1: Value<Outer> = Rc::new(RefCell::new(<Outer>::default()));
);
thread_local!(
    pub static static_inner_array_2: Value<Box<[Inner]>> = Rc::new(RefCell::new(
        (0..2).map(|_| <Inner>::default()).collect::<Box<[Inner]>>(),
    ));
);
thread_local!(
    pub static static_foo_3: Value<Foo> = Rc::new(RefCell::new(Foo {
        s1: Rc::new(RefCell::new(Ptr::from_string_literal(b"hello"))),
        s2: Rc::new(RefCell::new(Ptr::<::core::ffi::c_char>::null())),
        fn1: Rc::new(RefCell::new(FnPtr::null())),
        fn2: Rc::new(RefCell::new(FnPtr::null())),
        n: Rc::new(RefCell::new(42)),
    }));
);
thread_local!(
    pub static static_foo_array_4: Value<Box<[Foo]>> = Rc::new(RefCell::new(Box::new([
        Foo {
            s1: Rc::new(RefCell::new(Ptr::from_string_literal(b"first"))),
            s2: Rc::new(RefCell::new(Ptr::<::core::ffi::c_char>::null())),
            fn1: Rc::new(RefCell::new(FnPtr::null())),
            fn2: Rc::new(RefCell::new(FnPtr::null())),
            n: Rc::new(RefCell::new(1)),
        },
        Foo {
            s1: Rc::new(RefCell::new(Ptr::from_string_literal(b"second"))),
            s2: Rc::new(RefCell::new(Ptr::<::core::ffi::c_char>::null())),
            fn1: Rc::new(RefCell::new(FnPtr::null())),
            fn2: Rc::new(RefCell::new(FnPtr::null())),
            n: Rc::new(RefCell::new(2)),
        },
    ])));
);
pub fn check_local_static_5() {
    thread_local!(
        static local_outer_6: Value<Outer> = Rc::new(RefCell::new(<Outer>::default()));
    );
    thread_local!(
        static local_fn_7: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(FnPtr::null()));
    );
    thread_local!(
        static local_p_8: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
    );
    assert!((*(*local_outer_6.with(Value::clone).borrow()).p1.borrow()).is_null());
    assert!((*(*local_outer_6.with(Value::clone).borrow()).fn_.borrow()).is_null());
    assert!((*local_fn_7.with(Value::clone).borrow()).is_null());
    assert!((*local_p_8.with(Value::clone).borrow()).is_null());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((*static_fn_0.with(Value::clone).borrow()).is_null());
    assert!((*(*static_outer_1.with(Value::clone).borrow()).p1.borrow()).is_null());
    assert!((*(*static_outer_1.with(Value::clone).borrow()).p2.borrow()).is_null());
    assert!((*(*static_outer_1.with(Value::clone).borrow()).cp.borrow()).is_null());
    assert!((*(*static_outer_1.with(Value::clone).borrow()).pp.borrow()).is_null());
    assert!((*(*static_outer_1.with(Value::clone).borrow()).fn_.borrow()).is_null());
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 3) {
        assert!(
            ((*(*static_outer_1.with(Value::clone).borrow()).arr.borrow())[(*i.borrow()) as usize])
                .is_null()
        );
        (*i.borrow_mut()).prefix_inc();
    }
    assert!(
        (*(*(*static_outer_1.with(Value::clone).borrow()).inner.borrow())
            .name
            .borrow())
        .is_null()
    );
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 2) {
        assert!(
            (*(*static_inner_array_2.with(Value::clone).borrow())[(*i.borrow()) as usize]
                .name
                .borrow())
            .is_null()
        );
        (*i.borrow_mut()).prefix_inc();
    }
    assert!((*(*static_foo_3.with(Value::clone).borrow()).s2.borrow()).is_null());
    assert!((*(*static_foo_3.with(Value::clone).borrow()).fn1.borrow()).is_null());
    assert!((*(*static_foo_3.with(Value::clone).borrow()).fn2.borrow()).is_null());
    assert!(((*(*static_foo_3.with(Value::clone).borrow()).n.borrow()) == 42));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 2) {
        assert!(
            (*(*static_foo_array_4.with(Value::clone).borrow())[(*i.borrow()) as usize]
                .s2
                .borrow())
            .is_null()
        );
        assert!(
            (*(*static_foo_array_4.with(Value::clone).borrow())[(*i.borrow()) as usize]
                .fn1
                .borrow())
            .is_null()
        );
        assert!(
            (*(*static_foo_array_4.with(Value::clone).borrow())[(*i.borrow()) as usize]
                .fn2
                .borrow())
            .is_null()
        );
        (*i.borrow_mut()).prefix_inc();
    }
    ({ check_local_static_5() });
    return 0;
}
