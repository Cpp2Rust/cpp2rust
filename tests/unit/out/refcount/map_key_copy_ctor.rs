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
        (__tmp_0,) => ({ (*m.borrow_mut()).subscript(__tmp_0, || 0_i32) }),
    }
    .write(10);
    {
        match (
            { ((copies_0.with(|rc| *rc.borrow()) == 1) as i32) },
            &mut (ok),
        ) {
            (__tmp_1, __tmp_2) => {
                (*__tmp_2) = (((*__tmp_2) as i32) & (__tmp_1 as i32)) != 0;
            }
        }
    };
    match (b.as_pointer(),) {
        (__tmp_3,) => ({ (*m.borrow_mut()).subscript(__tmp_3, || 0_i32) }),
    }
    .write(20);
    {
        match (
            { ((copies_0.with(|rc| *rc.borrow()) == 2) as i32) },
            &mut (ok),
        ) {
            (__tmp_4, __tmp_5) => {
                (*__tmp_5) = (((*__tmp_5) as i32) & (__tmp_4 as i32)) != 0;
            }
        }
    };
    match (a.as_pointer(),) {
        (__tmp_6,) => ({ (*m.borrow_mut()).subscript(__tmp_6, || 0_i32) }),
    }
    .write(11);
    {
        match (
            { ((copies_0.with(|rc| *rc.borrow()) == 2) as i32) },
            &mut (ok),
        ) {
            (__tmp_7, __tmp_8) => {
                (*__tmp_8) = (((*__tmp_8) as i32) & (__tmp_7 as i32)) != 0;
            }
        }
    };
    {
        match (
            {
                (((*(*m
                    .as_pointer()
                    .with(|__m: &Map<Key, (Value<Key>, Value<i32>)>| __m.find(&a.as_pointer()))
                    .iter_deref()
                    .upgrade()
                    .deref())
                .1
                .borrow())
                    == 11) as i32)
            },
            &mut (ok),
        ) {
            (__tmp_9, __tmp_10) => {
                (*__tmp_10) = (((*__tmp_10) as i32) & (__tmp_9 as i32)) != 0;
            }
        }
    };
    {
        match (
            {
                (((({
                    (*m.borrow())
                        .get(&b.as_pointer())
                        .expect("out of range!")
                        .borrow()
                        .1
                        .as_pointer()
                })
                .read())
                    == 20) as i32)
            },
            &mut (ok),
        ) {
            (__tmp_11, __tmp_12) => {
                (*__tmp_12) = (((*__tmp_12) as i32) & (__tmp_11 as i32)) != 0;
            }
        }
    };
    {
        match (
            { ((copies_0.with(|rc| *rc.borrow()) == 2) as i32) },
            &mut (ok),
        ) {
            (__tmp_13, __tmp_14) => {
                (*__tmp_14) = (((*__tmp_14) as i32) & (__tmp_13 as i32)) != 0;
            }
        }
    };
    match (Rc::new(RefCell::new(({ make_key_3(4) }))).as_pointer(),) {
        (__tmp_15,) => {
            ({ (*m.borrow_mut()).subscript_move(__tmp_15, |__k| cpp_move!(__k), || 0_i32) })
        }
    }
    .write(40);
    match (Rc::new(RefCell::new(({ make_key_3(4) }))).as_pointer(),) {
        (__tmp_16,) => {
            ({ (*m.borrow_mut()).subscript_move(__tmp_16, |__k| cpp_move!(__k), || 0_i32) })
        }
    }
    .write(41);
    {
        match (
            {
                ((((made_2.with(|rc| *rc.borrow()) == 2) && (moves_1.with(|rc| *rc.borrow()) == 1))
                    && (copies_0.with(|rc| *rc.borrow()) == 2)) as i32)
            },
            &mut (ok),
        ) {
            (__tmp_17, __tmp_18) => {
                (*__tmp_18) = (((*__tmp_18) as i32) & (__tmp_17 as i32)) != 0;
            }
        }
    };
    match (c.as_pointer(),) {
        (__tmp_19,) => {
            ({ (*m.borrow_mut()).subscript_move(__tmp_19, |__k| cpp_move!(__k), || 0_i32) })
        }
    }
    .write(30);
    match (c.as_pointer(),) {
        (__tmp_20,) => {
            ({ (*m.borrow_mut()).subscript_move(__tmp_20, |__k| cpp_move!(__k), || 0_i32) })
        }
    }
    .write(31);
    {
        match (
            {
                ((((moves_1.with(|rc| *rc.borrow()) == 2)
                    && (copies_0.with(|rc| *rc.borrow()) == 2))
                    && ({ (*c.borrow()).k } == 3)) as i32)
            },
            &mut (ok),
        ) {
            (__tmp_21, __tmp_22) => {
                (*__tmp_22) = (((*__tmp_22) as i32) & (__tmp_21 as i32)) != 0;
            }
        }
    };
    {
        match ({ (((*m.borrow()).len() == 4_usize) as i32) }, &mut (ok)) {
            (__tmp_23, __tmp_24) => {
                (*__tmp_24) = (((*__tmp_24) as i32) & (__tmp_23 as i32)) != 0;
            }
        }
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
            (__tmp_25,) => ({ (*m.borrow_mut()).subscript(__tmp_25, || 0_i32) }),
        }
        .with_mut(|__v| *__v = *__v + 1)
    };
    {
        match (
            {
                (((copies_0.with(|rc| *rc.borrow()) == 2)
                    && ((match (a.as_pointer(),) {
                        (__tmp_28,) => ({ (*m.borrow_mut()).subscript(__tmp_28, || 0_i32) }),
                    }
                    .read())
                        == 12)) as i32)
            },
            &mut (ok),
        ) {
            (__tmp_26, __tmp_27) => {
                (*__tmp_27) = (((*__tmp_27) as i32) & (__tmp_26 as i32)) != 0;
            }
        }
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
        (__tmp_29,) => {
            ({ (*m.borrow_mut()).subscript_move(__tmp_29, |__k| cpp_move!(__k), || 0_i32) })
        }
    }
    .write(0);
    {
        match (
            { ((moves_1.with(|rc| *rc.borrow()) == 3) as i32) },
            &mut (ok),
        ) {
            (__tmp_30, __tmp_31) => {
                (*__tmp_31) = (((*__tmp_31) as i32) & (__tmp_30 as i32)) != 0;
            }
        }
    };
    {
        match (
            {
                (({
                    ((*m.as_pointer()
                        .with(|__m: &Map<Key, (Value<Key>, Value<i32>)>| __m.find(&a.as_pointer()))
                        .iter_deref()
                        .upgrade()
                        .deref())
                    .0
                    .as_pointer())
                } == { (first).clone() }) as i32)
            },
            &mut (ok),
        ) {
            (__tmp_32, __tmp_33) => {
                (*__tmp_33) = (((*__tmp_33) as i32) & (__tmp_32 as i32)) != 0;
            }
        }
    };
    let n: Value<Map<Key, (Value<Key>, Value<i32>)>> = Rc::new(RefCell::new((*m.borrow()).clone()));
    {
        match (
            { ((copies_0.with(|rc| *rc.borrow()) == (2 + 5)) as i32) },
            &mut (ok),
        ) {
            (__tmp_34, __tmp_35) => {
                (*__tmp_35) = (((*__tmp_35) as i32) & (__tmp_34 as i32)) != 0;
            }
        }
    };
    {
        match (
            {
                ((((*n.borrow()).len() == 5_usize)
                    && ((match (a.as_pointer(),) {
                        (__tmp_38,) => ({ (*n.borrow_mut()).subscript(__tmp_38, || 0_i32) }),
                    }
                    .read())
                        == 12)) as i32)
            },
            &mut (ok),
        ) {
            (__tmp_36, __tmp_37) => {
                (*__tmp_37) = (((*__tmp_37) as i32) & (__tmp_36 as i32)) != 0;
            }
        }
    };
    Map::erase(
        m.as_pointer(),
        &m.as_pointer()
            .with(|__m: &Map<Key, (Value<Key>, Value<i32>)>| __m.find(&a.as_pointer())),
    );
    {
        match (
            { ((Map::erase_key(m.as_pointer(), &b.as_pointer()) == 1_usize) as i32) },
            &mut (ok),
        ) {
            (__tmp_39, __tmp_40) => {
                (*__tmp_40) = (((*__tmp_40) as i32) & (__tmp_39 as i32)) != 0;
            }
        }
    };
    {
        match (
            { ((Map::erase_key(m.as_pointer(), &b.as_pointer()) == 0_usize) as i32) },
            &mut (ok),
        ) {
            (__tmp_41, __tmp_42) => {
                (*__tmp_42) = (((*__tmp_42) as i32) & (__tmp_41 as i32)) != 0;
            }
        }
    };
    {
        match (
            {
                ((Map::erase_key(
                    m.as_pointer(),
                    &(*m.as_pointer()
                        .with(|__m: &Map<Key, (Value<Key>, Value<i32>)>| __m.begin())
                        .iter_deref()
                        .upgrade()
                        .deref())
                    .0
                    .as_pointer(),
                ) == 1_usize) as i32)
            },
            &mut (ok),
        ) {
            (__tmp_43, __tmp_44) => {
                (*__tmp_44) = (((*__tmp_44) as i32) & (__tmp_43 as i32)) != 0;
            }
        }
    };
    {
        match (
            {
                (((copies_0.with(|rc| *rc.borrow()) == 7) && (moves_1.with(|rc| *rc.borrow()) == 3))
                    as i32)
            },
            &mut (ok),
        ) {
            (__tmp_45, __tmp_46) => {
                (*__tmp_46) = (((*__tmp_46) as i32) & (__tmp_45 as i32)) != 0;
            }
        }
    };
    {
        match (
            { ((((*m.borrow()).len() == 2_usize) && ((*n.borrow()).len() == 5_usize)) as i32) },
            &mut (ok),
        ) {
            (__tmp_47, __tmp_48) => {
                (*__tmp_48) = (((*__tmp_48) as i32) & (__tmp_47 as i32)) != 0;
            }
        }
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
    {
        match ({ ((sum == ((3 * 31) + (4 * 41))) as i32) }, &mut (ok)) {
            (__tmp_49, __tmp_50) => {
                (*__tmp_50) = (((*__tmp_50) as i32) & (__tmp_49 as i32)) != 0;
            }
        }
    };
    {
        match (
            {
                (((copies_0.with(|rc| *rc.borrow()) == 7) && (moves_1.with(|rc| *rc.borrow()) == 3))
                    as i32)
            },
            &mut (ok),
        ) {
            (__tmp_51, __tmp_52) => {
                (*__tmp_52) = (((*__tmp_52) as i32) & (__tmp_51 as i32)) != 0;
            }
        }
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
