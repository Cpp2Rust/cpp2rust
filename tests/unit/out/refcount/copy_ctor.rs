extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static copies_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Counted {
    #[offset(0)]
    pub v: i32,
}
impl Counted {
    pub fn new(mut v: i32) -> Self {
        Self { v: v }
    }
    pub fn copy_from(o: Ptr<Counted>) -> Self {
        let __this: Counted = Self {
            v: o.with(|__s| __s.v),
        };
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for Counted {
    fn clone(&self) -> Self {
        let __src: Value<Counted> = Rc::new(RefCell::new(Counted { v: self.v.clone() }));
        Counted::copy_from(__src.as_pointer())
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct NonConst {
    #[offset(0)]
    pub mark: i32,
}
impl NonConst {
    pub fn new() -> Self {
        Self { mark: 0 }
    }
    pub fn new_1(o: Ptr<NonConst>) -> Self {
        Self {
            mark: (o.with(|__s| __s.mark) + 1),
        }
    }
    pub fn new_2(o: Ptr<NonConst>) -> Self {
        Self {
            mark: (o.with(|__s| __s.mark) + 10),
        }
    }
}
impl Clone for NonConst {
    fn clone(&self) -> Self {
        let __src: Value<NonConst> = Rc::new(RefCell::new(NonConst {
            mark: self.mark.clone(),
        }));
        NonConst::new_1(__src.as_pointer())
    }
}
impl Default for NonConst {
    fn default() -> Self {
        { NonConst::new() }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Ignored {
    #[offset(0)]
    pub v: i32,
}
impl Ignored {
    pub fn new(mut v: i32) -> Self {
        Self { v: v }
    }
    pub fn copy_from(_a0: Ptr<Ignored>) -> Self {
        let __this: Ignored = Self { v: -1_i32 };
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for Ignored {
    fn clone(&self) -> Self {
        let __src: Value<Ignored> = Rc::new(RefCell::new(Ignored { v: self.v.clone() }));
        Ignored::copy_from(__src.as_pointer())
    }
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(4)]
    pub c: Counted,
    #[offset(4)]
    #[byte_size(8)]
    pub arr: Value<Box<[Counted]>>,
}
impl Default for Holder {
    fn default() -> Self {
        Holder {
            c: <Counted>::default(),
            arr: Rc::new(RefCell::new(
                (0..2)
                    .map(|_| <Counted>::default())
                    .collect::<Box<[Counted]>>(),
            )),
        }
    }
}
pub fn by_value_1(c: Counted) -> i32 {
    let c: Value<Counted> = Rc::new(RefCell::new(c));
    return { (*c.borrow()).v };
}
pub fn make_2(mut v: i32) -> Counted {
    let c: Value<Counted> = Rc::new(RefCell::new(Counted::new({ v })));
    return Counted::copy_from({ c.as_pointer() });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Counted> = Rc::new(RefCell::new(Counted::new({ 1 })));
    let b: Value<Counted> = Rc::new(RefCell::new(Counted::copy_from({ a.as_pointer() })));
    let c: Value<Counted> = Rc::new(RefCell::new(Counted::copy_from({ a.as_pointer() })));
    let d: Value<Counted> = Rc::new(RefCell::new(Counted::copy_from({ a.as_pointer() })));
    assert!((copies_0.with(|rc| *rc.borrow()) == 3));
    assert!(
        (({ (*b.borrow()).v } == 1) && ({ (*c.borrow()).v } == 1)) && ({ (*d.borrow()).v } == 1)
    );
    assert!((({ by_value_1(Counted::copy_from({ a.as_pointer() },),) }) == 1));
    assert!((copies_0.with(|rc| *rc.borrow()) == 4));
    let e: Value<Counted> = Rc::new(RefCell::new(({ make_2(5) })));
    assert!(({ (*e.borrow()).v } == 5));
    assert!((copies_0.with(|rc| *rc.borrow()) == 5));
    let f: Value<Counted> = Rc::new(RefCell::new(Counted::new({ 6 })));
    assert!(({ (*f.borrow()).v } == 6));
    assert!((copies_0.with(|rc| *rc.borrow()) == 5));
    let g: Value<Counted> = Rc::new(RefCell::new(Counted::new({ 7 })));
    let h: Value<Counted> = Rc::new(RefCell::new(Counted::copy_from({ g.as_pointer() })));
    assert!(({ (*h.borrow()).v } == 7));
    assert!((copies_0.with(|rc| *rc.borrow()) == 6));
    let hold: Value<Holder> = Rc::new(RefCell::new(Holder {
        c: Counted::new({ 8 }),
        arr: Rc::new(RefCell::new(Box::new([
            Counted::new({ 9 }),
            Counted::new({ 10 }),
        ]))),
    }));
    let hold2: Value<Holder> = Rc::new(RefCell::new((*hold.borrow()).clone()));
    assert!(
        (({ (*hold2.borrow()).c.v } == 8)
            && ({
                (*elem!(
                    (array_field_ptr!(hold2.as_pointer(), arr) as Ptr<Counted>),
                    0
                )
                .upgrade()
                .deref())
                .v
            } == 9))
            && ({
                (*elem!(
                    (array_field_ptr!(hold2.as_pointer(), arr) as Ptr<Counted>),
                    1
                )
                .upgrade()
                .deref())
                .v
            } == 10)
    );
    assert!((copies_0.with(|rc| *rc.borrow()) == 9));
    let vec_: Value<Vec<Counted>> = Rc::new(RefCell::new(Vec::new()));
    {
        let a0_clone = (*a.borrow()).clone();
        (*vec_.borrow_mut()).push(a0_clone)
    };
    assert!(
        ({
            (*elem!((vec_.as_pointer() as Ptr<Counted>), 0_usize)
                .upgrade()
                .deref())
            .v
        } == 1)
    );
    assert!((copies_0.with(|rc| *rc.borrow()) == 10));
    let i1: Value<Ignored> = Rc::new(RefCell::new(Ignored::new({ 1 })));
    let i2: Value<Ignored> = Rc::new(RefCell::new(Ignored::copy_from({ i1.as_pointer() })));
    assert!(({ (*i1.borrow()).v } == 1) && ({ (*i2.borrow()).v } == -1_i32));
    assert!((copies_0.with(|rc| *rc.borrow()) == 11));
    let n: Value<NonConst> = Rc::new(RefCell::new(NonConst::new()));
    let n1: Value<NonConst> = Rc::new(RefCell::new(NonConst::new_1({ n.as_pointer() })));
    let cn: Value<NonConst> = Rc::new(RefCell::new(NonConst::new()));
    let n2: Value<NonConst> = Rc::new(RefCell::new(NonConst::new_2({ cn.as_pointer() })));
    assert!(({ (*n1.borrow()).mark } == 1));
    assert!(({ (*n2.borrow()).mark } == 10));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = copies_0.with(|_| ());
}
