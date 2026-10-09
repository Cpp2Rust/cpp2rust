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
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(4)]
pub struct Box_int_ {
    #[offset(0)]
    pub val: i32,
}
impl Box_int_ {
    pub fn new(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        Self { val: (*v.borrow()) }
    }
    pub fn copy_from(o: Ptr<Box_int_>) -> Self {
        Self {
            val: o.with(|__s| __s.val),
        }
    }
    pub fn move_from(_a0: Ptr<Box_int_>) -> Self {
        Self {
            val: { (*_a0.upgrade().deref()).val },
        }
    }
}
impl Clone for Box_int_ {
    fn clone(&self) -> Self {
        let __src: Value<Box_int_> = Rc::new(RefCell::new(Box_int_ {
            val: self.val.clone(),
        }));
        Box_int_::copy_from(__src.as_pointer())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Owner {
    #[offset(0)]
    #[byte_size(4)]
    pub box_: Box_int_,
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct OutOfLine {
    #[offset(0)]
    pub v: i32,
}
impl OutOfLine {
    pub fn new(mut v: i32) -> Self {
        Self { v: v }
    }
    pub fn copy_from(o: Ptr<OutOfLine>) -> Self {
        let __this: OutOfLine = Self {
            v: (o.with(|__s| __s.v) + 100),
        };
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for OutOfLine {
    fn clone(&self) -> Self {
        let __src: Value<OutOfLine> = Rc::new(RefCell::new(OutOfLine { v: self.v.clone() }));
        OutOfLine::copy_from(__src.as_pointer())
    }
}
impl OutOfLine {}
pub fn by_value_1(mut c: Counted) -> i32 {
    return c.v;
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
    let mut b: Counted = Counted::copy_from({ a.as_pointer() });
    let mut c: Counted = Counted::copy_from({ a.as_pointer() });
    let mut d: Counted = Counted::copy_from({ a.as_pointer() });
    assert!((copies_0.with(|rc| *rc.borrow()) == 3));
    assert!(((b.v == 1) && (c.v == 1)) && (d.v == 1));
    assert!((({ by_value_1(Counted::copy_from({ a.as_pointer() },),) }) == 1));
    assert!((copies_0.with(|rc| *rc.borrow()) == 4));
    let mut e: Counted = ({ make_2(5) });
    assert!((e.v == 5));
    assert!((copies_0.with(|rc| *rc.borrow()) == 5));
    let mut f: Counted = Counted::new({ 6 });
    assert!((f.v == 6));
    assert!((copies_0.with(|rc| *rc.borrow()) == 5));
    let g: Value<Counted> = Rc::new(RefCell::new(Counted::new({ 7 })));
    let mut h: Counted = Counted::copy_from({ g.as_pointer() });
    assert!((h.v == 7));
    assert!((copies_0.with(|rc| *rc.borrow()) == 6));
    let hold: Value<Holder> = Rc::new(RefCell::new(Holder {
        c: Counted::new({ 8 }),
        arr: Rc::new(RefCell::new(Box::new([
            Counted::new({ 9 }),
            Counted::new({ 10 }),
        ]))),
    }));
    let mut hold2: Holder = (*hold.borrow()).clone();
    assert!(
        ((hold2.c.v == 8)
            && ({
                (*elem!((hold2.arr.as_pointer() as Ptr<Counted>), 0)
                    .upgrade()
                    .deref())
                .v
            } == 9))
            && ({
                (*elem!((hold2.arr.as_pointer() as Ptr<Counted>), 1)
                    .upgrade()
                    .deref())
                .v
            } == 10)
    );
    assert!((copies_0.with(|rc| *rc.borrow()) == 9));
    let mut vec_: Vec<Counted> = Vec::new();
    {
        let a0_clone = (*a.borrow()).clone();
        vec_.push(a0_clone)
    };
    assert!(({ vec_[0_usize].v } == 1));
    assert!((copies_0.with(|rc| *rc.borrow()) == 10));
    let i1: Value<Ignored> = Rc::new(RefCell::new(Ignored::new({ 1 })));
    let mut i2: Ignored = Ignored::copy_from({ i1.as_pointer() });
    assert!(({ (*i1.borrow()).v } == 1) && (i2.v == -1_i32));
    assert!((copies_0.with(|rc| *rc.borrow()) == 11));
    let o: Value<Owner> = Rc::new(RefCell::new(Owner {
        box_: Box_int_::new({ 3 }),
    }));
    let mut o2: Owner = (*o.borrow()).clone();
    assert!((o2.box_.val == 3));
    let ol1: Value<OutOfLine> = Rc::new(RefCell::new(OutOfLine::new({ 5 })));
    let mut ol2: OutOfLine = OutOfLine::copy_from({ ol1.as_pointer() });
    assert!((ol2.v == 105));
    assert!((copies_0.with(|rc| *rc.borrow()) == 12));
    let n: Value<NonConst> = Rc::new(RefCell::new(NonConst::new()));
    let mut n1: NonConst = NonConst::new_1({ n.as_pointer() });
    let cn: Value<NonConst> = Rc::new(RefCell::new(NonConst::new()));
    let mut n2: NonConst = NonConst::new_2({ cn.as_pointer() });
    assert!((n1.mark == 1));
    assert!((n2.mark == 10));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = copies_0.with(|_| ());
}
