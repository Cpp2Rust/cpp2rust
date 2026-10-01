extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(1));
    let b: Value<i32> = Rc::new(RefCell::new(2));
    let c: Value<i32> = Rc::new(RefCell::new(3));
    let by_value: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new({
        #[derive(Record, ByteRepr)]
        #[byte_size(12)]
        struct Captures {
            #[offset(0)]
            a: i32,
            #[offset(4)]
            b: i32,
            #[offset(8)]
            c: i32,
        }
        FnPtr::<fn(i32) -> i32>::with_captures(
            Captures {
                a: (*a.borrow()),
                b: (*b.borrow()),
                c: (*c.borrow()),
            },
            (|this: Ptr<Captures>, x: i32| {
                let x: Value<i32> = Rc::new(RefCell::new(x));
                return (((this.with(|__s| __s.a) + this.with(|__s| __s.b))
                    + this.with(|__s| __s.c))
                    + (*x.borrow()));
            }),
        )
    }));
    assert!((({ (*by_value.borrow()).call(10,) }) == 16));
    (*a.borrow_mut()) = 100;
    assert!((({ (*by_value.borrow()).call(10,) }) == 16));
    let by_ref: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new({
        #[derive(Record, ByteRepr)]
        #[byte_size(24)]
        struct Captures {
            #[offset(0)]
            #[byte_size(8)]
            a: Ptr<i32>,
            #[offset(8)]
            #[byte_size(8)]
            b: Ptr<i32>,
            #[offset(16)]
            #[byte_size(8)]
            c: Ptr<i32>,
        }
        FnPtr::<fn(i32) -> i32>::with_captures(
            Captures {
                a: a.as_pointer(),
                b: b.as_pointer(),
                c: c.as_pointer(),
            },
            (|this: Ptr<Captures>, x: i32| {
                let x: Value<i32> = Rc::new(RefCell::new(x));
                return ((((this.with(|__s| __s.a.clone()).read())
                    + (this.with(|__s| __s.b.clone()).read()))
                    + (this.with(|__s| __s.c.clone()).read()))
                    + (*x.borrow()));
            }),
        )
    }));
    assert!((({ (*by_ref.borrow()).call(10,) }) == 115));
    (*b.borrow_mut()) = 200;
    assert!((({ (*by_ref.borrow()).call(10,) }) == 313));
    let mixed: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new({
        #[derive(Record, ByteRepr)]
        #[byte_size(16)]
        struct Captures {
            #[offset(0)]
            #[byte_size(8)]
            c: Ptr<i32>,
            #[offset(8)]
            a: i32,
            #[offset(12)]
            b: i32,
        }
        FnPtr::<fn(i32) -> i32>::with_captures(
            Captures {
                c: c.as_pointer(),
                a: (*a.borrow()),
                b: (*b.borrow()),
            },
            (|this: Ptr<Captures>, x: i32| {
                let x: Value<i32> = Rc::new(RefCell::new(x));
                {
                    let _ptr = this.with(|__s| __s.c.clone()).clone();
                    _ptr.write(_ptr.read() + (*x.borrow()))
                };
                return ((this.with(|__s| __s.a) + this.with(|__s| __s.b))
                    + (this.with(|__s| __s.c.clone()).read()));
            }),
        )
    }));
    assert!((({ (*mixed.borrow()).call(1,) }) == ((100 + 200) + 4)));
    assert!(((*c.borrow()) == 4));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
