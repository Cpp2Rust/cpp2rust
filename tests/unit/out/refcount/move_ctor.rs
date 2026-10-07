extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(4)]
pub struct MoveOnly {
    #[offset(0)]
    pub v: i32,
}
impl MoveOnly {
    pub fn new(mut v: i32) -> Self {
        Self { v: v }
    }
    pub fn move_from(o: Ptr<MoveOnly>) -> Self {
        let __this: MoveOnly = Self {
            v: o.with(|__s| __s.v),
        };
        field!(o, v).write(0);
        __this
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(4)]
pub struct ConstMove {
    #[offset(0)]
    pub mark: i32,
}
impl ConstMove {
    pub fn new() -> Self {
        Self { mark: 0 }
    }
    pub fn new_1(o: Ptr<ConstMove>) -> Self {
        Self {
            mark: (o.with(|__s| __s.mark) + 1),
        }
    }
    pub fn new_2(o: Ptr<ConstMove>) -> Self {
        Self {
            mark: (o.with(|__s| __s.mark) + 10),
        }
    }
}
impl Default for ConstMove {
    fn default() -> Self {
        { ConstMove::new() }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct ThrowingMove {
    #[offset(0)]
    pub v: i32,
    #[offset(4)]
    pub copies: i32,
    #[offset(8)]
    pub moves: i32,
}
impl ThrowingMove {
    pub fn new(mut v: i32) -> Self {
        Self {
            v: v,
            copies: 0,
            moves: 0,
        }
    }
    pub fn copy_from(o: Ptr<ThrowingMove>) -> Self {
        Self {
            v: o.with(|__s| __s.v),
            copies: (o.with(|__s| __s.copies) + 1),
            moves: o.with(|__s| __s.moves),
        }
    }
    pub fn move_from(o: Ptr<ThrowingMove>) -> Self {
        let __this: ThrowingMove = Self {
            v: o.with(|__s| __s.v),
            copies: o.with(|__s| __s.copies),
            moves: (o.with(|__s| __s.moves) + 1),
        };
        field!(o, v).write(0);
        __this
    }
}
impl Clone for ThrowingMove {
    fn clone(&self) -> Self {
        let __src: Value<ThrowingMove> = Rc::new(RefCell::new(ThrowingMove {
            v: self.v.clone(),
            copies: self.copies.clone(),
            moves: self.moves.clone(),
        }));
        ThrowingMove::copy_from(__src.as_pointer())
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct NoexceptMove {
    #[offset(0)]
    pub v: i32,
    #[offset(4)]
    pub copies: i32,
    #[offset(8)]
    pub moves: i32,
}
impl NoexceptMove {
    pub fn new(mut v: i32) -> Self {
        Self {
            v: v,
            copies: 0,
            moves: 0,
        }
    }
    pub fn copy_from(o: Ptr<NoexceptMove>) -> Self {
        Self {
            v: o.with(|__s| __s.v),
            copies: (o.with(|__s| __s.copies) + 1),
            moves: o.with(|__s| __s.moves),
        }
    }
    pub fn move_from(o: Ptr<NoexceptMove>) -> Self {
        let __this: NoexceptMove = Self {
            v: o.with(|__s| __s.v),
            copies: o.with(|__s| __s.copies),
            moves: (o.with(|__s| __s.moves) + 1),
        };
        field!(o, v).write(0);
        __this
    }
}
impl Clone for NoexceptMove {
    fn clone(&self) -> Self {
        let __src: Value<NoexceptMove> = Rc::new(RefCell::new(NoexceptMove {
            v: self.v.clone(),
            copies: self.copies.clone(),
            moves: self.moves.clone(),
        }));
        NoexceptMove::copy_from(__src.as_pointer())
    }
}
pub fn by_value_0(mut m: MoveOnly) -> i32 {
    return m.v;
}
pub fn make_1(mut v: i32) -> MoveOnly {
    let m: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ v })));
    return MoveOnly::move_from({ m.as_pointer() });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ 1 })));
    let b: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::move_from({ a.as_pointer() })));
    assert!(({ (*b.borrow()).v } == 1));
    assert!(({ (*a.borrow()).v } == 0));
    let c: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::move_from({ b.as_pointer() })));
    assert!(({ (*c.borrow()).v } == 1));
    assert!(({ (*b.borrow()).v } == 0));
    let mut d: MoveOnly = MoveOnly::move_from({ c.as_pointer() });
    assert!((d.v == 1));
    assert!(({ (*c.borrow()).v } == 0));
    let e: Value<MoveOnly> = Rc::new(RefCell::new(({ make_1(5) })));
    assert!(({ (*e.borrow()).v } == 5));
    assert!((({ by_value_0(MoveOnly::new({ 6 },),) }) == 6));
    assert!((({ by_value_0(MoveOnly::move_from({ e.as_pointer() },),) }) == 5));
    assert!(({ (*e.borrow()).v } == 0));
    let mut vec_: Vec<MoveOnly> = Vec::new();
    {
        let __a1 = MoveOnly::new({ 7 });
        vec_.push(__a1)
    };
    let f: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ 8 })));
    {
        let __a1 = MoveOnly::move_from({ f.as_pointer() });
        vec_.push(__a1)
    };
    assert!(({ vec_[0_usize].v } == 7) && ({ vec_[1_usize].v } == 8));
    assert!(({ (*f.borrow()).v } == 0));
    let m: Value<ConstMove> = Rc::new(RefCell::new(ConstMove::new()));
    let mut m1: ConstMove = ConstMove::new_1({ m.as_pointer() });
    let cm: Value<ConstMove> = Rc::new(RefCell::new(ConstMove::new()));
    let mut m2: ConstMove = ConstMove::new_2({ cm.as_pointer() });
    assert!((m1.mark == 1));
    assert!((m2.mark == 10));
    let t: Value<ThrowingMove> = Rc::new(RefCell::new(ThrowingMove::new({ 1 })));
    let mut t1: ThrowingMove = ThrowingMove::copy_from({ t.as_pointer() });
    assert!((t1.v == 1));
    assert!((t1.copies == 1));
    assert!((t1.moves == 0));
    assert!(({ (*t.borrow()).v } == 1));
    let n: Value<NoexceptMove> = Rc::new(RefCell::new(NoexceptMove::new({ 2 })));
    let mut n1: NoexceptMove = NoexceptMove::move_from({ n.as_pointer() });
    assert!((n1.v == 2));
    assert!((n1.copies == 0));
    assert!((n1.moves == 1));
    assert!(({ (*n.borrow()).v } == 0));
    let g: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ 3 })));
    let mut g1: MoveOnly = MoveOnly::move_from({ g.as_pointer() });
    assert!((g1.v == 3));
    assert!(({ (*g.borrow()).v } == 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
