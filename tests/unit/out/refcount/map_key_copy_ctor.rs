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
thread_local!(
    pub static moves_1: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static made_2: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct Key {
    #[offset(0)]
    pub k: i32,
}
impl Key {
    pub fn new(mut k: i32) -> Self {
        Self { k: k }
    }
    pub fn copy_from(o: Ptr<Key>) -> Self {
        let __this: Key = Self {
            k: o.with(|__s| __s.k),
        };
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn move_from(o: Ptr<Key>) -> Self {
        let __this: Key = Self {
            k: o.with(|__s| __s.k),
        };
        (*moves_1.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl CppLess for Key {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        KeyImpl::operator_lt(__lhs, __rhs.clone())
    }
}
impl CppEq for Key {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(KeyImpl::operator_lt(__lhs, __rhs.clone()))
            && !(KeyImpl::operator_lt(__rhs, __lhs.clone()))
    }
}
impl Clone for Key {
    fn clone(&self) -> Self {
        let __src: Value<Key> = Rc::new(RefCell::new(Key { k: self.k.clone() }));
        Key::copy_from(__src.as_pointer())
    }
}
pub fn make_key_3(mut k: i32) -> Key {
    (*made_2.with(Value::clone).borrow_mut()).prefix_inc();
    return Key::new({ k });
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let mut ok: bool = true;
    let m: Value<Map<Key, (Value<Key>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    let a: Value<Key> = Rc::new(RefCell::new(Key::new({ 1 })));
    let b: Value<Key> = Rc::new(RefCell::new(Key::new({ 2 })));
    let c: Value<Key> = Rc::new(RefCell::new(Key::new({ 3 })));
    match (a.as_pointer(),) {
        (__tmp_0,) => ({ (*m.borrow_mut()).subscript(__tmp_0) }),
    }
    .write(10);
    ok = { ((ok as i32) & ((copies_0.with(|rc| *rc.borrow()) == 1) as i32)) != 0 };
    match (b.as_pointer(),) {
        (__tmp_1,) => ({ (*m.borrow_mut()).subscript(__tmp_1) }),
    }
    .write(20);
    ok = { ((ok as i32) & ((copies_0.with(|rc| *rc.borrow()) == 2) as i32)) != 0 };
    match (a.as_pointer(),) {
        (__tmp_2,) => ({ (*m.borrow_mut()).subscript(__tmp_2) }),
    }
    .write(11);
    ok = { ((ok as i32) & ((copies_0.with(|rc| *rc.borrow()) == 2) as i32)) != 0 };
    {
        let rhs_0 = ((ok as i32)
            & (((*(*m
                .as_pointer()
                .with(|__m: &Map<Key, (Value<Key>, Value<i32>)>| __m.find(&a.as_pointer()))
                .iter_deref()
                .upgrade()
                .deref())
            .1
            .borrow())
                == 11) as i32))
            != 0;
        ok = rhs_0
    };
    {
        let rhs_0 = ((ok as i32)
            & (((({
                (*m.borrow())
                    .get(&b.as_pointer())
                    .expect("out of range!")
                    .borrow()
                    .1
                    .as_pointer()
            })
            .read())
                == 20) as i32))
            != 0;
        ok = rhs_0
    };
    ok = { ((ok as i32) & ((copies_0.with(|rc| *rc.borrow()) == 2) as i32)) != 0 };
    match (Rc::new(RefCell::new(({ make_key_3(4) }))).as_pointer(),) {
        (__tmp_3,) => ({ (*m.borrow_mut()).subscript_move(__tmp_3, |__k| cpp_move!(__k)) }),
    }
    .write(40);
    match (Rc::new(RefCell::new(({ make_key_3(4) }))).as_pointer(),) {
        (__tmp_4,) => ({ (*m.borrow_mut()).subscript_move(__tmp_4, |__k| cpp_move!(__k)) }),
    }
    .write(41);
    ok = {
        ((ok as i32)
            & ((((made_2.with(|rc| *rc.borrow()) == 2) && (moves_1.with(|rc| *rc.borrow()) == 1))
                && (copies_0.with(|rc| *rc.borrow()) == 2)) as i32))
            != 0
    };
    match (c.as_pointer(),) {
        (__tmp_5,) => ({ (*m.borrow_mut()).subscript_move(__tmp_5, |__k| cpp_move!(__k)) }),
    }
    .write(30);
    match (c.as_pointer(),) {
        (__tmp_6,) => ({ (*m.borrow_mut()).subscript_move(__tmp_6, |__k| cpp_move!(__k)) }),
    }
    .write(31);
    ok = {
        ((ok as i32)
            & ((((moves_1.with(|rc| *rc.borrow()) == 2) && (copies_0.with(|rc| *rc.borrow()) == 2))
                && ({ (*c.borrow()).k } == 3)) as i32))
            != 0
    };
    {
        let rhs_0 = ((ok as i32) & (((*m.borrow()).len() == 4_usize) as i32)) != 0;
        ok = rhs_0
    };
    {
        match ((*m
            .as_pointer()
            .with(|__m: &Map<Key, (Value<Key>, Value<i32>)>| __m.begin())
            .iter_deref()
            .upgrade()
            .deref())
        .0
        .as_pointer(),)
        {
            (__tmp_7,) => ({ (*m.borrow_mut()).subscript(__tmp_7) }),
        }
        .with_mut(|__v| *__v = *__v + 1)
    };
    {
        let rhs_0 = ((ok as i32)
            & (((copies_0.with(|rc| *rc.borrow()) == 2)
                && ((match (a.as_pointer(),) {
                    (__tmp_8,) => ({ (*m.borrow_mut()).subscript(__tmp_8) }),
                }
                .read())
                    == 12)) as i32))
            != 0;
        ok = rhs_0
    };
    let mut first: Ptr<Key> = ((*m
        .as_pointer()
        .with(|__m: &Map<Key, (Value<Key>, Value<i32>)>| __m.begin())
        .iter_deref()
        .upgrade()
        .deref())
    .0
    .as_pointer());
    match (Rc::new(RefCell::new(Key::new({ 0 }))).as_pointer(),) {
        (__tmp_9,) => ({ (*m.borrow_mut()).subscript_move(__tmp_9, |__k| cpp_move!(__k)) }),
    }
    .write(0);
    ok = { ((ok as i32) & ((moves_1.with(|rc| *rc.borrow()) == 3) as i32)) != 0 };
    {
        let rhs_0 = ((ok as i32)
            & (({
                ((*m.as_pointer()
                    .with(|__m: &Map<Key, (Value<Key>, Value<i32>)>| __m.find(&a.as_pointer()))
                    .iter_deref()
                    .upgrade()
                    .deref())
                .0
                .as_pointer())
            } == { (first).clone() }) as i32))
            != 0;
        ok = rhs_0
    };
    let n: Value<Map<Key, (Value<Key>, Value<i32>)>> = Rc::new(RefCell::new((*m.borrow()).clone()));
    ok = { ((ok as i32) & ((copies_0.with(|rc| *rc.borrow()) == (2 + 5)) as i32)) != 0 };
    {
        let rhs_0 = ((ok as i32)
            & ((((*n.borrow()).len() == 5_usize)
                && ((match (a.as_pointer(),) {
                    (__tmp_10,) => ({ (*n.borrow_mut()).subscript(__tmp_10) }),
                }
                .read())
                    == 12)) as i32))
            != 0;
        ok = rhs_0
    };
    Map::erase(
        m.as_pointer(),
        &m.as_pointer()
            .with(|__m: &Map<Key, (Value<Key>, Value<i32>)>| __m.find(&a.as_pointer())),
    );
    {
        let rhs_0 = ((ok as i32)
            & ((Map::erase_key(m.as_pointer(), &b.as_pointer()) == 1_usize) as i32))
            != 0;
        ok = rhs_0
    };
    {
        let rhs_0 = ((ok as i32)
            & ((Map::erase_key(m.as_pointer(), &b.as_pointer()) == 0_usize) as i32))
            != 0;
        ok = rhs_0
    };
    {
        let rhs_0 = ((ok as i32)
            & ((Map::erase_key(
                m.as_pointer(),
                &(*m.as_pointer()
                    .with(|__m: &Map<Key, (Value<Key>, Value<i32>)>| __m.begin())
                    .iter_deref()
                    .upgrade()
                    .deref())
                .0
                .as_pointer(),
            ) == 1_usize) as i32))
            != 0;
        ok = rhs_0
    };
    ok = {
        ((ok as i32)
            & (((copies_0.with(|rc| *rc.borrow()) == 7) && (moves_1.with(|rc| *rc.borrow()) == 3))
                as i32))
            != 0
    };
    {
        let rhs_0 = ((ok as i32)
            & ((((*m.borrow()).len() == 2_usize) && ((*n.borrow()).len() == 5_usize)) as i32))
            != 0;
        ok = rhs_0
    };
    let mut sum: i32 = 0;
    'loop_: for entry in IterRange::new(
        m.as_pointer()
            .with(|__m: &Map<Key, (Value<Key>, Value<i32>)>| __m.begin()),
        m.as_pointer()
            .with(|__m: &Map<Key, (Value<Key>, Value<i32>)>| __m.end()),
    ) {
        let entry: Ptr<(Value<Key>, Value<i32>)> = entry.iter_deref();
        sum += ({ { (*(*entry.upgrade().deref()).0.borrow()).k } } * {
            (*(*entry.upgrade().deref()).1.borrow())
        });
    }
    ok = { ((ok as i32) & ((sum == ((3 * 31) + (4 * 41))) as i32)) != 0 };
    ok = {
        ((ok as i32)
            & (((copies_0.with(|rc| *rc.borrow()) == 7) && (moves_1.with(|rc| *rc.borrow()) == 3))
                as i32))
            != 0
    };
    printf!(
        b"%d %d %d\n",
        copies_0.with(|rc| *rc.borrow()),
        moves_1.with(|rc| *rc.borrow()),
        made_2.with(|rc| *rc.borrow())
    );
    return if ok { 0 } else { 1 };
}
pub trait KeyImpl {
    fn operator_lt(&self, o: Ptr<Key>) -> bool;
}
impl KeyImpl for Ptr<Key> {
    fn operator_lt(&self, o: Ptr<Key>) -> bool {
        return ({ (*self).with(|__s| __s.k) } < { o.with(|__s| __s.k) });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = copies_0.with(|_| ());
    let _ = moves_1.with(|_| ());
    let _ = made_2.with(|_| ());
}
