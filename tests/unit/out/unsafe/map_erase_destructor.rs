extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut destroyed_0: std::cell::LazyCell<[i32; 32]> =
    std::cell::LazyCell::new(|| unsafe { [0_i32; 32] });
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CppFromUnsafe, DestructorUnsafe)]
#[cpp_from(i32, new_1, value)]
pub struct Tracked {
    pub id: i32,
}
impl Tracked {
    pub unsafe fn new_2() -> Self {
        let mut this = Self { id: 0 };
        this
    }
    pub unsafe fn new_1(mut id: i32) -> Self {
        let mut this = Self { id: id };
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(self.id) as usize]
            .prefix_inc();
    }
}
impl std::cmp::Ord for Tracked {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_1(self as *const Tracked, other as *const Tracked) {
                std::cmp::Ordering::Less
            } else if operator_lt_1(other as *const Tracked, self as *const Tracked) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Tracked {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Tracked {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(operator_lt_1(self as *const Tracked, other as *const Tracked))
                && !(operator_lt_1(other as *const Tracked, self as *const Tracked))
        }
    }
}
impl std::cmp::Eq for Tracked {}
impl Default for Tracked {
    fn default() -> Self {
        unsafe { Tracked::new_2() }
    }
}
pub unsafe fn operator_lt_1(a: *const Tracked, b: *const Tracked) -> bool {
    return (((*a).id) < ((*b).id));
}
pub unsafe fn check_erase_2(mut key_destructor: bool, mut value_destructor: bool) {
    let mut one: Tracked = Tracked::new_1({ 1 });
    let _dtor_one = ScopedDestructorUnsafe::new(&raw mut one, Tracked::destructor);
    let mut two: Tracked = Tracked::new_1({ 2 });
    let _dtor_two = ScopedDestructorUnsafe::new(&raw mut two, Tracked::destructor);
    let mut three: Tracked = Tracked::new_1({ 3 });
    let _dtor_three = ScopedDestructorUnsafe::new(&raw mut three, Tracked::destructor);
    let mut m: UnsafeMap<Tracked, (Tracked, Tracked)> = UnsafeMap::new();
    (*m.subscript(&one)) = Tracked::new_1({ 11 });
    (*m.subscript(&two)) = Tracked::new_1({ 12 });
    (*m.subscript(&three)) = Tracked::new_1({ 13 });
    'loop_: for n in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)).as_mut_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))
            .as_mut_ptr()
            .offset((32_i64) as isize),
    ) {
        let n: *mut i32 = &mut (*n);
        (*n) = 0;
    }
    {
        let mut first: UnsafeBidirectionalIterator<(Tracked, Tracked)> = m.find(&one);
        let mut middle: UnsafeBidirectionalIterator<(Tracked, Tracked)> = m.find(&two);
        let mut last: UnsafeBidirectionalIterator<(Tracked, Tracked)> = m.find(&three);
        let mut retained: UnsafeBidirectionalIterator<(Tracked, Tracked)> = (middle).clone();
        let mut retained_const: UnsafeBidirectionalIterator<(Tracked, Tracked)> =
            <UnsafeBidirectionalIterator<(Tracked, Tracked)>>::from((middle).clone());
        let mut first_value: *mut Tracked = (&mut (*first.iter_deref()).1 as *mut Tracked);
        let mut last_value: *mut Tracked = (&mut (*last.iter_deref()).1 as *mut Tracked);
        let mut end: UnsafeBidirectionalIterator<(Tracked, Tracked)> = m.end();
        let mut next: UnsafeBidirectionalIterator<(Tracked, Tracked)> = m.erase(&(middle).clone());
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(2) as usize])
                == (key_destructor as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(12) as usize])
                == (value_destructor as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(1) as usize]) == (0))
                && (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(3) as usize])
                    == (0))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(11) as usize]) == (0))
                && (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(13) as usize])
                    == (0))
        );
        assert!(((m.len()) == (2_usize)) && (m.find(&two) == end));
        assert!((next == last) && (first == m.begin()));
        assert!(
            ((&mut (*first.iter_deref()).1 as *mut Tracked) == (first_value))
                && ((&mut (*last.iter_deref()).1 as *mut Tracked) == (last_value))
        );
        let mut walking: UnsafeBidirectionalIterator<(Tracked, Tracked)> = (first).clone();
        walking.iter_inc();
        assert!(walking == last);
        walking.iter_inc();
        assert!(walking == end);
        let mut const_first: UnsafeBidirectionalIterator<(Tracked, Tracked)> =
            <UnsafeBidirectionalIterator<(Tracked, Tracked)>>::from((first).clone());
        next = m.erase(&(const_first).clone());
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(1) as usize])
                == (key_destructor as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(11) as usize])
                == (value_destructor as i32))
        );
        assert!((((m.len()) == (1_usize)) && (next == last)) && (m.begin() == last));
        assert!(((&mut (*last.iter_deref()).1 as *mut Tracked) == (last_value)));
        next = m.erase(&(last).clone());
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(3) as usize])
                == (key_destructor as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(13) as usize])
                == (value_destructor as i32))
        );
        assert!(((next == end) && ((m.len()) == (0_usize))) && (m.begin() == end));
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (32)) {
        let mut expected: i32 = 0;
        if ((i) >= (1)) && ((i) <= (3)) {
            expected = (key_destructor as i32);
        }
        if ((i) >= (11)) && ((i) <= (13)) {
            expected = (value_destructor as i32);
        }
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(i) as usize])
                == (expected))
        );
        i.prefix_inc();
    }
}
pub unsafe fn check_erase_3(mut key_destructor: bool, mut value_destructor: bool) {
    let mut one: Tracked = Tracked::new_1({ 1 });
    let _dtor_one = ScopedDestructorUnsafe::new(&raw mut one, Tracked::destructor);
    let mut two: Tracked = Tracked::new_1({ 2 });
    let _dtor_two = ScopedDestructorUnsafe::new(&raw mut two, Tracked::destructor);
    let mut three: Tracked = Tracked::new_1({ 3 });
    let _dtor_three = ScopedDestructorUnsafe::new(&raw mut three, Tracked::destructor);
    let mut m: UnsafeMap<Tracked, (Tracked, i32)> = UnsafeMap::new();
    (*m.subscript(&one)) = 11;
    (*m.subscript(&two)) = 12;
    (*m.subscript(&three)) = 13;
    'loop_: for n in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)).as_mut_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))
            .as_mut_ptr()
            .offset((32_i64) as isize),
    ) {
        let n: *mut i32 = &mut (*n);
        (*n) = 0;
    }
    {
        let mut first: UnsafeBidirectionalIterator<(Tracked, i32)> = m.find(&one);
        let mut middle: UnsafeBidirectionalIterator<(Tracked, i32)> = m.find(&two);
        let mut last: UnsafeBidirectionalIterator<(Tracked, i32)> = m.find(&three);
        let mut retained: UnsafeBidirectionalIterator<(Tracked, i32)> = (middle).clone();
        let mut retained_const: UnsafeBidirectionalIterator<(Tracked, i32)> =
            <UnsafeBidirectionalIterator<(Tracked, i32)>>::from((middle).clone());
        let mut first_value: *mut i32 = (&mut (*first.iter_deref()).1 as *mut i32);
        let mut last_value: *mut i32 = (&mut (*last.iter_deref()).1 as *mut i32);
        let mut end: UnsafeBidirectionalIterator<(Tracked, i32)> = m.end();
        let mut next: UnsafeBidirectionalIterator<(Tracked, i32)> = m.erase(&(middle).clone());
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(2) as usize])
                == (key_destructor as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(12) as usize])
                == (value_destructor as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(1) as usize]) == (0))
                && (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(3) as usize])
                    == (0))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(11) as usize]) == (0))
                && (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(13) as usize])
                    == (0))
        );
        assert!(((m.len()) == (2_usize)) && (m.find(&two) == end));
        assert!((next == last) && (first == m.begin()));
        assert!(
            ((&mut (*first.iter_deref()).1 as *mut i32) == (first_value))
                && ((&mut (*last.iter_deref()).1 as *mut i32) == (last_value))
        );
        let mut walking: UnsafeBidirectionalIterator<(Tracked, i32)> = (first).clone();
        walking.iter_inc();
        assert!(walking == last);
        walking.iter_inc();
        assert!(walking == end);
        let mut const_first: UnsafeBidirectionalIterator<(Tracked, i32)> =
            <UnsafeBidirectionalIterator<(Tracked, i32)>>::from((first).clone());
        next = m.erase(&(const_first).clone());
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(1) as usize])
                == (key_destructor as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(11) as usize])
                == (value_destructor as i32))
        );
        assert!((((m.len()) == (1_usize)) && (next == last)) && (m.begin() == last));
        assert!(((&mut (*last.iter_deref()).1 as *mut i32) == (last_value)));
        next = m.erase(&(last).clone());
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(3) as usize])
                == (key_destructor as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(13) as usize])
                == (value_destructor as i32))
        );
        assert!(((next == end) && ((m.len()) == (0_usize))) && (m.begin() == end));
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (32)) {
        let mut expected: i32 = 0;
        if ((i) >= (1)) && ((i) <= (3)) {
            expected = (key_destructor as i32);
        }
        if ((i) >= (11)) && ((i) <= (13)) {
            expected = (value_destructor as i32);
        }
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(i) as usize])
                == (expected))
        );
        i.prefix_inc();
    }
}
pub unsafe fn check_erase_4(mut key_destructor: bool, mut value_destructor: bool) {
    let mut one: i32 = 1;
    let mut two: i32 = 2;
    let mut three: i32 = 3;
    let mut m: UnsafeMap<i32, (i32, Tracked)> = UnsafeMap::new();
    (*m.subscript(&one)) = Tracked::new_1({ 11 });
    (*m.subscript(&two)) = Tracked::new_1({ 12 });
    (*m.subscript(&three)) = Tracked::new_1({ 13 });
    'loop_: for n in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)).as_mut_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))
            .as_mut_ptr()
            .offset((32_i64) as isize),
    ) {
        let n: *mut i32 = &mut (*n);
        (*n) = 0;
    }
    {
        let mut first: UnsafeBidirectionalIterator<(i32, Tracked)> = m.find(&one);
        let mut middle: UnsafeBidirectionalIterator<(i32, Tracked)> = m.find(&two);
        let mut last: UnsafeBidirectionalIterator<(i32, Tracked)> = m.find(&three);
        let mut retained: UnsafeBidirectionalIterator<(i32, Tracked)> = (middle).clone();
        let mut retained_const: UnsafeBidirectionalIterator<(i32, Tracked)> =
            <UnsafeBidirectionalIterator<(i32, Tracked)>>::from((middle).clone());
        let mut first_value: *mut Tracked = (&mut (*first.iter_deref()).1 as *mut Tracked);
        let mut last_value: *mut Tracked = (&mut (*last.iter_deref()).1 as *mut Tracked);
        let mut end: UnsafeBidirectionalIterator<(i32, Tracked)> = m.end();
        let mut next: UnsafeBidirectionalIterator<(i32, Tracked)> = m.erase(&(middle).clone());
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(2) as usize])
                == (key_destructor as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(12) as usize])
                == (value_destructor as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(1) as usize]) == (0))
                && (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(3) as usize])
                    == (0))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(11) as usize]) == (0))
                && (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(13) as usize])
                    == (0))
        );
        assert!(((m.len()) == (2_usize)) && (m.find(&two) == end));
        assert!((next == last) && (first == m.begin()));
        assert!(
            ((&mut (*first.iter_deref()).1 as *mut Tracked) == (first_value))
                && ((&mut (*last.iter_deref()).1 as *mut Tracked) == (last_value))
        );
        let mut walking: UnsafeBidirectionalIterator<(i32, Tracked)> = (first).clone();
        walking.iter_inc();
        assert!(walking == last);
        walking.iter_inc();
        assert!(walking == end);
        let mut const_first: UnsafeBidirectionalIterator<(i32, Tracked)> =
            <UnsafeBidirectionalIterator<(i32, Tracked)>>::from((first).clone());
        next = m.erase(&(const_first).clone());
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(1) as usize])
                == (key_destructor as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(11) as usize])
                == (value_destructor as i32))
        );
        assert!((((m.len()) == (1_usize)) && (next == last)) && (m.begin() == last));
        assert!(((&mut (*last.iter_deref()).1 as *mut Tracked) == (last_value)));
        next = m.erase(&(last).clone());
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(3) as usize])
                == (key_destructor as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(13) as usize])
                == (value_destructor as i32))
        );
        assert!(((next == end) && ((m.len()) == (0_usize))) && (m.begin() == end));
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (32)) {
        let mut expected: i32 = 0;
        if ((i) >= (1)) && ((i) <= (3)) {
            expected = (key_destructor as i32);
        }
        if ((i) >= (11)) && ((i) <= (13)) {
            expected = (value_destructor as i32);
        }
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(i) as usize])
                == (expected))
        );
        i.prefix_inc();
    }
}
pub unsafe fn check_erase_5(mut key_destructor: bool, mut value_destructor: bool) {
    let mut one: i32 = 1;
    let mut two: i32 = 2;
    let mut three: i32 = 3;
    let mut m: UnsafeMap<i32, (i32, i32)> = UnsafeMap::new();
    (*m.subscript(&one)) = 11;
    (*m.subscript(&two)) = 12;
    (*m.subscript(&three)) = 13;
    'loop_: for n in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)).as_mut_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))
            .as_mut_ptr()
            .offset((32_i64) as isize),
    ) {
        let n: *mut i32 = &mut (*n);
        (*n) = 0;
    }
    {
        let mut first: UnsafeBidirectionalIterator<(i32, i32)> = m.find(&one);
        let mut middle: UnsafeBidirectionalIterator<(i32, i32)> = m.find(&two);
        let mut last: UnsafeBidirectionalIterator<(i32, i32)> = m.find(&three);
        let mut retained: UnsafeBidirectionalIterator<(i32, i32)> = (middle).clone();
        let mut retained_const: UnsafeBidirectionalIterator<(i32, i32)> =
            <UnsafeBidirectionalIterator<(i32, i32)>>::from((middle).clone());
        let mut first_value: *mut i32 = (&mut (*first.iter_deref()).1 as *mut i32);
        let mut last_value: *mut i32 = (&mut (*last.iter_deref()).1 as *mut i32);
        let mut end: UnsafeBidirectionalIterator<(i32, i32)> = m.end();
        let mut next: UnsafeBidirectionalIterator<(i32, i32)> = m.erase(&(middle).clone());
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(2) as usize])
                == (key_destructor as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(12) as usize])
                == (value_destructor as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(1) as usize]) == (0))
                && (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(3) as usize])
                    == (0))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(11) as usize]) == (0))
                && (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(13) as usize])
                    == (0))
        );
        assert!(((m.len()) == (2_usize)) && (m.find(&two) == end));
        assert!((next == last) && (first == m.begin()));
        assert!(
            ((&mut (*first.iter_deref()).1 as *mut i32) == (first_value))
                && ((&mut (*last.iter_deref()).1 as *mut i32) == (last_value))
        );
        let mut walking: UnsafeBidirectionalIterator<(i32, i32)> = (first).clone();
        walking.iter_inc();
        assert!(walking == last);
        walking.iter_inc();
        assert!(walking == end);
        let mut const_first: UnsafeBidirectionalIterator<(i32, i32)> =
            <UnsafeBidirectionalIterator<(i32, i32)>>::from((first).clone());
        next = m.erase(&(const_first).clone());
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(1) as usize])
                == (key_destructor as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(11) as usize])
                == (value_destructor as i32))
        );
        assert!((((m.len()) == (1_usize)) && (next == last)) && (m.begin() == last));
        assert!(((&mut (*last.iter_deref()).1 as *mut i32) == (last_value)));
        next = m.erase(&(last).clone());
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(3) as usize])
                == (key_destructor as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(13) as usize])
                == (value_destructor as i32))
        );
        assert!(((next == end) && ((m.len()) == (0_usize))) && (m.begin() == end));
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (32)) {
        let mut expected: i32 = 0;
        if ((i) >= (1)) && ((i) <= (3)) {
            expected = (key_destructor as i32);
        }
        if ((i) >= (11)) && ((i) <= (13)) {
            expected = (value_destructor as i32);
        }
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(i) as usize])
                == (expected))
        );
        i.prefix_inc();
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, DestructorUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct MoveOnly {
    pub id: i32,
}
impl MoveOnly {
    pub unsafe fn new(mut id: i32) -> Self {
        let mut this = Self { id: id };
        this
    }
    pub unsafe fn move_from(other: *mut MoveOnly) -> Self {
        let mut this = Self { id: (*other).id };
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(self.id) as usize]
            .prefix_inc();
    }
}
impl std::cmp::Ord for MoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_6(self as *const MoveOnly, other as *const MoveOnly) {
                std::cmp::Ordering::Less
            } else if operator_lt_6(other as *const MoveOnly, self as *const MoveOnly) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for MoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for MoveOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(operator_lt_6(self as *const MoveOnly, other as *const MoveOnly))
                && !(operator_lt_6(other as *const MoveOnly, self as *const MoveOnly))
        }
    }
}
impl std::cmp::Eq for MoveOnly {}
pub unsafe fn operator_lt_6(a: *const MoveOnly, b: *const MoveOnly) -> bool {
    return (((*a).id) < ((*b).id));
}
pub unsafe fn check_nested_move_only_7() {
    let mut outer: UnsafeMap<i32, (i32, UnsafeMap<MoveOnly, (MoveOnly, Tracked)>)> =
        UnsafeMap::new();
    let m: *mut UnsafeMap<MoveOnly, (MoveOnly, Tracked)> =
        &mut (*outer.subscript_move(&mut 0, |__k| cpp_move_unsafe!(__k)));
    let mut key: MoveOnly = MoveOnly::new({ 4 });
    let _dtor_key = ScopedDestructorUnsafe::new(&raw mut key, MoveOnly::destructor);
    (*(*m).subscript_move(&mut key, |__k| cpp_move_unsafe!(__k))) = Tracked::new_1({ 14 });
    (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(4) as usize] = {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(14) as usize] = 0;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(14) as usize]
    };
    let mut next: UnsafeBidirectionalIterator<(MoveOnly, Tracked)> = (*m).erase(&(*m).begin());
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(4) as usize]) == (1))
            && (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0))[(14) as usize])
                == (1))
    );
    assert!((((*m).len()) == (0_usize)) && (next == (*m).end()));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { check_erase_2(true, true) });
    (unsafe { check_erase_3(true, false) });
    (unsafe { check_erase_4(false, true) });
    (unsafe { check_erase_5(false, false) });
    (unsafe { check_nested_move_only_7() });
    0
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destroyed_0);
}
