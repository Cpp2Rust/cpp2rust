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
impl CppLess for Tracked {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_1(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for Tracked {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(operator_lt_1(__lhs.clone(), __rhs.clone()))
            && !(operator_lt_1(__rhs.clone(), __lhs.clone()))
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
pub fn operator_lt_1(a: Ptr<Tracked>, b: Ptr<Tracked>) -> bool {
    return ({ a.with(|__s| __s.v) } < { b.with(|__s| __s.v) });
}
pub fn greater_2(a: Ptr<Tracked>, b: Ptr<Tracked>) -> bool {
    return ({ a.with(|__s| __s.v) } > { b.with(|__s| __s.v) });
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, CppFrom, MoveAssign, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct MoveOnly {
    #[offset(0)]
    pub v: i32,
}
impl MoveOnly {
    pub fn new(mut v: i32) -> Self {
        Self { v: v }
    }
    pub fn move_from(_a0: Ptr<MoveOnly>) -> Self {
        Self {
            v: { (*_a0.upgrade().deref()).v },
        }
    }
}
impl CppLess for MoveOnly {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_3(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for MoveOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(operator_lt_3(__lhs.clone(), __rhs.clone()))
            && !(operator_lt_3(__rhs.clone(), __lhs.clone()))
    }
}
pub fn operator_lt_3(a: Ptr<MoveOnly>, b: Ptr<MoveOnly>) -> bool {
    return ({ a.with(|__s| __s.v) } < { b.with(|__s| __s.v) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(24)]
pub struct Bag {
    #[offset(0)]
    #[byte_size(24)]
    pub items: Vec<Tracked>,
}
impl Bag {
    pub fn move_from(_a0: Ptr<Bag>) -> Self {
        Self {
            items: field!(_a0, items).with_mut(std::mem::take),
        }
    }
}
impl CppLess for Bag {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_4(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for Bag {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_5(__lhs.clone(), __rhs.clone())
    }
}
pub fn operator_lt_4(a: Ptr<Bag>, b: Ptr<Bag>) -> bool {
    return ({ (*a.upgrade().deref()).items.len() } < { (*b.upgrade().deref()).items.len() });
}
pub fn operator_eq_5(a: Ptr<Bag>, b: Ptr<Bag>) -> bool {
    return ({ (*a.upgrade().deref()).items.len() } == { (*b.upgrade().deref()).items.len() });
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let arr: Value<Box<[Tracked]>> = Rc::new(RefCell::new(Box::new([
        Tracked::new_1({ 3 }),
        Tracked::new_1({ 1 }),
        Tracked::new_1({ 4 }),
        Tracked::new_1({ 1 }),
        Tracked::new_1({ 5 }),
    ])));
    {
        let mut __tmp_0 = { arr.decay() };
        let __n = arr.decay().offset((5) as isize).iter_distance(&__tmp_0);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| cpp_cmp(&__tmp_0.iter_at(__x), &__tmp_0.iter_at(__y)));
        reorder(__order, |__i| __tmp_0.iter_at(__i));
    };
    assert!(
        ({
            (((({ (*arr.borrow())[(0) as usize].v } == 1)
                && ({ (*arr.borrow())[(1) as usize].v } == 1))
                && ({ (*arr.borrow())[(2) as usize].v } == 3))
                && ({ (*arr.borrow())[(3) as usize].v } == 4))
                && ({ (*arr.borrow())[(4) as usize].v } == 5)
        })
    );
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    let vec_: Value<Vec<Tracked>> = Rc::new(RefCell::new(Vec::new()));
    vec_.with_mut(|__b0| {
        if 4_usize > (*__b0).capacity() as usize {
            let len_0 = (*__b0).len();
            (*__b0).reserve_exact(4_usize - len_0 as usize);
        }
    });
    {
        let mut __tmp_1 = { Tracked::new_1({ 2 }) };
        (*vec_.borrow_mut()).push(__tmp_1);
    };
    {
        let mut __tmp_2 = { Tracked::new_1({ 7 }) };
        (*vec_.borrow_mut()).push(__tmp_2);
    };
    {
        let mut __tmp_3 = { Tracked::new_1({ 1 }) };
        (*vec_.borrow_mut()).push(__tmp_3);
    };
    {
        let mut __tmp_4 = { Tracked::new_1({ 8 }) };
        (*vec_.borrow_mut()).push(__tmp_4);
    };
    {
        let mut __tmp_5 = { vec_.decay().begin() };
        let __n = vec_.decay().end().iter_distance(&__tmp_5);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| cpp_cmp(&__tmp_5.iter_at(__x), &__tmp_5.iter_at(__y)));
        reorder(__order, |__i| __tmp_5.iter_at(__i));
    };
    assert!(
        ({
            ((({ (*vec_.borrow())[0_usize].v } == 1) && ({ (*vec_.borrow())[1_usize].v } == 2))
                && ({ (*vec_.borrow())[2_usize].v } == 7))
                && ({ (*vec_.borrow())[3_usize].v } == 8)
        })
    );
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    {
        let mut __tmp_6 = { vec_.decay().begin() };
        let __n = vec_.decay().end().iter_distance(&__tmp_6);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| {
            let __a = __tmp_6.iter_at(__x);
            let __b = __tmp_6.iter_at(__y);
            if greater_2.call(__a.clone(), __b.clone()) {
                std::cmp::Ordering::Less
            } else if greater_2.call(__b, __a) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        reorder(__order, |__i| __tmp_6.iter_at(__i));
    };
    assert!(({ ({ (*vec_.borrow())[0_usize].v } == 8) && ({ (*vec_.borrow())[3_usize].v } == 1) }));
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    {
        let mut __tmp_7 = { vec_.decay().begin() };
        let __n = vec_.decay().end().iter_distance(&__tmp_7);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| {
            let __a = __tmp_7.iter_at(__x);
            let __b = __tmp_7.iter_at(__y);
            if FnPtr::<fn(Ptr<Tracked>, Ptr<Tracked>) -> bool>::new(
                |a: Ptr<Tracked>, b: Ptr<Tracked>| -> bool {
                    {
                        return ({ a.with(|__s| __s.v) } < { b.with(|__s| __s.v) });
                    }
                },
            )
            .call(__a.clone(), __b.clone())
            {
                std::cmp::Ordering::Less
            } else if FnPtr::<fn(Ptr<Tracked>, Ptr<Tracked>) -> bool>::new(
                |a: Ptr<Tracked>, b: Ptr<Tracked>| -> bool {
                    {
                        return ({ a.with(|__s| __s.v) } < { b.with(|__s| __s.v) });
                    }
                },
            )
            .call(__b, __a)
            {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        reorder(__order, |__i| __tmp_7.iter_at(__i));
    };
    assert!(({ ({ (*vec_.borrow())[0_usize].v } == 1) && ({ (*vec_.borrow())[3_usize].v } == 8) }));
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    {
        let mut __tmp_8 = { vec_.decay().begin().iter_add(1_isize) };
        let __n = vec_.decay().end().iter_distance(&__tmp_8);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| {
            let __a = __tmp_8.iter_at(__x);
            let __b = __tmp_8.iter_at(__y);
            if greater_2.call(__a.clone(), __b.clone()) {
                std::cmp::Ordering::Less
            } else if greater_2.call(__b, __a) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        reorder(__order, |__i| __tmp_8.iter_at(__i));
    };
    assert!(
        ({
            (({ (*vec_.borrow())[0_usize].v } == 1) && ({ (*vec_.borrow())[1_usize].v } == 8))
                && ({ (*vec_.borrow())[3_usize].v } == 2)
        })
    );
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    let bags: Value<Box<[Bag]>> = Rc::new(RefCell::new(
        (0..3).map(|_| <Bag>::default()).collect::<Box<[Bag]>>(),
    ));
    {
        let mut __tmp_9 = { Tracked::new_1({ 1 }) };
        (*bags.borrow_mut())[(0) as usize].items.push(__tmp_9);
    };
    {
        let mut __tmp_10 = { Tracked::new_1({ 2 }) };
        (*bags.borrow_mut())[(0) as usize].items.push(__tmp_10);
    };
    {
        let mut __tmp_11 = { Tracked::new_1({ 3 }) };
        (*bags.borrow_mut())[(2) as usize].items.push(__tmp_11);
    };
    {
        let mut __tmp_12 = { bags.decay() };
        let __n = bags.decay().offset((3) as isize).iter_distance(&__tmp_12);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| cpp_cmp(&__tmp_12.iter_at(__x), &__tmp_12.iter_at(__y)));
        reorder(__order, |__i| __tmp_12.iter_at(__i));
    };
    assert!(
        ({
            (((*bags.borrow())[(0) as usize].items.is_empty())
                && ((*bags.borrow())[(1) as usize].items.len() == 1_usize))
                && ((*bags.borrow())[(2) as usize].items.len() == 2_usize)
        })
    );
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    let bag_vec: Value<Vec<Bag>> =
        Rc::new(RefCell::new(Vec::from_fn(3_usize, |_| <Bag>::default())));
    match (
        field_ref!(bag_vec.decay().offset(1_usize), items).as_pointer(),
        { Tracked::new_1({ 4 }) },
    ) {
        (__tmp_13, mut __tmp_14) => {
            __tmp_13.with_mut(|__v: &mut Vec<Tracked>| __v.push(__tmp_14));
        }
    };
    match (
        field_ref!(bag_vec.decay().offset(1_usize), items).as_pointer(),
        { Tracked::new_1({ 5 }) },
    ) {
        (__tmp_15, mut __tmp_16) => {
            __tmp_15.with_mut(|__v: &mut Vec<Tracked>| __v.push(__tmp_16));
        }
    };
    match (
        field_ref!(bag_vec.decay().offset(2_usize), items).as_pointer(),
        { Tracked::new_1({ 6 }) },
    ) {
        (__tmp_17, mut __tmp_18) => {
            __tmp_17.with_mut(|__v: &mut Vec<Tracked>| __v.push(__tmp_18));
        }
    };
    {
        let mut __tmp_19 = { bag_vec.decay().begin() };
        let __n = bag_vec.decay().end().iter_distance(&__tmp_19);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| cpp_cmp(&__tmp_19.iter_at(__x), &__tmp_19.iter_at(__y)));
        reorder(__order, |__i| __tmp_19.iter_at(__i));
    };
    assert!(
        ({
            (((*bag_vec.borrow())[0_usize].items.is_empty())
                && ((*bag_vec.borrow())[1_usize].items.len() == 1_usize))
                && (vec_elem!(bag_vec.decay().offset(2_usize), items, 1_usize).with(|__s| __s.v)
                    == 5)
        })
    );
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    assert!(
        {
            let mut __tmp_20 = { bag_vec.decay().begin() };
            let mut __tmp_21 = { bag_vec.decay().end() };
            let mut __it = (__tmp_20).clone();
            while __it != __tmp_21 && !CppEq::equal(&__it.iter_deref(), &bags.decay().offset(1)) {
                __it.iter_inc();
            }
            __it
        } == bag_vec.decay().begin().iter_add(1_isize)
    );
    assert!(
        {
            let mut __tmp_22 = { bag_vec.decay().begin() };
            let mut __tmp_23 = { bag_vec.decay().end() };
            let mut __best = __tmp_22.clone();
            if __tmp_22 != __tmp_23 {
                let mut __it = (__tmp_22).clone();
                __it.iter_inc();
                while __it != __tmp_23 {
                    if CppLess::less(&__best.iter_deref(), &__it.iter_deref()) {
                        __best = __it.clone();
                    }
                    __it.iter_inc();
                }
            }
            __best
        } == bag_vec.decay().begin().iter_add(2_isize)
    );
    assert!(
        ((if CppLess::less(&bags.decay().offset(2), &bags.decay().offset(0)) {
            bags.decay().offset(2)
        } else {
            bags.decay().offset(0)
        }) == (bags.decay().offset(0)))
    );
    assert!(
        ((if CppLess::less(&bags.decay().offset(0), &bags.decay().offset(2)) {
            bags.decay().offset(2)
        } else {
            bags.decay().offset(0)
        }) == (bags.decay().offset(2)))
    );
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    let bag_map: Value<Map<Bag, (Value<Bag>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    match (bag_vec.decay().offset(2_usize),) {
        (__tmp_24,) => {
            ({ (*bag_map.borrow_mut()).subscript_move(__tmp_24, |__k| cpp_move!(__k), || 0_i32) })
        }
    }
    .write(2);
    match (bag_vec.decay().offset(0_usize),) {
        (__tmp_25,) => {
            ({ (*bag_map.borrow_mut()).subscript_move(__tmp_25, |__k| cpp_move!(__k), || 0_i32) })
        }
    }
    .write(0);
    assert!(
        ((*(*bag_map
            .as_pointer()
            .with(|__m: &Map<Bag, (Value<Bag>, Value<i32>)>| __m.find(&bags.decay().offset(2)))
            .iter_deref()
            .upgrade()
            .deref())
        .1
        .borrow())
            == 2)
    );
    assert!(
        bag_map
            .as_pointer()
            .with(|__m: &Map<Bag, (Value<Bag>, Value<i32>)>| __m.find(&bags.decay().offset(1)))
            == bag_map
                .as_pointer()
                .with(|__m: &Map<Bag, (Value<Bag>, Value<i32>)>| __m.end())
    );
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    let moves: Value<Vec<MoveOnly>> = Rc::new(RefCell::new(Vec::new()));
    {
        let mut __tmp_26 = { MoveOnly::new({ 3 }) };
        (*moves.borrow_mut()).push(__tmp_26);
    };
    {
        let mut __tmp_27 = { MoveOnly::new({ 1 }) };
        (*moves.borrow_mut()).push(__tmp_27);
    };
    {
        let mut __tmp_28 = { MoveOnly::new({ 2 }) };
        (*moves.borrow_mut()).push(__tmp_28);
    };
    {
        let mut __tmp_29 = { moves.decay().begin() };
        let __n = moves.decay().end().iter_distance(&__tmp_29);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| cpp_cmp(&__tmp_29.iter_at(__x), &__tmp_29.iter_at(__y)));
        reorder(__order, |__i| __tmp_29.iter_at(__i));
    };
    assert!(
        ({
            (({ (*moves.borrow())[0_usize].v } == 1) && ({ (*moves.borrow())[1_usize].v } == 2))
                && ({ (*moves.borrow())[2_usize].v } == 3)
        })
    );
    {
        let mut __tmp_30 = { moves.decay().begin() };
        let __n = moves.decay().end().iter_distance(&__tmp_30);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| {
            let __a = __tmp_30.iter_at(__x);
            let __b = __tmp_30.iter_at(__y);
            if FnPtr::<fn(Ptr<MoveOnly>, Ptr<MoveOnly>) -> bool>::new(
                |a: Ptr<MoveOnly>, b: Ptr<MoveOnly>| -> bool {
                    {
                        return ({ a.with(|__s| __s.v) } > { b.with(|__s| __s.v) });
                    }
                },
            )
            .call(__a.clone(), __b.clone())
            {
                std::cmp::Ordering::Less
            } else if FnPtr::<fn(Ptr<MoveOnly>, Ptr<MoveOnly>) -> bool>::new(
                |a: Ptr<MoveOnly>, b: Ptr<MoveOnly>| -> bool {
                    {
                        return ({ a.with(|__s| __s.v) } > { b.with(|__s| __s.v) });
                    }
                },
            )
            .call(__b, __a)
            {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        reorder(__order, |__i| __tmp_30.iter_at(__i));
    };
    assert!(
        ({
            (({ (*moves.borrow())[0_usize].v } == 3) && ({ (*moves.borrow())[1_usize].v } == 2))
                && ({ (*moves.borrow())[2_usize].v } == 1)
        })
    );
    return copies_0.with(|rc| *rc.borrow());
}
pub trait BagImpl {
    fn move_assign(&self, _a0: Ptr<Bag>) -> Ptr<Bag>;
}
impl BagImpl for Ptr<Bag> {
    fn move_assign(&self, _a0: Ptr<Bag>) -> Ptr<Bag> {
        field!((*self), items).with_mut(|__b0| {
            (*__b0) = field!(_a0, items).with_mut(std::mem::take);
        });
        return (*self).clone();
    }
}
pub trait MoveOnlyImpl {
    fn move_assign(&self, _a0: Ptr<MoveOnly>) -> Ptr<MoveOnly>;
}
impl MoveOnlyImpl for Ptr<MoveOnly> {
    fn move_assign(&self, _a0: Ptr<MoveOnly>) -> Ptr<MoveOnly> {
        field!((*self), v).write({ { (*_a0.upgrade().deref()).v } });
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
}
