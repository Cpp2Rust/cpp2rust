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
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(20)]
pub struct Buffer {
    #[offset(0)]
    #[byte_size(16)]
    pub data: Value<Box<[i32]>>,
    #[offset(16)]
    pub size: i32,
}
impl Buffer {
    pub fn new(mut size: i32) -> Self {
        let __this: Value<Buffer> = Rc::new(RefCell::new(Self {
            data: Rc::new(RefCell::new((0..4).map(|_| 0_i32).collect::<Box<[i32]>>())),
            size: size,
        }));
        let this: Ptr<Buffer> = __this.as_pointer();
        let mut i: i32 = 0;
        'loop_: while (i < 4) {
            elem!((array_field_ptr!(this, data) as Ptr::<i32>), i)
                .write({ if (i < size) { i } else { -1_i32 } });
            i.prefix_inc();
        }
        (*alive_0.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn copy_from(o: Ptr<Buffer>) -> Self {
        let __this: Value<Buffer> = Rc::new(RefCell::new(Self {
            data: Rc::new(RefCell::new((0..4).map(|_| 0_i32).collect::<Box<[i32]>>())),
            size: o.with(|__s| __s.size),
        }));
        let this: Ptr<Buffer> = __this.as_pointer();
        let mut i: i32 = 0;
        'loop_: while (i < 4) {
            elem!((array_field_ptr!(this, data) as Ptr::<i32>), i)
                .write({ (elem!((array_field_ptr!(o, data) as Ptr::<i32>), i).read()) });
            i.prefix_inc();
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
        Buffer::copy_from(__src.as_pointer())
    }
}
impl Default for Buffer {
    fn default() -> Self {
        Buffer {
            data: Rc::new(RefCell::new((0..4).map(|_| 0_i32).collect::<Box<[i32]>>())),
            size: 0_i32,
        }
    }
}
pub fn sum_2(b: Ptr<Buffer>) -> i32 {
    let mut s: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ({ i } < { b.with(|__s| __s.size) }) {
        s += (elem!((array_field_ptr!(b, data) as Ptr::<i32>), i).read());
        i.prefix_inc();
    }
    return s;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        let a: Value<Buffer> = Rc::new(RefCell::new(Buffer::new({ 4 })));
        let _dtor_a = ScopedDestructor::new(&a, |__p| __p.destructor());
        let b: Value<Buffer> = Rc::new(RefCell::new(Buffer::copy_from({ a.as_pointer() })));
        let _dtor_b = ScopedDestructor::new(&b, |__p| __p.destructor());
        assert!((alive_0.with(|rc| *rc.borrow()) == 2) && (copies_1.with(|rc| *rc.borrow()) == 1));
        elem!((array_field_ptr!(b.as_pointer(), data) as Ptr::<i32>), 0).write(100);
        assert!(((elem!((array_field_ptr!(a.as_pointer(), data) as Ptr::<i32>), 0).read()) == 0));
        let c: Value<Buffer> = Rc::new(RefCell::new(Buffer::new({ 2 })));
        let _dtor_c = ScopedDestructor::new(&c, |__p| __p.destructor());
        ({ BufferImpl::copy_assign(&c.as_pointer(), a.as_pointer()) });
        assert!(
            ({ (*c.borrow()).size } == 4)
                && ((elem!((array_field_ptr!(c.as_pointer(), data) as Ptr::<i32>), 3).read()) == 3)
        );
        assert!((alive_0.with(|rc| *rc.borrow()) == 3) && (copies_1.with(|rc| *rc.borrow()) == 2));
        ({
            let _o: Ptr<Buffer> = c.as_pointer();
            BufferImpl::copy_assign(&c.as_pointer(), _o)
        });
        assert!((copies_1.with(|rc| *rc.borrow()) == 2));
        assert!((({ sum_2(a.as_pointer(),) }) == 6));
        assert!((({ sum_2(b.as_pointer(),) }) == 106));
        let d: Value<Buffer> = Rc::new(RefCell::new(Buffer::copy_from({ a.as_pointer() })));
        let _dtor_d = ScopedDestructor::new(&d, |__p| __p.destructor());
        assert!((alive_0.with(|rc| *rc.borrow()) == 4) && (copies_1.with(|rc| *rc.borrow()) == 3));
        assert!(
            ({ (*a.borrow()).size } == 4)
                && ((elem!((array_field_ptr!(a.as_pointer(), data) as Ptr::<i32>), 3).read()) == 3)
        );
        ({ BufferImpl::copy_assign(&d.as_pointer(), b.as_pointer()) });
        assert!((copies_1.with(|rc| *rc.borrow()) == 4));
        assert!(
            ((elem!((array_field_ptr!(b.as_pointer(), data) as Ptr::<i32>), 0).read()) == 100)
                && ((elem!((array_field_ptr!(d.as_pointer(), data) as Ptr::<i32>), 0).read())
                    == 100)
        );
    }
    assert!((alive_0.with(|rc| *rc.borrow()) == 0));
    return 0;
}
pub trait BufferImpl {
    fn destructor(&self);
    fn copy_assign(&self, o: Ptr<Buffer>) -> Ptr<Buffer>;
}
impl BufferImpl for Ptr<Buffer> {
    fn destructor(&self) {
        (*alive_0.with(Value::clone).borrow_mut()).prefix_dec();
    }
    fn copy_assign(&self, o: Ptr<Buffer>) -> Ptr<Buffer> {
        if ((*self) == (o)) {
            return (*self).clone();
        }
        field!((*self), size).write({ o.with(|__s| __s.size) });
        let mut i: i32 = 0;
        'loop_: while (i < 4) {
            elem!((array_field_ptr!((*self), data) as Ptr::<i32>), i)
                .write({ (elem!((array_field_ptr!(o, data) as Ptr::<i32>), i).read()) });
            i.prefix_inc();
        }
        (*copies_1.with(Value::clone).borrow_mut()).prefix_inc();
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = alive_0.with(|_| ());
    let _ = copies_1.with(|_| ());
}
