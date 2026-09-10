extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static alive_0: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static copies_1: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive()]
pub struct Buffer {
    pub data: Value<Box<[i32]>>,
    pub size: Value<i32>,
}
impl Buffer {
    pub fn Buffer(size: i32) -> Self {
        let size: Value<i32> = Rc::new(RefCell::new(size));
        let __this: Value<Buffer> = Rc::new(RefCell::new(Self {
            data: Rc::new(RefCell::new(
                (0..4).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
            )),
            size: Rc::new(RefCell::new((*size.borrow()))),
        }));
        let this: Ptr<Buffer> = __this.as_pointer();
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 4) {
            let __rhs = if ((*i.borrow()) < (*size.borrow())) {
                (*i.borrow())
            } else {
                -1_i32
            };
            (*(*this.upgrade().deref()).data.borrow_mut())[(*i.borrow()) as usize] = __rhs;
            (*i.borrow_mut()).prefix_inc();
        }
        (*alive_0.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn Buffer_pconstBuffer(o: Ptr<Buffer>) -> Self {
        let __this: Value<Buffer> = Rc::new(RefCell::new(Self {
            data: Rc::new(RefCell::new(
                (0..4).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
            )),
            size: Rc::new(RefCell::new((*(*o.upgrade().deref()).size.borrow()))),
        }));
        let this: Ptr<Buffer> = __this.as_pointer();
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 4) {
            let __rhs = (*(*o.upgrade().deref()).data.borrow())[(*i.borrow()) as usize];
            (*(*this.upgrade().deref()).data.borrow_mut())[(*i.borrow()) as usize] = __rhs;
            (*i.borrow_mut()).prefix_inc();
        }
        (*alive_0.with(Value::clone).borrow_mut()).prefix_inc();
        (*copies_1.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for Buffer {
    fn clone(&self) -> Self {
        let __src: Value<Buffer> = Rc::new(RefCell::new(Buffer {
            data: self.data.clone(),
            size: self.size.clone(),
        }));
        Buffer::Buffer_pconstBuffer(__src.as_pointer())
    }
}
impl Default for Buffer {
    fn default() -> Self {
        Buffer {
            data: Rc::new(RefCell::new(
                (0..4).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
            )),
            size: <Value<i32>>::default(),
        }
    }
}
impl ByteRepr for Buffer {
    fn byte_size() -> usize {
        20
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.data.borrow()).to_bytes(&mut buf[0..16]);
        (*self.size.borrow()).to_bytes(&mut buf[16..20]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            data: Rc::new(RefCell::new(<Box<[i32]>>::from_bytes(&buf[0..16]))),
            size: Rc::new(RefCell::new(<i32>::from_bytes(&buf[16..20]))),
        }
    }
}
pub fn sum_2(b: Ptr<Buffer>) -> i32 {
    let s: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*(*b.upgrade().deref()).size.borrow())
    } {
        (*s.borrow_mut()) += (*(*b.upgrade().deref()).data.borrow())[(*i.borrow()) as usize];
        (*i.borrow_mut()).prefix_inc();
    }
    return (*s.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        let a: Value<Buffer> = Rc::new(RefCell::new(Buffer::Buffer({ 4 })));
        let _dtor_a = ScopedDestructor::new(&a, |__p| __p.destructor());
        let b: Value<Buffer> = Rc::new(RefCell::new(Buffer::Buffer_pconstBuffer({
            a.as_pointer()
        })));
        let _dtor_b = ScopedDestructor::new(&b, |__p| __p.destructor());
        assert!(
            ((*alive_0.with(Value::clone).borrow()) == 2)
                && ((*copies_1.with(Value::clone).borrow()) == 1)
        );
        (*(*b.borrow()).data.borrow_mut())[(0) as usize] = 100;
        assert!(((*(*a.borrow()).data.borrow())[(0) as usize] == 0));
        let c: Value<Buffer> = Rc::new(RefCell::new(Buffer::Buffer({ 2 })));
        let _dtor_c = ScopedDestructor::new(&c, |__p| __p.destructor());
        ({ BufferImpl::operator_assign(&c.as_pointer(), a.as_pointer()) });
        assert!(
            ((*(*c.borrow()).size.borrow()) == 4)
                && ((*(*c.borrow()).data.borrow())[(3) as usize] == 3)
        );
        assert!(
            ((*alive_0.with(Value::clone).borrow()) == 3)
                && ((*copies_1.with(Value::clone).borrow()) == 2)
        );
        ({
            let _o: Ptr<Buffer> = c.as_pointer();
            BufferImpl::operator_assign(&c.as_pointer(), _o)
        });
        assert!(((*copies_1.with(Value::clone).borrow()) == 2));
        assert!((({ sum_2(a.as_pointer(),) }) == 6));
        assert!((({ sum_2(b.as_pointer(),) }) == 106));
        let d: Value<Buffer> = Rc::new(RefCell::new(Buffer::Buffer_pconstBuffer({
            a.as_pointer()
        })));
        let _dtor_d = ScopedDestructor::new(&d, |__p| __p.destructor());
        assert!(
            ((*alive_0.with(Value::clone).borrow()) == 4)
                && ((*copies_1.with(Value::clone).borrow()) == 3)
        );
        assert!(
            ((*(*a.borrow()).size.borrow()) == 4)
                && ((*(*a.borrow()).data.borrow())[(3) as usize] == 3)
        );
        ({ BufferImpl::operator_assign(&d.as_pointer(), b.as_pointer()) });
        assert!(((*copies_1.with(Value::clone).borrow()) == 4));
        assert!(
            ((*(*b.borrow()).data.borrow())[(0) as usize] == 100)
                && ((*(*d.borrow()).data.borrow())[(0) as usize] == 100)
        );
    }
    assert!(((*alive_0.with(Value::clone).borrow()) == 0));
    return 0;
}
pub trait BufferImpl {
    fn destructor(&self);
    fn operator_assign(&self, o: Ptr<Buffer>) -> Ptr<Buffer>;
}
impl BufferImpl for Ptr<Buffer> {
    fn destructor(&self) {
        (*alive_0.with(Value::clone).borrow_mut()).prefix_dec();
    }
    fn operator_assign(&self, o: Ptr<Buffer>) -> Ptr<Buffer> {
        if ((*self) == (o)) {
            return (*self).clone();
        }
        let __rhs = (*(*o.upgrade().deref()).size.borrow());
        (*(*(*self).upgrade().deref()).size.borrow_mut()) = __rhs;
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 4) {
            let __rhs = (*(*o.upgrade().deref()).data.borrow())[(*i.borrow()) as usize];
            (*(*(*self).upgrade().deref()).data.borrow_mut())[(*i.borrow()) as usize] = __rhs;
            (*i.borrow_mut()).prefix_inc();
        }
        (*copies_1.with(Value::clone).borrow_mut()).prefix_inc();
        return (*self).clone();
    }
}
