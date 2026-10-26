extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static destroyed_0: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static last_1: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub v: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(8)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(4)]
    pub inner: S,
    #[offset(4)]
    pub w: i32,
}
pub fn make_2(mut v: i32) -> S {
    return S { v: v };
}
pub fn make_outer_3() -> Outer {
    return Outer {
        inner: S { v: 7 },
        w: 8,
    };
}
pub fn use_4(s: Ptr<S>) -> i32 {
    assert!((destroyed_0.with(|rc| *rc.borrow()) == 0));
    return s.with(|__s| __s.v);
}
pub fn value_5(s: Ptr<S>) -> i32 {
    return s.with(|__s| __s.v);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ make_2(1) });
    assert!((destroyed_0.with(|rc| *rc.borrow()) == 1) && (last_1.with(|rc| *rc.borrow()) == 1));
    destroyed_0.with(|rc| *rc.borrow_mut() = 0);
    &({ make_2(2) });
    assert!((destroyed_0.with(|rc| *rc.borrow()) == 1) && (last_1.with(|rc| *rc.borrow()) == 2));
    destroyed_0.with(|rc| *rc.borrow_mut() = 0);
    let mut x: i32 = { ({ make_2(3) }).v };
    assert!(
        ((x == 3) && (destroyed_0.with(|rc| *rc.borrow()) == 1))
            && (last_1.with(|rc| *rc.borrow()) == 3)
    );
    destroyed_0.with(|rc| *rc.borrow_mut() = 0);
    x = ({ SImpl::get(&Rc::new(RefCell::new(({ make_2(4) }))).as_pointer()) });
    assert!(
        ((x == 4) && (destroyed_0.with(|rc| *rc.borrow()) == 1))
            && (last_1.with(|rc| *rc.borrow()) == 4)
    );
    destroyed_0.with(|rc| *rc.borrow_mut() = 0);
    x = { ({ make_outer_3() }).inner.v };
    assert!(
        ((x == 7) && (destroyed_0.with(|rc| *rc.borrow()) == 1))
            && (last_1.with(|rc| *rc.borrow()) == 7)
    );
    destroyed_0.with(|rc| *rc.borrow_mut() = 0);
    x = { S { v: 5 }.v };
    assert!((x == 5) && (destroyed_0.with(|rc| *rc.borrow()) == 1));
    destroyed_0.with(|rc| *rc.borrow_mut() = 0);
    x = ({
        let _s: Value<S> = Rc::new(RefCell::new(({ make_2(6) })));
        use_4(_s.as_pointer())
    });
    assert!(
        ((x == 6) && (destroyed_0.with(|rc| *rc.borrow()) == 1))
            && (last_1.with(|rc| *rc.borrow()) == 6)
    );
    destroyed_0.with(|rc| *rc.borrow_mut() = 0);
    x = (({
        let _s: Value<S> = Rc::new(RefCell::new(S { v: 1 }));
        value_5(_s.as_pointer())
    }) + ({
        let _s: Value<S> = Rc::new(RefCell::new(S { v: 2 }));
        value_5(_s.as_pointer())
    }));
    assert!((x == 3) && (destroyed_0.with(|rc| *rc.borrow()) == 2));
    destroyed_0.with(|rc| *rc.borrow_mut() = 0);
    let __rhs = {
        (if (x == 3) {
            ({ make_2(9) })
        } else {
            S { v: 10 }
        })
        .v
    };
    x = __rhs;
    assert!(
        ((x == 9) && (destroyed_0.with(|rc| *rc.borrow()) == 1))
            && (last_1.with(|rc| *rc.borrow()) == 9)
    );
    destroyed_0.with(|rc| *rc.borrow_mut() = 0);
    {
        let __tmp_0: Value<S> = Rc::new(RefCell::new(({ make_2(11) })));
        let r: Ptr<S> = __tmp_0.as_pointer();
        assert!((destroyed_0.with(|rc| *rc.borrow()) == 0) && (r.with(|__s| __s.v) == 11));
    }
    assert!((destroyed_0.with(|rc| *rc.borrow()) == 1) && (last_1.with(|rc| *rc.borrow()) == 11));
    destroyed_0.with(|rc| *rc.borrow_mut() = 0);
    {
        let s: Value<S> = Rc::new(RefCell::new(S { v: 12 }));
        let _dtor_s = ScopedDestructor::new(&s, |__p| __p.destructor());
        (*s.borrow_mut()) = ({ make_2(13) });
        assert!(
            ((destroyed_0.with(|rc| *rc.borrow()) == 1) && (last_1.with(|rc| *rc.borrow()) == 13))
                && ({ (*s.borrow()).v } == 13)
        );
    }
    assert!((destroyed_0.with(|rc| *rc.borrow()) == 2) && (last_1.with(|rc| *rc.borrow()) == 13));
    return 0;
}
pub trait OuterImpl {
    fn destructor(&self);
}
impl OuterImpl for Ptr<Outer> {
    fn destructor(&self) {
        field_ptr!(self, inner).destructor();
    }
}
pub trait SImpl {
    fn destructor(&self);
    fn get(&self) -> i32;
}
impl SImpl for Ptr<S> {
    fn destructor(&self) {
        (*destroyed_0.with(Value::clone).borrow_mut()).postfix_inc();
        last_1.with(|rc| *rc.borrow_mut() = (*self).with(|__s| __s.v));
    }
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.v);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = destroyed_0.with(|_| ());
    let _ = last_1.with(|_| ());
}
