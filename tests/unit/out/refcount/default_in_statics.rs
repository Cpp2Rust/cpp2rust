extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Inner {
    #[offset(0)]
    pub v: i32,
    #[offset(8)]
    pub name: Ptr<u8>,
}
impl ByteRepr for Inner {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.v.to_bytes(&mut buf[0..4]);
        self.name.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: <i32>::from_bytes(&buf[0..4]),
            name: <Ptr<u8>>::from_bytes(&buf[8..16]),
        }
    }
}
#[derive(Record, VaArg, FnPtrArg)]
pub struct Outer {
    #[offset(0)]
    pub p1: Ptr<i32>,
    #[offset(8)]
    pub p2: Ptr<i32>,
    #[offset(16)]
    pub arr: Value<Box<[Ptr<i32>]>>,
    #[offset(40)]
    pub cp: Ptr<u8>,
    #[offset(48)]
    pub pp: Ptr<Ptr<i32>>,
    #[offset(56)]
    pub inner: Inner,
    #[offset(72)]
    pub x: i32,
    #[offset(80)]
    pub fn_: FnPtr<fn(i32) -> i32>,
}
impl Clone for Outer {
    fn clone(&self) -> Self {
        let __this: Value<Outer> = Rc::new(RefCell::new(Self {
            p1: { self.p1.clone() },
            p2: { self.p2.clone() },
            arr: Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 3, _>(
                |__i: usize| ((*self.arr.borrow())[(__i) as usize]).clone(),
            )))),
            cp: { self.cp.clone() },
            pp: { self.pp.clone() },
            inner: { self.inner.clone() },
            x: { self.x },
            fn_: { self.fn_.clone() },
        }));
        let this: Ptr<Outer> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Outer {
    fn default() -> Self {
        Outer {
            p1: Ptr::<i32>::null(),
            p2: Ptr::<i32>::null(),
            arr: Rc::new(RefCell::new(
                (0..3)
                    .map(|_| Ptr::<i32>::null())
                    .collect::<Box<[Ptr<i32>]>>(),
            )),
            cp: Ptr::<u8>::null(),
            pp: Ptr::<Ptr<i32>>::null(),
            inner: <Inner>::default(),
            x: 0_i32,
            fn_: FnPtr::<fn(i32) -> i32>::null(),
        }
    }
}
impl ByteRepr for Outer {
    fn byte_size() -> usize {
        88
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.p1.to_bytes(&mut buf[0..8]);
        self.p2.to_bytes(&mut buf[8..16]);
        (*self.arr.borrow()).to_bytes(&mut buf[16..40]);
        self.cp.to_bytes(&mut buf[40..48]);
        self.pp.to_bytes(&mut buf[48..56]);
        self.inner.to_bytes(&mut buf[56..72]);
        self.x.to_bytes(&mut buf[72..76]);
        self.fn_.to_bytes(&mut buf[80..88]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            p1: <Ptr<i32>>::from_bytes(&buf[0..8]),
            p2: <Ptr<i32>>::from_bytes(&buf[8..16]),
            arr: Rc::new(RefCell::new(<Box<[Ptr<i32>]>>::from_bytes(&buf[16..40]))),
            cp: <Ptr<u8>>::from_bytes(&buf[40..48]),
            pp: <Ptr<Ptr<i32>>>::from_bytes(&buf[48..56]),
            inner: <Inner>::from_bytes(&buf[56..72]),
            x: <i32>::from_bytes(&buf[72..76]),
            fn_: <FnPtr<fn(i32) -> i32>>::from_bytes(&buf[80..88]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg)]
pub struct Foo {
    #[offset(0)]
    pub s1: Ptr<u8>,
    #[offset(8)]
    pub s2: Ptr<u8>,
    #[offset(16)]
    pub fn1: FnPtr<fn(i32) -> i32>,
    #[offset(24)]
    pub fn2: FnPtr<fn(i32) -> i32>,
    #[offset(32)]
    pub n: i32,
}
impl Default for Foo {
    fn default() -> Self {
        Foo {
            s1: Ptr::<u8>::null(),
            s2: Ptr::<u8>::null(),
            fn1: FnPtr::<fn(i32) -> i32>::null(),
            fn2: FnPtr::<fn(i32) -> i32>::null(),
            n: 0_i32,
        }
    }
}
impl ByteRepr for Foo {
    fn byte_size() -> usize {
        40
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.s1.to_bytes(&mut buf[0..8]);
        self.s2.to_bytes(&mut buf[8..16]);
        self.fn1.to_bytes(&mut buf[16..24]);
        self.fn2.to_bytes(&mut buf[24..32]);
        self.n.to_bytes(&mut buf[32..36]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            s1: <Ptr<u8>>::from_bytes(&buf[0..8]),
            s2: <Ptr<u8>>::from_bytes(&buf[8..16]),
            fn1: <FnPtr<fn(i32) -> i32>>::from_bytes(&buf[16..24]),
            fn2: <FnPtr<fn(i32) -> i32>>::from_bytes(&buf[24..32]),
            n: <i32>::from_bytes(&buf[32..36]),
        }
    }
}
thread_local!(
    pub static static_fn_0: Value<FnPtr<fn(i32) -> i32>> =
        Rc::new(RefCell::new(FnPtr::<fn(i32) -> i32>::null()));
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
        s1: Ptr::<u8>::from_string_literal(b"hello"),
        s2: Ptr::<u8>::null(),
        fn1: FnPtr::<fn(i32) -> i32>::null(),
        fn2: FnPtr::<fn(i32) -> i32>::null(),
        n: 42,
    }));
);
thread_local!(
    pub static static_foo_array_4: Value<Box<[Foo]>> = Rc::new(RefCell::new(Box::new([
        Foo {
            s1: Ptr::<u8>::from_string_literal(b"first"),
            s2: Ptr::<u8>::null(),
            fn1: FnPtr::<fn(i32) -> i32>::null(),
            fn2: FnPtr::<fn(i32) -> i32>::null(),
            n: 1,
        },
        Foo {
            s1: Ptr::<u8>::from_string_literal(b"second"),
            s2: Ptr::<u8>::null(),
            fn1: FnPtr::<fn(i32) -> i32>::null(),
            fn2: FnPtr::<fn(i32) -> i32>::null(),
            n: 2,
        },
    ])));
);
pub fn check_local_static_5() {
    thread_local!(
        static local_outer_6: Value<Outer> = Rc::new(RefCell::new(<Outer>::default()));
    );
    thread_local!(
        static local_fn_7: Value<FnPtr<fn(i32) -> i32>> =
            Rc::new(RefCell::new(FnPtr::<fn(i32) -> i32>::null()));
    );
    thread_local!(
        static local_p_8: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
    );
    assert!(({ (*local_outer_6.with(Value::clone).borrow()).p1.clone() }).is_null());
    assert!(({ (*local_outer_6.with(Value::clone).borrow()).fn_.clone() }).is_null());
    assert!((*local_fn_7.with(Value::clone).borrow()).is_null());
    assert!((*local_p_8.with(Value::clone).borrow()).is_null());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((*static_fn_0.with(Value::clone).borrow()).is_null());
    assert!(({ (*static_outer_1.with(Value::clone).borrow()).p1.clone() }).is_null());
    assert!(({ (*static_outer_1.with(Value::clone).borrow()).p2.clone() }).is_null());
    assert!(({ (*static_outer_1.with(Value::clone).borrow()).cp.clone() }).is_null());
    assert!(({ (*static_outer_1.with(Value::clone).borrow()).pp.clone() }).is_null());
    assert!(({ (*static_outer_1.with(Value::clone).borrow()).fn_.clone() }).is_null());
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 3) {
        assert!(
            ((*(*static_outer_1.with(Value::clone).borrow()).arr.borrow())[(*i.borrow()) as usize])
                .is_null()
        );
        (*i.borrow_mut()).prefix_inc();
    }
    assert!(
        ({
            (*static_outer_1.with(Value::clone).borrow())
                .inner
                .name
                .clone()
        })
        .is_null()
    );
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 2) {
        assert!(
            ({
                (*static_inner_array_2.with(Value::clone).borrow())[(*i.borrow()) as usize]
                    .name
                    .clone()
            })
            .is_null()
        );
        (*i.borrow_mut()).prefix_inc();
    }
    assert!(({ (*static_foo_3.with(Value::clone).borrow()).s2.clone() }).is_null());
    assert!(({ (*static_foo_3.with(Value::clone).borrow()).fn1.clone() }).is_null());
    assert!(({ (*static_foo_3.with(Value::clone).borrow()).fn2.clone() }).is_null());
    assert!(({ (*static_foo_3.with(Value::clone).borrow()).n } == 42));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 2) {
        assert!(
            ({
                (*static_foo_array_4.with(Value::clone).borrow())[(*i.borrow()) as usize]
                    .s2
                    .clone()
            })
            .is_null()
        );
        assert!(
            ({
                (*static_foo_array_4.with(Value::clone).borrow())[(*i.borrow()) as usize]
                    .fn1
                    .clone()
            })
            .is_null()
        );
        assert!(
            ({
                (*static_foo_array_4.with(Value::clone).borrow())[(*i.borrow()) as usize]
                    .fn2
                    .clone()
            })
            .is_null()
        );
        (*i.borrow_mut()).prefix_inc();
    }
    ({ check_local_static_5() });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = static_fn_0.with(|_| ());
    let _ = static_outer_1.with(|_| ());
    let _ = static_inner_array_2.with(|_| ());
    let _ = static_foo_3.with(|_| ());
    let _ = static_foo_array_4.with(|_| ());
}
