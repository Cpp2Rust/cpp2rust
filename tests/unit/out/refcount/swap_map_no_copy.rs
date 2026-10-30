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
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CppFrom, CopyAssign, MoveAssign)]
#[cpp_from(i32, new_1, value)]
#[byte_size(4)]
pub struct Tracked {
    #[offset(0)]
    pub v: i32,
}
impl Tracked {
    pub fn new_2() -> Self {
        Self { v: 0 }
    }
    pub fn new_1(mut v: i32) -> Self {
        Self { v: v }
    }
    pub fn copy_from(o: Ptr<Tracked>) -> Self {
        let __this: Tracked = Self {
            v: o.with(|__s| __s.v),
        };
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn move_from(o: Ptr<Tracked>) -> Self {
        Self {
            v: o.with(|__s| __s.v),
        }
    }
}
impl Clone for Tracked {
    fn clone(&self) -> Self {
        let __src: Value<Tracked> = Rc::new(RefCell::new(Tracked { v: self.v.clone() }));
        Tracked::copy_from(__src.as_pointer())
    }
}
impl Default for Tracked {
    fn default() -> Self {
        { Tracked::new_2() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign, MoveAssign, Default)]
#[byte_size(8)]
pub struct Pair {
    #[offset(0)]
    #[byte_size(4)]
    pub x: Tracked,
    #[offset(4)]
    #[byte_size(4)]
    pub y: Tracked,
}
impl Pair {
    pub fn move_from(_a0: Ptr<Pair>) -> Self {
        Self {
            x: Tracked::move_from({ field_ptr!(_a0, x) }),
            y: Tracked::move_from({ field_ptr!(_a0, y) }),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign, MoveAssign)]
#[byte_size(16)]
pub struct Arrays {
    #[offset(0)]
    #[byte_size(8)]
    pub a: Box<[Tracked]>,
    #[offset(8)]
    #[byte_size(8)]
    pub b: Box<[Tracked]>,
}
impl Arrays {
    pub fn move_from(_a0: Ptr<Arrays>) -> Self {
        Self {
            a: Box::new(std::array::from_fn::<_, 2, _>(|__i: usize| {
                Tracked::move_from({ (array_field_ptr!(_a0, a) as Ptr<Tracked>).offset(__i) })
            })),
            b: Box::new(std::array::from_fn::<_, 2, _>(|__i: usize| {
                Tracked::move_from({ (array_field_ptr!(_a0, b) as Ptr<Tracked>).offset(__i) })
            })),
        }
    }
}
impl Default for Arrays {
    fn default() -> Self {
        {
            Arrays {
                a: (0..2)
                    .map(|_| <Tracked>::default())
                    .collect::<Box<[Tracked]>>(),
                b: (0..2)
                    .map(|_| <Tracked>::default())
                    .collect::<Box<[Tracked]>>(),
            }
        }
    }
}
thread_local!(
    pub static copy_only_copies_1: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static copy_only_dtors_2: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CppFrom, CopyAssign, Destructor, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct CopyOnly {
    #[offset(0)]
    pub v: i32,
}
impl CopyOnly {
    pub fn new(mut v: i32) -> Self {
        Self { v: v }
    }
    pub fn copy_from(o: Ptr<CopyOnly>) -> Self {
        let __this: CopyOnly = Self {
            v: o.with(|__s| __s.v),
        };
        (*copy_only_copies_1.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for CopyOnly {
    fn clone(&self) -> Self {
        let __src: Value<CopyOnly> = Rc::new(RefCell::new(CopyOnly { v: self.v.clone() }));
        CopyOnly::copy_from(__src.as_pointer())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor, Default)]
#[byte_size(8)]
pub struct CopyOnlyPair {
    #[offset(0)]
    #[byte_size(4)]
    pub x: CopyOnly,
    #[offset(4)]
    #[byte_size(4)]
    pub y: CopyOnly,
}
thread_local!(
    pub static moves_3: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static move_assigns_4: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static moved_dtors_5: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(
    Record,
    ByteRepr,
    VaArg,
    FnPtrArg,
    MoveCtor,
    CppFrom,
    CopyAssign,
    MoveAssign,
    Destructor,
    Default,
)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct Moved {
    #[offset(0)]
    pub v: i32,
}
impl Moved {
    pub fn new(mut v: i32) -> Self {
        Self { v: v }
    }
    pub fn copy_from(o: Ptr<Moved>) -> Self {
        let __this: Moved = Self {
            v: o.with(|__s| __s.v),
        };
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn move_from(o: Ptr<Moved>) -> Self {
        let __this: Moved = Self {
            v: o.with(|__s| __s.v),
        };
        field!(o, v).write(-1_i32);
        (*moves_3.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for Moved {
    fn clone(&self) -> Self {
        let __src: Value<Moved> = Rc::new(RefCell::new(Moved { v: self.v.clone() }));
        Moved::copy_from(__src.as_pointer())
    }
}
#[derive(
    Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign, MoveAssign, Destructor, Default,
)]
#[byte_size(8)]
pub struct MovedPair {
    #[offset(0)]
    #[byte_size(4)]
    pub x: Moved,
    #[offset(4)]
    #[byte_size(4)]
    pub y: Moved,
}
impl MovedPair {
    pub fn move_from(_a0: Ptr<MovedPair>) -> Self {
        Self {
            x: Moved::move_from({ field_ptr!(_a0, x) }),
            y: Moved::move_from({ field_ptr!(_a0, y) }),
        }
    }
}
pub fn swapped_once_6() -> bool {
    let mut ok: bool = ((moves_3.with(|rc| *rc.borrow()) == 1)
        && (move_assigns_4.with(|rc| *rc.borrow()) == 2))
        && (moved_dtors_5.with(|rc| *rc.borrow()) == 1);
    moves_3.with(|rc| {
        *rc.borrow_mut() = {
            move_assigns_4.with(|rc| {
                *rc.borrow_mut() = {
                    moved_dtors_5.with(|rc| *rc.borrow_mut() = 0);
                    moved_dtors_5.with(|rc| *rc.borrow())
                }
            });
            move_assigns_4.with(|rc| *rc.borrow())
        }
    });
    return ok;
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(8)]
pub struct Owner {
    #[offset(0)]
    #[byte_size(8)]
    pub p: Option<Value<i32>>,
}
impl Owner {
    pub fn move_from(_a0: Ptr<Owner>) -> Self {
        Self {
            p: field!(_a0, p).with_mut(|__v: &mut Option<Value<i32>>| __v.take()),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Tracked> = Rc::new(RefCell::new(Tracked::new_1({ 1 })));
    let b: Value<Tracked> = Rc::new(RefCell::new(Tracked::new_1({ 2 })));
    cpp_swap!(a.as_pointer(), b.as_pointer());
    assert!(({ (*a.borrow()).v } == 2) && ({ (*b.borrow()).v } == 1));
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    let m1: Value<Map<i32, (Value<i32>, Value<Tracked>)>> = Rc::new(RefCell::new(Map::new()));
    ({
        let _o: Value<Tracked> = Rc::new(RefCell::new(Tracked::new_1({ 10 })));
        TrackedImpl::move_assign(
            &Map::subscript_move(
                m1.as_pointer(),
                Rc::new(RefCell::new(1)).as_pointer(),
                |__k| cpp_move!(__k),
            ),
            _o.as_pointer(),
        )
    });
    ({
        let _o: Value<Tracked> = Rc::new(RefCell::new(Tracked::new_1({ 20 })));
        TrackedImpl::move_assign(
            &Map::subscript_move(
                m1.as_pointer(),
                Rc::new(RefCell::new(2)).as_pointer(),
                |__k| cpp_move!(__k),
            ),
            _o.as_pointer(),
        )
    });
    let m2: Value<Map<i32, (Value<i32>, Value<Tracked>)>> = Rc::new(RefCell::new(Map::new()));
    ({
        let _o: Value<Tracked> = Rc::new(RefCell::new(Tracked::new_1({ 30 })));
        TrackedImpl::move_assign(
            &Map::subscript_move(
                m2.as_pointer(),
                Rc::new(RefCell::new(3)).as_pointer(),
                |__k| cpp_move!(__k),
            ),
            _o.as_pointer(),
        )
    });
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    cpp_swap!(
        Map::subscript_move(
            m1.as_pointer(),
            Rc::new(RefCell::new(1)).as_pointer(),
            |__k| cpp_move!(__k)
        ),
        Map::subscript_move(
            m1.as_pointer(),
            Rc::new(RefCell::new(2)).as_pointer(),
            |__k| cpp_move!(__k)
        )
    );
    assert!(
        (Map::subscript_move(
            m1.as_pointer(),
            Rc::new(RefCell::new(1)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .with(|__s| __s.v)
            == 20)
            && (Map::subscript_move(
                m1.as_pointer(),
                Rc::new(RefCell::new(2)).as_pointer(),
                |__k| cpp_move!(__k)
            )
            .with(|__s| __s.v)
                == 10)
    );
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    cpp_swap!(m1.as_pointer(), m2.as_pointer());
    assert!(
        ((*m1.borrow()).len() == 1_usize)
            && (Map::subscript_move(
                m1.as_pointer(),
                Rc::new(RefCell::new(3)).as_pointer(),
                |__k| cpp_move!(__k)
            )
            .with(|__s| __s.v)
                == 30)
    );
    assert!(
        (((*m2.borrow()).len() == 2_usize)
            && (Map::subscript_move(
                m2.as_pointer(),
                Rc::new(RefCell::new(1)).as_pointer(),
                |__k| cpp_move!(__k)
            )
            .with(|__s| __s.v)
                == 20))
            && (Map::subscript_move(
                m2.as_pointer(),
                Rc::new(RefCell::new(2)).as_pointer(),
                |__k| cpp_move!(__k)
            )
            .with(|__s| __s.v)
                == 10)
    );
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    let arr: Value<Box<[Tracked]>> = Rc::new(RefCell::new(Box::new([
        Tracked::new_1({ 1 }),
        Tracked::new_1({ 2 }),
        Tracked::new_1({ 3 }),
    ])));
    cpp_swap!(
        (arr.as_pointer() as Ptr<Tracked>).offset(0),
        (arr.as_pointer() as Ptr<Tracked>).offset(2)
    );
    assert!(
        ({ (*arr.borrow())[(0) as usize].v } == 3) && ({ (*arr.borrow())[(2) as usize].v } == 1)
    );
    cpp_swap!(
        (arr.as_pointer() as Ptr<Tracked>).offset(1),
        (arr.as_pointer() as Ptr<Tracked>).offset(1)
    );
    assert!(({ (*arr.borrow())[(1) as usize].v } == 2));
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    let vec_: Value<Vec<Tracked>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __init = Tracked::new_1({ 4 });
        (*vec_.borrow_mut()).push(__init)
    };
    {
        let __init = Tracked::new_1({ 5 });
        (*vec_.borrow_mut()).push(__init)
    };
    cpp_swap!(vec_.decay().offset(0_usize), vec_.decay().offset(1_usize));
    assert!(({ (*vec_.borrow())[0_usize].v } == 5) && ({ (*vec_.borrow())[1_usize].v } == 4));
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    let pairs: Value<Box<[Pair]>> = Rc::new(RefCell::new(Box::new([
        Pair {
            x: Tracked::new_1({ 6 }),
            y: Tracked::new_1({ 7 }),
        },
        Pair {
            x: Tracked::new_1({ 8 }),
            y: Tracked::new_1({ 9 }),
        },
    ])));
    let p: Value<Pair> = Rc::new(RefCell::new(Pair {
        x: Tracked::new_1({ 10 }),
        y: Tracked::new_1({ 11 }),
    }));
    cpp_swap!(
        field_ptr!((pairs.as_pointer() as Ptr<Pair>).offset(0), x),
        field_ptr!(p.as_pointer(), y)
    );
    assert!(({ (*pairs.borrow())[(0) as usize].x.v } == 11) && ({ (*p.borrow()).y.v } == 6));
    cpp_swap!(
        (pairs.as_pointer() as Ptr<Pair>).offset(0),
        (pairs.as_pointer() as Ptr<Pair>).offset(1)
    );
    assert!(
        ({ (*pairs.borrow())[(0) as usize].x.v } == 8)
            && ({ (*pairs.borrow())[(1) as usize].x.v } == 11)
    );
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    cpp_swap!(field_ptr!(p.as_pointer(), x), field_ptr!(p.as_pointer(), y));
    assert!(({ (*p.borrow()).x.v } == 6) && ({ (*p.borrow()).y.v } == 10));
    cpp_swap!(
        field_ptr!((pairs.as_pointer() as Ptr<Pair>).offset(0), x),
        field_ptr!((pairs.as_pointer() as Ptr<Pair>).offset(1), y)
    );
    assert!(
        ({ (*pairs.borrow())[(0) as usize].x.v } == 7)
            && ({ (*pairs.borrow())[(1) as usize].y.v } == 8)
    );
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    let arrays: Value<Arrays> = Rc::new(RefCell::new(<Arrays>::default()));
    field!(array_elem!(arrays.as_pointer(), a, 0), v).write(1);
    field!(array_elem!(arrays.as_pointer(), b, 1), v).write(2);
    cpp_swap!(
        (array_field_ptr!(arrays.as_pointer(), a) as Ptr<Tracked>).offset((0) as isize),
        (array_field_ptr!(arrays.as_pointer(), b) as Ptr<Tracked>).offset((1) as isize)
    );
    assert!(
        (array_elem!(arrays.as_pointer(), a, 0).with(|__s| __s.v) == 2)
            && (array_elem!(arrays.as_pointer(), b, 1).with(|__s| __s.v) == 1)
    );
    cpp_swap!(
        (array_field_ptr!(arrays.as_pointer(), a) as Ptr<Tracked>).offset((0) as isize),
        (array_field_ptr!(arrays.as_pointer(), a) as Ptr<Tracked>).offset((1) as isize)
    );
    assert!(
        (array_elem!(arrays.as_pointer(), a, 0).with(|__s| __s.v) == 0)
            && (array_elem!(arrays.as_pointer(), a, 1).with(|__s| __s.v) == 2)
    );
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    let grid: Value<Vec<Vec<Tracked>>> = Rc::new(RefCell::new(
        (0..(2_usize) as usize)
            .map(|_| Default::default())
            .collect::<Vec<_>>(),
    ));
    {
        let __init = Tracked::new_1({ 1 });
        elem!(grid.decay(), 0_usize).with_mut(|__v: &mut Vec<Tracked>| __v.push(__init))
    };
    {
        let __init = Tracked::new_1({ 2 });
        elem!(grid.decay(), 0_usize).with_mut(|__v: &mut Vec<Tracked>| __v.push(__init))
    };
    {
        let __init = Tracked::new_1({ 3 });
        elem!(grid.decay(), 1_usize).with_mut(|__v: &mut Vec<Tracked>| __v.push(__init))
    };
    cpp_swap!(
        grid.decay().offset(0_usize).decay().offset(1_usize),
        grid.decay().offset(1_usize).decay().offset(0_usize)
    );
    assert!(
        (index!(elem!(grid.decay(), 0_usize), 1_usize).with(|__s| __s.v) == 3)
            && (index!(elem!(grid.decay(), 1_usize), 0_usize).with(|__s| __s.v) == 2)
    );
    cpp_swap!(grid.decay().offset(0_usize), grid.decay().offset(1_usize));
    assert!(
        ((*grid.decay().offset(0_usize).upgrade().deref()).len() == 1_usize)
            && (index!(elem!(grid.decay(), 0_usize), 0_usize).with(|__s| __s.v) == 2)
    );
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    let maps: Value<Box<[Map<i32, (Value<i32>, Value<Tracked>)>]>> = Rc::new(RefCell::new(
        (0..2)
            .map(|_| Map::new())
            .collect::<Box<[Map<i32, (Value<i32>, Value<Tracked>)>]>>(),
    ));
    ({
        let _o: Value<Tracked> = Rc::new(RefCell::new(Tracked::new_1({ 1 })));
        TrackedImpl::move_assign(
            &Map::subscript_move(
                (maps.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<Tracked>)>>).offset(0),
                Rc::new(RefCell::new(1)).as_pointer(),
                |__k| cpp_move!(__k),
            ),
            _o.as_pointer(),
        )
    });
    ({
        let _o: Value<Tracked> = Rc::new(RefCell::new(Tracked::new_1({ 2 })));
        TrackedImpl::move_assign(
            &Map::subscript_move(
                (maps.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<Tracked>)>>).offset(1),
                Rc::new(RefCell::new(2)).as_pointer(),
                |__k| cpp_move!(__k),
            ),
            _o.as_pointer(),
        )
    });
    cpp_swap!(
        (maps.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<Tracked>)>>).offset(0),
        (maps.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<Tracked>)>>).offset(1)
    );
    assert!(
        ((*maps.borrow())[(0) as usize].len() == 1_usize)
            && (Map::subscript_move(
                (maps.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<Tracked>)>>).offset(0),
                Rc::new(RefCell::new(2)).as_pointer(),
                |__k| cpp_move!(__k)
            )
            .with(|__s| __s.v)
                == 2)
    );
    assert!(
        ((*maps.borrow())[(1) as usize].len() == 1_usize)
            && (Map::subscript_move(
                (maps.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<Tracked>)>>).offset(1),
                Rc::new(RefCell::new(1)).as_pointer(),
                |__k| cpp_move!(__k)
            )
            .with(|__s| __s.v)
                == 1)
    );
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    let c1: Value<CopyOnly> = Rc::new(RefCell::new(CopyOnly::new({ 1 })));
    let _dtor_c1 = ScopedDestructor::new(&c1, |__p| __p.destructor());
    let c2: Value<CopyOnly> = Rc::new(RefCell::new(CopyOnly::new({ 2 })));
    let _dtor_c2 = ScopedDestructor::new(&c2, |__p| __p.destructor());
    cpp_swap!(c1.as_pointer(), c2.as_pointer());
    assert!(({ (*c1.borrow()).v } == 2) && ({ (*c2.borrow()).v } == 1));
    assert!(
        (copy_only_copies_1.with(|rc| *rc.borrow()) == 3)
            && (copy_only_dtors_2.with(|rc| *rc.borrow()) == 1)
    );
    let cp: Value<CopyOnlyPair> = Rc::new(RefCell::new(CopyOnlyPair {
        x: CopyOnly::new({ 3 }),
        y: CopyOnly::new({ 4 }),
    }));
    let _dtor_cp = ScopedDestructor::new(&cp, |__p| __p.destructor());
    cpp_swap!(
        field_ptr!(cp.as_pointer(), x),
        field_ptr!(cp.as_pointer(), y)
    );
    assert!(({ (*cp.borrow()).x.v } == 4) && ({ (*cp.borrow()).y.v } == 3));
    assert!(
        (copy_only_copies_1.with(|rc| *rc.borrow()) == 6)
            && (copy_only_dtors_2.with(|rc| *rc.borrow()) == 2)
    );
    let carr: Value<Box<[CopyOnly]>> = Rc::new(RefCell::new(Box::new([
        CopyOnly::new({ 5 }),
        CopyOnly::new({ 6 }),
    ])));
    cpp_swap!(
        (carr.as_pointer() as Ptr<CopyOnly>).offset(0),
        (carr.as_pointer() as Ptr<CopyOnly>).offset(1)
    );
    assert!(
        ({ (*carr.borrow())[(0) as usize].v } == 6) && ({ (*carr.borrow())[(1) as usize].v } == 5)
    );
    assert!(
        (copy_only_copies_1.with(|rc| *rc.borrow()) == 9)
            && (copy_only_dtors_2.with(|rc| *rc.borrow()) == 3)
    );
    let mv1: Value<Moved> = Rc::new(RefCell::new(Moved::new({ 1 })));
    let _dtor_mv1 = ScopedDestructor::new(&mv1, |__p| __p.destructor());
    let mv2: Value<Moved> = Rc::new(RefCell::new(Moved::new({ 2 })));
    let _dtor_mv2 = ScopedDestructor::new(&mv2, |__p| __p.destructor());
    cpp_swap!(mv1.as_pointer(), mv2.as_pointer());
    assert!(({ (*mv1.borrow()).v } == 2) && ({ (*mv2.borrow()).v } == 1));
    assert!(({ swapped_once_6() }));
    let mp: Value<MovedPair> = Rc::new(RefCell::new(MovedPair {
        x: Moved::new({ 3 }),
        y: Moved::new({ 4 }),
    }));
    let _dtor_mp = ScopedDestructor::new(&mp, |__p| __p.destructor());
    cpp_swap!(
        field_ptr!(mp.as_pointer(), x),
        field_ptr!(mp.as_pointer(), y)
    );
    assert!(({ (*mp.borrow()).x.v } == 4) && ({ (*mp.borrow()).y.v } == 3));
    assert!(({ swapped_once_6() }));
    let marr: Value<Box<[Moved]>> = Rc::new(RefCell::new(Box::new([
        Moved::new({ 5 }),
        Moved::new({ 6 }),
    ])));
    cpp_swap!(
        (marr.as_pointer() as Ptr<Moved>).offset(0),
        (marr.as_pointer() as Ptr<Moved>).offset(1)
    );
    assert!(
        ({ (*marr.borrow())[(0) as usize].v } == 6) && ({ (*marr.borrow())[(1) as usize].v } == 5)
    );
    assert!(({ swapped_once_6() }));
    cpp_swap!(mv1.as_pointer(), mv1.as_pointer());
    assert!(({ (*mv1.borrow()).v } == 2));
    assert!(({ swapped_once_6() }));
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    let o1: Value<Owner> = Rc::new(RefCell::new(Owner {
        p: Some(Rc::new(RefCell::new(1))),
    }));
    let o2: Value<Owner> = Rc::new(RefCell::new(Owner {
        p: Some(Rc::new(RefCell::new(2))),
    }));
    cpp_swap!(o1.as_pointer(), o2.as_pointer());
    assert!(
        (({ (*o1.borrow()).p.clone() }.as_pointer().read()) == 2)
            && (({ (*o2.borrow()).p.clone() }.as_pointer().read()) == 1)
    );
    return copies_0.with(|rc| *rc.borrow());
}
pub trait ArraysImpl {
    fn copy_assign(&self, _a0: Ptr<Arrays>) -> Ptr<Arrays>;
    fn move_assign(&self, _a0: Ptr<Arrays>) -> Ptr<Arrays>;
}
impl ArraysImpl for Ptr<Arrays> {
    fn copy_assign(&self, _a0: Ptr<Arrays>) -> Ptr<Arrays> {
        let mut __i0: usize = 0_usize;
        'loop_: while (__i0 != 2_usize) {
            ({
                let _o: Ptr<Tracked> =
                    (array_field_ptr!(_a0, a) as Ptr<Tracked>).offset((__i0) as isize);
                TrackedImpl::copy_assign(
                    &(array_field_ptr!((*self), a) as Ptr<Tracked>).offset((__i0) as isize),
                    _o,
                )
            });
            __i0.prefix_inc();
        }
        let mut __i0: usize = 0_usize;
        'loop_: while (__i0 != 2_usize) {
            ({
                let _o: Ptr<Tracked> =
                    (array_field_ptr!(_a0, b) as Ptr<Tracked>).offset((__i0) as isize);
                TrackedImpl::copy_assign(
                    &(array_field_ptr!((*self), b) as Ptr<Tracked>).offset((__i0) as isize),
                    _o,
                )
            });
            __i0.prefix_inc();
        }
        return (*self).clone();
    }
    fn move_assign(&self, _a0: Ptr<Arrays>) -> Ptr<Arrays> {
        let mut __i0: usize = 0_usize;
        'loop_: while (__i0 != 2_usize) {
            ({
                let _o: Ptr<Tracked> =
                    (array_field_ptr!(_a0, a) as Ptr<Tracked>).offset((__i0) as isize);
                TrackedImpl::move_assign(
                    &(array_field_ptr!((*self), a) as Ptr<Tracked>).offset((__i0) as isize),
                    _o,
                )
            });
            __i0.prefix_inc();
        }
        let mut __i0: usize = 0_usize;
        'loop_: while (__i0 != 2_usize) {
            ({
                let _o: Ptr<Tracked> =
                    (array_field_ptr!(_a0, b) as Ptr<Tracked>).offset((__i0) as isize);
                TrackedImpl::move_assign(
                    &(array_field_ptr!((*self), b) as Ptr<Tracked>).offset((__i0) as isize),
                    _o,
                )
            });
            __i0.prefix_inc();
        }
        return (*self).clone();
    }
}
pub trait CopyOnlyImpl {
    fn destructor(&self);
    fn copy_assign(&self, o: Ptr<CopyOnly>) -> Ptr<CopyOnly>;
}
impl CopyOnlyImpl for Ptr<CopyOnly> {
    fn destructor(&self) {
        (*copy_only_dtors_2.with(Value::clone).borrow_mut()).prefix_inc();
    }
    fn copy_assign(&self, o: Ptr<CopyOnly>) -> Ptr<CopyOnly> {
        field!((*self), v).write({ o.with(|__s| __s.v) });
        (*copy_only_copies_1.with(Value::clone).borrow_mut()).prefix_inc();
        return (*self).clone();
    }
}
pub trait CopyOnlyPairImpl {
    fn copy_assign(&self, _a0: Ptr<CopyOnlyPair>) -> Ptr<CopyOnlyPair>;
    fn destructor(&self);
}
impl CopyOnlyPairImpl for Ptr<CopyOnlyPair> {
    fn copy_assign(&self, _a0: Ptr<CopyOnlyPair>) -> Ptr<CopyOnlyPair> {
        ({
            let _o: Ptr<CopyOnly> = field_ptr!(_a0, x);
            CopyOnlyImpl::copy_assign(&field_ptr!((*self), x), _o)
        });
        ({
            let _o: Ptr<CopyOnly> = field_ptr!(_a0, y);
            CopyOnlyImpl::copy_assign(&field_ptr!((*self), y), _o)
        });
        return (*self).clone();
    }
    fn destructor(&self) {
        field_ptr!(self, y).destructor();
        field_ptr!(self, x).destructor();
    }
}
pub trait MovedImpl {
    fn copy_assign(&self, o: Ptr<Moved>) -> Ptr<Moved>;
    fn move_assign(&self, o: Ptr<Moved>) -> Ptr<Moved>;
    fn destructor(&self);
}
impl MovedImpl for Ptr<Moved> {
    fn copy_assign(&self, o: Ptr<Moved>) -> Ptr<Moved> {
        field!((*self), v).write({ o.with(|__s| __s.v) });
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
        return (*self).clone();
    }
    fn move_assign(&self, o: Ptr<Moved>) -> Ptr<Moved> {
        field!((*self), v).write({ o.with(|__s| __s.v) });
        field!(o, v).write(-1_i32);
        (*move_assigns_4.with(Value::clone).borrow_mut()).prefix_inc();
        return (*self).clone();
    }
    fn destructor(&self) {
        (*moved_dtors_5.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub trait MovedPairImpl {
    fn copy_assign(&self, _a0: Ptr<MovedPair>) -> Ptr<MovedPair>;
    fn move_assign(&self, _a0: Ptr<MovedPair>) -> Ptr<MovedPair>;
    fn destructor(&self);
}
impl MovedPairImpl for Ptr<MovedPair> {
    fn copy_assign(&self, _a0: Ptr<MovedPair>) -> Ptr<MovedPair> {
        ({
            let _o: Ptr<Moved> = field_ptr!(_a0, x);
            MovedImpl::copy_assign(&field_ptr!((*self), x), _o)
        });
        ({
            let _o: Ptr<Moved> = field_ptr!(_a0, y);
            MovedImpl::copy_assign(&field_ptr!((*self), y), _o)
        });
        return (*self).clone();
    }
    fn move_assign(&self, _a0: Ptr<MovedPair>) -> Ptr<MovedPair> {
        ({
            let _o: Ptr<Moved> = field_ptr!(_a0, x);
            MovedImpl::move_assign(&field_ptr!((*self), x), _o)
        });
        ({
            let _o: Ptr<Moved> = field_ptr!(_a0, y);
            MovedImpl::move_assign(&field_ptr!((*self), y), _o)
        });
        return (*self).clone();
    }
    fn destructor(&self) {
        field_ptr!(self, y).destructor();
        field_ptr!(self, x).destructor();
    }
}
pub trait OwnerImpl {
    fn move_assign(&self, _a0: Ptr<Owner>) -> Ptr<Owner>;
}
impl OwnerImpl for Ptr<Owner> {
    fn move_assign(&self, _a0: Ptr<Owner>) -> Ptr<Owner> {
        field_ptr!((*self), p)
            .write(field!(_a0, p).with_mut(|__v: &mut Option<Value<i32>>| __v.take()));
        return (*self).clone();
    }
}
pub trait PairImpl {
    fn copy_assign(&self, _a0: Ptr<Pair>) -> Ptr<Pair>;
    fn move_assign(&self, _a0: Ptr<Pair>) -> Ptr<Pair>;
}
impl PairImpl for Ptr<Pair> {
    fn copy_assign(&self, _a0: Ptr<Pair>) -> Ptr<Pair> {
        ({
            let _o: Ptr<Tracked> = field_ptr!(_a0, x);
            TrackedImpl::copy_assign(&field_ptr!((*self), x), _o)
        });
        ({
            let _o: Ptr<Tracked> = field_ptr!(_a0, y);
            TrackedImpl::copy_assign(&field_ptr!((*self), y), _o)
        });
        return (*self).clone();
    }
    fn move_assign(&self, _a0: Ptr<Pair>) -> Ptr<Pair> {
        ({
            let _o: Ptr<Tracked> = field_ptr!(_a0, x);
            TrackedImpl::move_assign(&field_ptr!((*self), x), _o)
        });
        ({
            let _o: Ptr<Tracked> = field_ptr!(_a0, y);
            TrackedImpl::move_assign(&field_ptr!((*self), y), _o)
        });
        return (*self).clone();
    }
}
pub trait TrackedImpl {
    fn copy_assign(&self, o: Ptr<Tracked>) -> Ptr<Tracked>;
    fn move_assign(&self, o: Ptr<Tracked>) -> Ptr<Tracked>;
}
impl TrackedImpl for Ptr<Tracked> {
    fn copy_assign(&self, o: Ptr<Tracked>) -> Ptr<Tracked> {
        field!((*self), v).write({ o.with(|__s| __s.v) });
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
        return (*self).clone();
    }
    fn move_assign(&self, o: Ptr<Tracked>) -> Ptr<Tracked> {
        field!((*self), v).write({ o.with(|__s| __s.v) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = copies_0.with(|_| ());
    let _ = copy_only_copies_1.with(|_| ());
    let _ = copy_only_dtors_2.with(|_| ());
    let _ = moves_3.with(|_| ());
    let _ = move_assigns_4.with(|_| ());
    let _ = moved_dtors_5.with(|_| ());
}
