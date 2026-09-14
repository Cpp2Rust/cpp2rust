extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn next_0() -> i32 {
    thread_local!(
        static counter_1: Value<i32> = Rc::new(RefCell::new(0));
    );
    return (*counter_1.with(Value::clone).borrow_mut()).prefix_inc();
}
pub fn marker_2(tag: u8) -> u8 {
    let tag: Value<u8> = Rc::new(RefCell::new(tag));
    return (((((*tag.borrow()) as i32) << 3) | 2) as u8);
}
thread_local!(
    pub static signature_3: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        ({ marker_2(1_u8) }),
        4_u8,
        ('B' as u8),
    ])));
);
thread_local!(
    pub static single_4: Value<u8> = Rc::new(RefCell::new(({ marker_2(2_u8) })));
);
thread_local!(
    pub static from_call_5: Value<i32> = Rc::new(RefCell::new(({ next_0() })));
);
thread_local!(
    pub static depends_on_call_6: Value<i32> = Rc::new(RefCell::new(
        ((*from_call_5.with(Value::clone).borrow()) + 1),
    ));
);
#[derive()]
pub struct Ctor {
    pub v: Value<i32>,
}
impl Ctor {
    pub fn Ctor1() -> Self {
        let __this: Value<Ctor> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new(({ next_0() }))),
        }));
        let this: Ptr<Ctor> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn Ctor2(x: i32) -> Self {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let __this: Value<Ctor> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*x.borrow()))),
        }));
        let this: Ptr<Ctor> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for Ctor {
    fn clone(&self) -> Self {
        let __this: Value<Ctor> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        }));
        let this: Ptr<Ctor> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Ctor {
    fn default() -> Self {
        { Ctor::Ctor1() }
    }
}
impl ByteRepr for Ctor {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
thread_local!(
    pub static default_ctor_7: Value<Ctor> = Rc::new(RefCell::new(Ctor::Ctor1()));
);
thread_local!(
    pub static arg_ctor_8: Value<Ctor> = Rc::new(RefCell::new(Ctor::Ctor2({ 7 })));
);
thread_local!(
    pub static str_9: Value<Vec<u8>> = Rc::new(RefCell::new(
        Ptr::from_string_literal(b"abc")
            .to_c_string_iterator()
            .chain(std::iter::once(0))
            .collect::<Vec<u8>>(),
    ));
);
thread_local!();
thread_local!(
    pub static inline_member_11: Value<Ctor> = Rc::new(RefCell::new(Ctor::Ctor2({ 5 })));
);
#[derive(Default)]
pub struct Holder {}
impl Clone for Holder {
    fn clone(&self) -> Self {
        let __this: Value<Holder> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<Holder> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Holder {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
thread_local!(
    pub static member_10: Value<i32> = Rc::new(RefCell::new(({ next_0() })));
);
pub fn local_static_12() -> i32 {
    thread_local!(
        static once_13: Value<i32> = Rc::new(RefCell::new(({ next_0() })));
    );
    thread_local!(
        static local_ctor_14: Value<Ctor> = Rc::new(RefCell::new(Ctor::Ctor2({ 3 })));
    );
    return ((*once_13.with(Value::clone).borrow())
        + (*(*local_ctor_14.with(Value::clone).borrow()).v.borrow()));
}
#[derive()]
pub struct Singleton {
    pub hits: Value<i32>,
}
impl Singleton {
    pub fn Singleton() -> Self {
        let __this: Value<Singleton> = Rc::new(RefCell::new(Self {
            hits: Rc::new(RefCell::new(0)),
        }));
        let this: Ptr<Singleton> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn instance() -> Ptr<Singleton> {
        thread_local!(
            static s_15: Value<Singleton> = Rc::new(RefCell::new(Singleton::Singleton()));
        );
        return s_15.with(Value::clone).as_pointer();
    }
}
impl Clone for Singleton {
    fn clone(&self) -> Self {
        let __this: Value<Singleton> = Rc::new(RefCell::new(Self {
            hits: Rc::new(RefCell::new((*self.hits.borrow()))),
        }));
        let this: Ptr<Singleton> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Singleton {
    fn default() -> Self {
        { Singleton::Singleton() }
    }
}
impl ByteRepr for Singleton {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.hits.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            hits: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((((*signature_3.with(Value::clone).borrow())[(0) as usize] as i32) == 10));
    assert!((((*signature_3.with(Value::clone).borrow())[(1) as usize] as i32) == 4));
    assert!((((*single_4.with(Value::clone).borrow()) as i32) == 18));
    assert!(((*from_call_5.with(Value::clone).borrow()) == 1));
    assert!(((*depends_on_call_6.with(Value::clone).borrow()) == 2));
    assert!(((*(*default_ctor_7.with(Value::clone).borrow()).v.borrow()) == 2));
    assert!(((*(*arg_ctor_8.with(Value::clone).borrow()).v.borrow()) == 7));
    assert!(
        (*str_9.with(Value::clone).borrow())
            .iter()
            .copied()
            .take((*str_9.with(Value::clone).borrow()).len().saturating_sub(1))
            .eq(Ptr::from_string_literal(b"abc").to_c_string_iterator())
    );
    assert!(((*member_10.with(Value::clone).borrow()) == 3));
    assert!(((*(*inline_member_11.with(Value::clone).borrow()).v.borrow()) == 5));
    assert!((({ local_static_12() }) == 7));
    assert!((({ local_static_12() }) == 7));
    (*(*({ Singleton::instance() }).upgrade().deref())
        .hits
        .borrow_mut())
    .postfix_inc();
    (*(*({ Singleton::instance() }).upgrade().deref())
        .hits
        .borrow_mut())
    .postfix_inc();
    assert!(
        ((*(*({ Singleton::instance() }).upgrade().deref())
            .hits
            .borrow())
            == 2)
    );
    assert!((({ Singleton::instance() }) == ({ Singleton::instance() })));
    return 0;
}
