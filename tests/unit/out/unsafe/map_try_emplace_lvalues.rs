extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut made_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut visits_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Item {
    pub value: i32,
}
impl Item {
    pub unsafe fn new_1() -> Self {
        let mut this = Self { value: 0 };
        this
    }
    pub unsafe fn new_2(counter: *mut i32, mut amount: i32) -> Self {
        let mut this = Self {
            value: match ({ amount }, &mut (*counter)) {
                (__tmp_0, __tmp_1) => {
                    (*__tmp_1) = (((*__tmp_1) as i32) + (__tmp_0 as i32)) as i32;
                    (*__tmp_1).clone()
                }
            },
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut made_0)).prefix_inc();
        this
    }
}
impl Default for Item {
    fn default() -> Self {
        unsafe { Item::new_1() }
    }
}
pub unsafe fn receiver_2(m: *mut UnsafeMap<i32, (i32, Item)>) -> *mut UnsafeMap<i32, (i32, Item)> {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut visits_1)).prefix_inc();
    return m;
}
pub unsafe fn place_3(n: *mut i32) -> *mut i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut visits_1)).prefix_inc();
    return n;
}
pub static mut temporary_calls_4: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Temporary {
    pub n: i32,
    pub data: [i32; 2],
}
impl Temporary {
    pub unsafe fn new() -> Self {
        let mut this = Self { n: 5, data: [6, 7] };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut temporary_calls_4)).prefix_inc();
        this
    }
}
impl Default for Temporary {
    fn default() -> Self {
        unsafe { Temporary::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Nested {
    pub inner: Temporary,
}
pub unsafe fn make_temporary_5() -> Temporary {
    return Temporary::new();
}
pub unsafe fn make_nested_6() -> Nested {
    return <Nested>::default();
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct Owner {
    pub ptr: Option<Box<Temporary>>,
}
impl Owner {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            ptr: Some(Box::new(Temporary::new())),
        };
        this
    }
    pub unsafe fn value(&mut self) -> *const i32 {
        return &(*self.ptr.as_deref().map_or(::std::ptr::null_mut(), |v| {
            v as *const Temporary as *mut Temporary
        }))
        .n;
    }
    pub unsafe fn move_from(_a0: *mut Owner) -> Self {
        let mut this = Self {
            ptr: (*_a0).ptr.take(),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Owner) -> *mut Owner {
        {
            self.ptr = (*_a0).ptr.take();
        };
        return &mut (*(self as *mut Owner));
    }
}
impl Default for Owner {
    fn default() -> Self {
        unsafe { Owner::new() }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct Noncopy {
    pub n: i32,
}
impl Default for Noncopy {
    fn default() -> Self {
        unsafe { Noncopy { n: 5 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(i32, new_1, ptr)]
pub struct Held {
    pub ptr: *const i32,
}
impl Held {
    pub unsafe fn new_2() -> Self {
        let mut this = Self {
            ptr: std::ptr::null(),
        };
        this
    }
    pub unsafe fn new_1(n: *const i32) -> Self {
        let mut this = Self { ptr: (n) };
        this
    }
    pub unsafe fn new_3(n: *const [i32; 2], mut _a1: i32) -> Self {
        let mut this = Self { ptr: (*n).as_ptr() };
        this
    }
}
impl Default for Held {
    fn default() -> Self {
        unsafe { Held::new_2() }
    }
}
pub static mut global_values_7: std::cell::LazyCell<UnsafeMap<i32, (i32, i32)>> =
    std::cell::LazyCell::new(|| unsafe { UnsafeMap::new() });
pub static mut global_key_8: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut conditional_key_9: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 1 });
pub static mut owned_key_10: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 2 });
pub static mut global_a_11: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut global_b_12: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut global_choose_13: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut global_temporary_14: std::cell::LazyCell<(
    UnsafeBidirectionalIterator<(i32, i32)>,
    bool,
)> = std::cell::LazyCell::new(|| unsafe {
    {
        let mut __tmp_3: Temporary;
        match ((&mut (*{
            __tmp_3 = (unsafe { make_temporary_5() });
            &mut __tmp_3
        })
        .n as *mut i32),)
        {
            (__tmp_2,) => (*std::cell::LazyCell::force_mut(&mut *&raw mut global_values_7))
                .try_emplace(
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut global_key_8)),
                    || (*__tmp_2),
                ),
        }
    }
});
pub static mut global_conditional_15: std::cell::LazyCell<(
    UnsafeBidirectionalIterator<(i32, i32)>,
    bool,
)> = std::cell::LazyCell::new(|| unsafe {
    match (
        (if (*std::cell::LazyCell::force_mut(&mut *&raw mut global_choose_13)) {
            ({
                let __tmp_5 = &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut global_a_11));
                (*__tmp_5).prefix_inc();
                __tmp_5
            } as *mut i32)
        } else {
            ({
                let __tmp_6 = &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut global_b_12));
                (*__tmp_6).prefix_inc();
                __tmp_6
            } as *mut i32)
        } as *mut i32),
    ) {
        (__tmp_4,) => (*std::cell::LazyCell::force_mut(&mut *&raw mut global_values_7))
            .try_emplace(
                (*std::cell::LazyCell::force_mut(&mut *&raw mut conditional_key_9)),
                || (*__tmp_4),
            ),
    }
});
pub static mut global_owned_16: std::cell::LazyCell<(
    UnsafeBidirectionalIterator<(i32, i32)>,
    bool,
)> = std::cell::LazyCell::new(|| unsafe {
    {
        let mut __tmp_8: Option<Box<Temporary>>;
        match ((&mut (*(*{
            __tmp_8 = Some(Box::new(Temporary::new()));
            &mut __tmp_8
        })
        .as_deref()
        .map_or(::std::ptr::null_mut(), |v| {
            v as *const Temporary as *mut Temporary
        }))
        .n as *mut i32),)
        {
            (__tmp_7,) => (*std::cell::LazyCell::force_mut(&mut *&raw mut global_values_7))
                .try_emplace(
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut owned_key_10)),
                    || (*__tmp_7),
                ),
        }
    }
});
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut global_temporary_14)).1)
            && (((*(*std::cell::LazyCell::force_mut(&mut *&raw mut global_temporary_14))
                .0
                .iter_deref())
            .1) == (5))
    );
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut global_owned_16)).1)
            && (((*(*std::cell::LazyCell::force_mut(&mut *&raw mut global_owned_16))
                .0
                .iter_deref())
            .1) == (5))
    );
    assert!(
        ((((*std::cell::LazyCell::force_mut(&mut *&raw mut global_conditional_15)).1)
            && (((*(*std::cell::LazyCell::force_mut(&mut *&raw mut global_conditional_15))
                .0
                .iter_deref())
            .1) == (1)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut global_a_11)) == (0)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut global_b_12)) == (1))
    );
    let mut m: UnsafeMap<i32, (i32, Item)> = UnsafeMap::new();
    let mut key: i32 = 0;
    let mut counter: i32 = 0;
    m.try_emplace(key, || Item::new_2({ &mut counter }, { 3 }));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut made_0)) = 0;
    let mut hit: (UnsafeBidirectionalIterator<(i32, Item)>, bool) = match (
        ((unsafe { receiver_2(&mut m) }) as *mut UnsafeMap<i32, (i32, Item)>),
        ({
            let __tmp_11 = (unsafe { place_3(&mut counter) });
            (*__tmp_11).prefix_inc();
            __tmp_11
        } as *mut i32),
    ) {
        (__tmp_9, __tmp_10) => (*__tmp_9).try_emplace(key, || Item::new_2({ __tmp_10 }, { 2 })),
    };
    assert!(
        ((((counter) == (4)) && ((*std::cell::LazyCell::force_mut(&mut *&raw mut made_0)) == (0)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut visits_1)) == (2)))
            && (!(hit.1))
    );
    key.prefix_inc();
    let mut inserted: (UnsafeBidirectionalIterator<(i32, Item)>, bool) = match (({
        let __tmp_13 = &mut counter;
        (*__tmp_13).prefix_inc();
        __tmp_13
    } as *mut i32),)
    {
        (__tmp_12,) => m.try_emplace(key, || Item::new_2({ __tmp_12 }, { 2 })),
    };
    assert!(
        (((counter) == (7)) && ((*std::cell::LazyCell::force_mut(&mut *&raw mut made_0)) == (1)))
            && (inserted.1)
    );
    let mut assigned: (UnsafeBidirectionalIterator<(i32, Item)>, bool) =
        match ((match ({ 9 }, (&mut counter as *mut i32)) {
            (__tmp_16, __tmp_15) => {
                __tmp_15.write(__tmp_16);
                __tmp_15
            }
        } as *mut i32),)
        {
            (__tmp_14,) => m.try_emplace(key, || Item::new_2({ __tmp_14 }, { 2 })),
        };
    assert!(
        (((counter) == (9)) && ((*std::cell::LazyCell::force_mut(&mut *&raw mut made_0)) == (1)))
            && (!(assigned.1))
    );
    key.prefix_inc();
    let mut compound: (UnsafeBidirectionalIterator<(i32, Item)>, bool) =
        match ((match ({ 4 }, (&mut counter as *mut i32)) {
            (__tmp_19, __tmp_18) => {
                __tmp_18.write((((*__tmp_18) as i32) + (__tmp_19 as i32)) as i32);
                __tmp_18
            }
        } as *mut i32),)
        {
            (__tmp_17,) => m.try_emplace(key, || Item::new_2({ __tmp_17 }, { 2 })),
        };
    assert!(
        (((counter) == (15)) && ((*std::cell::LazyCell::force_mut(&mut *&raw mut made_0)) == (2)))
            && (compound.1)
    );
    assert!(
        (((*inserted.0.iter_deref()).1.value) == (7))
            && (((*compound.0.iter_deref()).1.value) == (15))
    );
    let mut array: [i32; 2] = [0, 0];
    let mut index: i32 = 0;
    key.prefix_inc();
    match (({
        let __tmp_21 = &mut array[(index.postfix_inc()) as usize];
        (*__tmp_21).prefix_inc();
        __tmp_21
    } as *mut i32),)
    {
        (__tmp_20,) => m.try_emplace(key, || Item::new_2({ __tmp_20 }, { 3 })),
    };
    assert!(
        (((index) == (1)) && ((array[(0) as usize]) == (4)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut made_0)) == (3))
    );
    key.prefix_inc();
    match ((match (
        { 5 },
        (&mut array[(index.postfix_inc()) as usize] as *mut i32),
    ) {
        (__tmp_24, __tmp_23) => {
            __tmp_23.write(__tmp_24);
            __tmp_23
        }
    } as *mut i32),)
    {
        (__tmp_22,) => m.try_emplace(key, || Item::new_2({ __tmp_22 }, { 3 })),
    };
    assert!(
        (((index) == (2)) && ((array[(1) as usize]) == (8)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut made_0)) == (4))
    );
    match ((match ({ ((counter) + (1)) }, (&mut counter as *mut i32)) {
        (__tmp_27, __tmp_26) => {
            __tmp_26.write(__tmp_27);
            __tmp_26
        }
    } as *mut i32),)
    {
        (__tmp_25,) => m.try_emplace(key, || Item::new_2({ __tmp_25 }, { 3 })),
    };
    assert!(
        ((counter) == (16)) && ((*std::cell::LazyCell::force_mut(&mut *&raw mut made_0)) == (4))
    );
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    let mut choose: bool = false;
    match ((if choose {
        ({
            let __tmp_29 = &mut a;
            (*__tmp_29).prefix_inc();
            __tmp_29
        } as *mut i32)
    } else {
        ({
            let __tmp_30 = &mut b;
            (*__tmp_30).prefix_inc();
            __tmp_30
        } as *mut i32)
    } as *mut i32),)
    {
        (__tmp_28,) => m.try_emplace(key, || Item::new_2({ __tmp_28 }, { 3 })),
    };
    assert!(
        (((a) == (0)) && ((b) == (1)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut made_0)) == (4))
    );
    choose = true;
    match ((if choose {
        ({
            let __tmp_32 = &mut a;
            (*__tmp_32).prefix_inc();
            __tmp_32
        } as *mut i32)
    } else {
        ({
            let __tmp_33 = &mut b;
            (*__tmp_33).prefix_inc();
            __tmp_33
        } as *mut i32)
    } as *mut i32),)
    {
        (__tmp_31,) => m.try_emplace(key, || Item::new_2({ __tmp_31 }, { 3 })),
    };
    assert!(
        (((a) == (1)) && ((b) == (1)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut made_0)) == (4))
    );
    match (({
        counter.postfix_inc();
        &mut counter
    } as *mut i32),)
    {
        (__tmp_34,) => m.try_emplace(key, || Item::new_2({ __tmp_34 }, { 3 })),
    };
    assert!(
        ((counter) == (17)) && ((*std::cell::LazyCell::force_mut(&mut *&raw mut made_0)) == (4))
    );
    let mut unsigned_map: UnsafeMap<i32, (i32, u32)> = UnsafeMap::new();
    let mut n: u32 = !0_u32;
    let mut wrapped: (UnsafeBidirectionalIterator<(i32, u32)>, bool) =
        match ((match ({ 1_u32 }, (&mut n as *mut u32)) {
            (__tmp_37, __tmp_36) => {
                __tmp_36.write((((*__tmp_36) as u32).wrapping_add((__tmp_37 as u32))) as u32);
                __tmp_36
            }
        } as *mut u32),)
        {
            (__tmp_35,) => unsigned_map.try_emplace(key, || (*__tmp_35)),
        };
    assert!(((n) == (0_u32)) && (((*wrapped.0.iter_deref()).1) == (0_u32)));
    let mut bytes: [u8; 2] = [255_u8, 0_u8];
    index = 0;
    key.prefix_inc();
    let mut narrowed: (UnsafeBidirectionalIterator<(i32, u32)>, bool) = match ((match (
        { 2 },
        (&mut bytes[(index.postfix_inc()) as usize] as *mut u8),
    ) {
        (__tmp_40, __tmp_39) => {
            __tmp_39.write((((*__tmp_39) as i32) + (__tmp_40 as i32)) as u8);
            __tmp_39
        }
    } as *mut u8),)
    {
        (__tmp_38,) => unsigned_map.try_emplace(key, || ((*__tmp_38) as u32)),
    };
    assert!(
        (((index) == (1)) && ((bytes[(0) as usize] as i32) == (1)))
            && (((*narrowed.0.iter_deref()).1) == (1_u32))
    );
    let mut changed: UnsafeMap<i32, (i32, i32)> = UnsafeMap::new();
    (*changed.subscript(&key, || 0_i32)) = 10;
    let mut effects: i32 = 0;
    let mut erased: (UnsafeBidirectionalIterator<(i32, i32)>, bool) = match (({
        changed.erase_key(&key);
        {
            let __tmp_42 = &mut effects;
            (*__tmp_42).prefix_inc();
            __tmp_42
        }
    } as *mut i32),)
    {
        (__tmp_41,) => changed.try_emplace(key, || (*__tmp_41)),
    };
    assert!((((effects) == (1)) && (erased.1)) && (((*erased.0.iter_deref()).1) == (1)));
    let mut updated: (UnsafeBidirectionalIterator<(i32, i32)>, bool) = match (({
        (*changed.subscript(&key, || 0_i32)).prefix_inc();
        {
            let __tmp_44 = &mut effects;
            (*__tmp_44).prefix_inc();
            __tmp_44
        }
    } as *mut i32),)
    {
        (__tmp_43,) => changed.try_emplace(key, || (*__tmp_43)),
    };
    assert!((((effects) == (2)) && (!(updated.1))) && (((*updated.0.iter_deref()).1) == (2)));
    let mut pointers: UnsafeMap<i32, (i32, *mut i32)> = UnsafeMap::new();
    let mut p: *mut i32 = array.as_mut_ptr();
    let mut pointer: (UnsafeBidirectionalIterator<(i32, *mut i32)>, bool) =
        match ((match ({ 1 }, (&mut p as *mut *mut i32)) {
            (__tmp_47, __tmp_46) => {
                __tmp_46.write((*__tmp_46).offset((__tmp_47 as isize)));
                __tmp_46
            }
        } as *mut *mut i32),)
        {
            (__tmp_45,) => pointers.try_emplace(key, || (*__tmp_45)),
        };
    assert!(
        ((p) == (array.as_mut_ptr().offset((1) as isize)))
            && (((*pointer.0.iter_deref()).1) == (p))
    );
    {
        let mut hit: i32 = 0;
        'loop_: while ((hit) < (2)) {
            let mut values: UnsafeMap<i32, (i32, i32)> = UnsafeMap::new();
            let keys: [i32; 6] = [0, 1, 2, 3, 4, 5];
            if (hit != 0) {
                'loop_: for k in
                    IterRange::new(keys.as_ptr(), keys.as_ptr().offset((6_i64) as isize))
                {
                    let mut k: i32 = (*k);
                    (*values.subscript(&k, || 0_i32)) = 42;
                }
            }
            let mut moved: i32 = 0;
            let mut forwarded: i32 = 0;
            let mut xvalue: i32 = 0;
            let mut lvalue: i32 = 0;
            let mut assigned: i32 = 0;
            let mut nested: i32 = 0;
            let mut r0: (UnsafeBidirectionalIterator<(i32, i32)>, bool) = match (({
                let __tmp_49 = &mut moved;
                (*__tmp_49).prefix_inc();
                __tmp_49
            }
                as *mut i32),)
            {
                (__tmp_48,) => values.try_emplace(keys[(0) as usize], || (*__tmp_48)),
            };
            let mut r1: (UnsafeBidirectionalIterator<(i32, i32)>, bool) = match (({
                let __tmp_51 = &mut forwarded;
                (*__tmp_51).prefix_inc();
                __tmp_51
            }
                as *mut i32),)
            {
                (__tmp_50,) => values.try_emplace(keys[(1) as usize], || (*__tmp_50)),
            };
            let mut r2: (UnsafeBidirectionalIterator<(i32, i32)>, bool) = match (({
                let __tmp_53 = &mut xvalue;
                (*__tmp_53).prefix_inc();
                __tmp_53
            }
                as *mut i32),)
            {
                (__tmp_52,) => values.try_emplace(keys[(2) as usize], || (*__tmp_52)),
            };
            let mut r3: (UnsafeBidirectionalIterator<(i32, i32)>, bool) = match (({
                let __tmp_55 = &mut lvalue;
                (*__tmp_55).prefix_inc();
                __tmp_55
            }
                as *mut i32),)
            {
                (__tmp_54,) => values.try_emplace(keys[(3) as usize], || (*__tmp_54)),
            };
            let mut r4: (UnsafeBidirectionalIterator<(i32, i32)>, bool) =
                match ((match ({ 5 }, (&mut assigned as *mut i32)) {
                    (__tmp_58, __tmp_57) => {
                        __tmp_57.write(__tmp_58);
                        __tmp_57
                    }
                } as *mut i32),)
                {
                    (__tmp_56,) => values.try_emplace(keys[(4) as usize], || (*__tmp_56)),
                };
            let mut r5: (UnsafeBidirectionalIterator<(i32, i32)>, bool) =
                match ((match ({ 1 }, (&mut nested as *mut i32)) {
                    (__tmp_61, __tmp_60) => {
                        __tmp_60.write((((*__tmp_60) as i32) + (__tmp_61 as i32)) as i32);
                        __tmp_60
                    }
                } as *mut i32),)
                {
                    (__tmp_59,) => values.try_emplace(keys[(5) as usize], || (*__tmp_59)),
                };
            assert!(
                ((((((moved) == (1)) && ((forwarded) == (1))) && ((xvalue) == (1)))
                    && ((lvalue) == (1)))
                    && ((assigned) == (5)))
                    && ((nested) == (1))
            );
            assert!(
                (((((((r0.1) as i32) == (!(hit != 0) as i32))
                    && (((r1.1) as i32) == (!(hit != 0) as i32)))
                    && (((r2.1) as i32) == (!(hit != 0) as i32)))
                    && (((r3.1) as i32) == (!(hit != 0) as i32)))
                    && (((r4.1) as i32) == (!(hit != 0) as i32)))
                    && (((r5.1) as i32) == (!(hit != 0) as i32))
            );
            'loop_: for k in IterRange::new(keys.as_ptr(), keys.as_ptr().offset((6_i64) as isize)) {
                let mut k: i32 = (*k);
                assert!(
                    ((*values.subscript(&k, || 0_i32))
                        == (if (hit != 0) {
                            42
                        } else {
                            if ((k) == (4)) { 5 } else { 1 }
                        }))
                );
            }
            hit.prefix_inc();
        }
    }
    {
        let mut hit: i32 = 0;
        'loop_: while ((hit) < (2)) {
            {
                let mut choose: i32 = 0;
                'loop_: while ((choose) < (2)) {
                    let mut selected: UnsafeMap<i32, (i32, Item)> = UnsafeMap::new();
                    let keys: [i32; 3] = [0, 1, 2];
                    if (hit != 0) {
                        'loop_: for k in
                            IterRange::new(keys.as_ptr(), keys.as_ptr().offset((3_i64) as isize))
                        {
                            let mut k: i32 = (*k);
                            (*selected.subscript(&k, || <Item>::default())).value = 42;
                        }
                    }
                    let mut a: i32 = 0;
                    let mut b: i32 = 0;
                    let mut c: i32 = 0;
                    let mut d: i32 = 0;
                    let mut e: i32 = 0;
                    let mut f: i32 = 0;
                    let mut prefix: (UnsafeBidirectionalIterator<(i32, Item)>, bool) =
                        match ((if (choose != 0) {
                            ({
                                let __tmp_63 = &mut a;
                                (*__tmp_63).prefix_inc();
                                __tmp_63
                            } as *mut i32)
                        } else {
                            ({
                                let __tmp_64 = &mut b;
                                (*__tmp_64).prefix_inc();
                                __tmp_64
                            } as *mut i32)
                        } as *mut i32),)
                        {
                            (__tmp_62,) => selected.try_emplace(keys[(0) as usize], || {
                                Item::new_2({ __tmp_62 }, { 2 })
                            }),
                        };
                    let mut assignment: (UnsafeBidirectionalIterator<(i32, Item)>, bool) =
                        match ((if (choose != 0) {
                            ((match ({ 5 }, (&mut c as *mut i32)) {
                                (__tmp_67, __tmp_66) => {
                                    __tmp_66.write(__tmp_67);
                                    __tmp_66
                                }
                            }) as *mut i32)
                        } else {
                            ((match ({ 6 }, (&mut d as *mut i32)) {
                                (__tmp_69, __tmp_68) => {
                                    __tmp_68.write(__tmp_69);
                                    __tmp_68
                                }
                            }) as *mut i32)
                        } as *mut i32),)
                        {
                            (__tmp_65,) => selected.try_emplace(keys[(1) as usize], || {
                                Item::new_2({ __tmp_65 }, { 2 })
                            }),
                        };
                    let mut compound: (UnsafeBidirectionalIterator<(i32, Item)>, bool) =
                        match ((if (choose != 0) {
                            ((match ({ 1 }, (&mut e as *mut i32)) {
                                (__tmp_72, __tmp_71) => {
                                    __tmp_71
                                        .write((((*__tmp_71) as i32) + (__tmp_72 as i32)) as i32);
                                    __tmp_71
                                }
                            }) as *mut i32)
                        } else {
                            ((match ({ 2 }, (&mut f as *mut i32)) {
                                (__tmp_74, __tmp_73) => {
                                    __tmp_73
                                        .write((((*__tmp_73) as i32) + (__tmp_74 as i32)) as i32);
                                    __tmp_73
                                }
                            }) as *mut i32)
                        } as *mut i32),)
                        {
                            (__tmp_70,) => selected.try_emplace(keys[(2) as usize], || {
                                Item::new_2({ __tmp_70 }, { 2 })
                            }),
                        };
                    assert!(
                        ((a) == (if (choose != 0) {
                            (if (hit != 0) { 1 } else { 3 })
                        } else {
                            0
                        })) && ((b)
                            == (if (choose != 0) {
                                0
                            } else {
                                (if (hit != 0) { 1 } else { 3 })
                            }))
                    );
                    assert!(
                        ((c) == (if (choose != 0) {
                            (if (hit != 0) { 5 } else { 7 })
                        } else {
                            0
                        })) && ((d)
                            == (if (choose != 0) {
                                0
                            } else {
                                (if (hit != 0) { 6 } else { 8 })
                            }))
                    );
                    assert!(
                        ((e) == (if (choose != 0) {
                            (if (hit != 0) { 1 } else { 3 })
                        } else {
                            0
                        })) && ((f)
                            == (if (choose != 0) {
                                0
                            } else {
                                (if (hit != 0) { 2 } else { 4 })
                            }))
                    );
                    assert!(
                        ((((prefix.1) as i32) == (!(hit != 0) as i32))
                            && (((assignment.1) as i32) == (!(hit != 0) as i32)))
                            && (((compound.1) as i32) == (!(hit != 0) as i32))
                    );
                    assert!(
                        (((*prefix.0.iter_deref()).1.value) == (if (hit != 0) { 42 } else { 3 }))
                    );
                    assert!(
                        (((*assignment.0.iter_deref()).1.value)
                            == (if (hit != 0) {
                                42
                            } else {
                                if (choose != 0) { 7 } else { 8 }
                            }))
                    );
                    assert!(
                        (((*compound.0.iter_deref()).1.value)
                            == (if (hit != 0) {
                                42
                            } else {
                                if (choose != 0) { 3 } else { 4 }
                            }))
                    );
                    choose.prefix_inc();
                }
            }
            let mut values: UnsafeMap<i32, (i32, i32)> = UnsafeMap::new();
            let keys: [i32; 3] = [0, 1, 2];
            if (hit != 0) {
                'loop_: for k in
                    IterRange::new(keys.as_ptr(), keys.as_ptr().offset((3_i64) as isize))
                {
                    let mut k: i32 = (*k);
                    (*values.subscript(&k, || 0_i32)) = 42;
                }
            }
            let mut before: i32 =
                (*std::cell::LazyCell::force_mut(&mut *&raw mut temporary_calls_4));
            let mut index: i32 = 0;
            let mut member: (UnsafeBidirectionalIterator<(i32, i32)>, bool) = {
                let mut __tmp_76: Temporary;
                match ((&mut (*{
                    __tmp_76 = (unsafe { make_temporary_5() });
                    &mut __tmp_76
                })
                .n as *mut i32),)
                {
                    (__tmp_75,) => values.try_emplace(keys[(0) as usize], || (*__tmp_75)),
                }
            };
            let mut nested: (UnsafeBidirectionalIterator<(i32, i32)>, bool) = {
                let mut __tmp_78: Nested;
                match ((&mut (*&mut (*{
                    __tmp_78 = (unsafe { make_nested_6() });
                    &mut __tmp_78
                })
                .inner)
                    .n as *mut i32),)
                {
                    (__tmp_77,) => values.try_emplace(keys[(1) as usize], || (*__tmp_77)),
                }
            };
            let mut element: (UnsafeBidirectionalIterator<(i32, i32)>, bool) = {
                let mut __tmp_80: Temporary;
                match ((&mut (*{
                    __tmp_80 = (unsafe { make_temporary_5() });
                    &mut __tmp_80
                })
                .data[(index.postfix_inc()) as usize] as *mut i32),)
                {
                    (__tmp_79,) => values.try_emplace(keys[(2) as usize], || (*__tmp_79)),
                }
            };
            assert!(
                ((*std::cell::LazyCell::force_mut(&mut *&raw mut temporary_calls_4))
                    == ((before) + (3)))
                    && ((index) == (1))
            );
            assert!(
                ((((member.1) as i32) == (!(hit != 0) as i32))
                    && (((nested.1) as i32) == (!(hit != 0) as i32)))
                    && (((element.1) as i32) == (!(hit != 0) as i32))
            );
            assert!(
                ((((*member.0.iter_deref()).1) == (if (hit != 0) { 42 } else { 5 }))
                    && (((*nested.0.iter_deref()).1) == (if (hit != 0) { 42 } else { 5 })))
                    && (((*element.0.iter_deref()).1) == (if (hit != 0) { 42 } else { 6 }))
            );
            {
                let mut choose: i32 = 0;
                'loop_: while ((choose) < (2)) {
                    let mut references: UnsafeMap<i32, (i32, Held)> = UnsafeMap::new();
                    let mut ordinary: i32 = 9;
                    if (hit != 0) {
                        references.try_emplace(key, || Held::new_1({ &ordinary }));
                    }
                    before = (*std::cell::LazyCell::force_mut(&mut *&raw mut temporary_calls_4));
                    index = 0;
                    let mut value: i32 = 0;
                    {
                        let mut __tmp_82: Nested;
                        let mut __tmp_83: Temporary;
                        {
                            match ((if (choose != 0) {
                                (&mut (*&mut (*{
                                    __tmp_82 = (unsafe { make_nested_6() });
                                    &mut __tmp_82
                                })
                                .inner)
                                    .n as *mut i32)
                            } else {
                                (&mut (*{
                                    __tmp_83 = (unsafe { make_temporary_5() });
                                    &mut __tmp_83
                                })
                                .data[(index.postfix_inc()) as usize]
                                    as *mut i32)
                            } as *mut i32),)
                            {
                                (__tmp_81,) => {
                                    references.try_emplace(key, || Held::new_1({ __tmp_81 }))
                                }
                            };
                            value = (*(*references.subscript(&key, || <Held>::default())).ptr)
                        }
                    };
                    assert!(
                        ((*std::cell::LazyCell::force_mut(&mut *&raw mut temporary_calls_4))
                            == ((before) + (1)))
                            && ((index) == (if (choose != 0) { 0 } else { 1 }))
                    );
                    assert!(
                        ((value)
                            == (if (hit != 0) {
                                9
                            } else {
                                if (choose != 0) { 5 } else { 6 }
                            }))
                    );
                    let mut arrays: UnsafeMap<i32, (i32, Held)> = UnsafeMap::new();
                    let ordinary_array: [i32; 2] = [9, 10];
                    if (hit != 0) {
                        arrays.try_emplace(key, || Held::new_3({ &ordinary_array }, { 0 }));
                    }
                    before = (*std::cell::LazyCell::force_mut(&mut *&raw mut temporary_calls_4));
                    {
                        let mut __tmp_85: Nested;
                        let mut __tmp_86: Temporary;
                        {
                            match ((if (choose != 0) {
                                (&mut (*&mut (*{
                                    __tmp_85 = (unsafe { make_nested_6() });
                                    &mut __tmp_85
                                })
                                .inner)
                                    .data as *mut [i32; 2])
                            } else {
                                (&mut (*{
                                    __tmp_86 = (unsafe { make_temporary_5() });
                                    &mut __tmp_86
                                })
                                .data as *mut [i32; 2])
                            } as *mut [i32; 2]),)
                            {
                                (__tmp_84,) => {
                                    arrays.try_emplace(key, || Held::new_3({ __tmp_84 }, { 0 }))
                                }
                            };
                            value = (*(*arrays.subscript(&key, || <Held>::default()))
                                .ptr
                                .offset((1) as isize))
                        }
                    };
                    assert!(
                        ((*std::cell::LazyCell::force_mut(&mut *&raw mut temporary_calls_4))
                            == ((before) + (1)))
                            && ((value) == (if (hit != 0) { 10 } else { 7 }))
                    );
                    choose.prefix_inc();
                }
            }
            hit.prefix_inc();
        }
    }
    {
        let mut hit: i32 = 0;
        'loop_: while ((hit) < (2)) {
            let mut values: UnsafeMap<i32, (i32, i32)> = UnsafeMap::new();
            let keys: [i32; 6] = [0, 1, 2, 3, 4, 5];
            if (hit != 0) {
                'loop_: for k in
                    IterRange::new(keys.as_ptr(), keys.as_ptr().offset((6_i64) as isize))
                {
                    let mut k: i32 = (*k);
                    (*values.subscript(&k, || 0_i32)) = 42;
                }
            }
            let mut before: i32 =
                (*std::cell::LazyCell::force_mut(&mut *&raw mut temporary_calls_4));
            let mut index: i32 = 0;
            let mut arrow: (UnsafeBidirectionalIterator<(i32, i32)>, bool) = {
                let mut __tmp_88: Option<Box<Temporary>>;
                match ((&mut (*(*{
                    __tmp_88 = Some(Box::new(Temporary::new()));
                    &mut __tmp_88
                })
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| {
                    v as *const Temporary as *mut Temporary
                }))
                .n as *mut i32),)
                {
                    (__tmp_87,) => values.try_emplace(keys[(0) as usize], || (*__tmp_87)),
                }
            };
            let mut nested: (UnsafeBidirectionalIterator<(i32, i32)>, bool) = {
                let mut __tmp_90: Option<Box<Nested>>;
                match ((&mut (*&mut (*(*{
                    __tmp_90 = Some(Box::new(<Nested>::default()));
                    &mut __tmp_90
                })
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| {
                    v as *const Nested as *mut Nested
                }))
                .inner)
                    .n as *mut i32),)
                {
                    (__tmp_89,) => values.try_emplace(keys[(1) as usize], || (*__tmp_89)),
                }
            };
            let mut element: (UnsafeBidirectionalIterator<(i32, i32)>, bool) = {
                let mut __tmp_92: Option<Box<Temporary>>;
                match ((&mut (*(*{
                    __tmp_92 = Some(Box::new(Temporary::new()));
                    &mut __tmp_92
                })
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| {
                    v as *const Temporary as *mut Temporary
                }))
                .data[(index.postfix_inc()) as usize] as *mut i32),)
                {
                    (__tmp_91,) => values.try_emplace(keys[(2) as usize], || (*__tmp_91)),
                }
            };
            let mut get: (UnsafeBidirectionalIterator<(i32, i32)>, bool) = {
                let mut __tmp_94: Option<Box<Temporary>>;
                match ((&mut (*(*{
                    __tmp_94 = Some(Box::new(Temporary::new()));
                    &mut __tmp_94
                })
                .as_deref_mut()
                .map_or(::std::ptr::null_mut(), |v| v as *mut Temporary))
                .n as *mut i32),)
                {
                    (__tmp_93,) => values.try_emplace(keys[(3) as usize], || (*__tmp_93)),
                }
            };
            let mut deref: (UnsafeBidirectionalIterator<(i32, i32)>, bool) = {
                let mut __tmp_96: Option<Box<Temporary>>;
                match ((&mut (*((*{
                    __tmp_96 = Some(Box::new(Temporary::new()));
                    &mut __tmp_96
                })
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| {
                    v as *const Temporary as *mut Temporary
                })))
                .n as *mut i32),)
                {
                    (__tmp_95,) => values.try_emplace(keys[(4) as usize], || (*__tmp_95)),
                }
            };
            let mut indirect: (UnsafeBidirectionalIterator<(i32, i32)>, bool) = {
                let mut __tmp_98: Option<Box<Option<Box<Temporary>>>>;
                match ((&mut (*((*(*{
                    __tmp_98 = Some(Box::new(Some(Box::new(Temporary::new()))));
                    &mut __tmp_98
                })
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| {
                    v as *const Option<Box<Temporary>> as *mut Option<Box<Temporary>>
                }))
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| {
                    v as *const Temporary as *mut Temporary
                })))
                .n as *mut i32),)
                {
                    (__tmp_97,) => values.try_emplace(keys[(5) as usize], || (*__tmp_97)),
                }
            };
            assert!(
                ((*std::cell::LazyCell::force_mut(&mut *&raw mut temporary_calls_4))
                    == ((before) + (6)))
                    && ((index) == (1))
            );
            assert!(
                (((((((arrow.1) as i32) == (!(hit != 0) as i32))
                    && (((nested.1) as i32) == (!(hit != 0) as i32)))
                    && (((element.1) as i32) == (!(hit != 0) as i32)))
                    && (((get.1) as i32) == (!(hit != 0) as i32)))
                    && (((deref.1) as i32) == (!(hit != 0) as i32)))
                    && (((indirect.1) as i32) == (!(hit != 0) as i32))
            );
            'loop_: for k in IterRange::new(keys.as_ptr(), keys.as_ptr().offset((6_i64) as isize)) {
                let mut k: i32 = (*k);
                assert!(
                    ((*values.subscript(&k, || 0_i32))
                        == (if (hit != 0) {
                            42
                        } else {
                            if ((k) == (2)) { 6 } else { 5 }
                        }))
                );
            }
            {
                let mut choose: i32 = 0;
                'loop_: while ((choose) < (2)) {
                    let mut references: UnsafeMap<i32, (i32, Held)> = UnsafeMap::new();
                    let mut ordinary: i32 = 9;
                    if (hit != 0) {
                        references.try_emplace(key, || Held::new_1({ &ordinary }));
                    }
                    before = (*std::cell::LazyCell::force_mut(&mut *&raw mut temporary_calls_4));
                    index = 0;
                    let mut value: i32 = 0;
                    {
                        let mut __tmp_100: Option<Box<Temporary>>;
                        let mut __tmp_101: Option<Box<Temporary>>;
                        {
                            match ((if (choose != 0) {
                                (&mut (*(*{
                                    __tmp_100 = Some(Box::new(Temporary::new()));
                                    &mut __tmp_100
                                })
                                .as_deref()
                                .map_or(::std::ptr::null_mut(), |v| {
                                    v as *const Temporary as *mut Temporary
                                }))
                                .n as *mut i32)
                            } else {
                                (&mut (*(*{
                                    __tmp_101 = Some(Box::new(Temporary::new()));
                                    &mut __tmp_101
                                })
                                .as_deref()
                                .map_or(::std::ptr::null_mut(), |v| {
                                    v as *const Temporary as *mut Temporary
                                }))
                                .data[(index.postfix_inc()) as usize]
                                    as *mut i32)
                            } as *mut i32),)
                            {
                                (__tmp_99,) => {
                                    references.try_emplace(key, || Held::new_1({ __tmp_99 }))
                                }
                            };
                            value = (*(*references.subscript(&key, || <Held>::default())).ptr)
                        }
                    };
                    assert!(
                        ((*std::cell::LazyCell::force_mut(&mut *&raw mut temporary_calls_4))
                            == ((before) + (1)))
                            && ((index) == (if (choose != 0) { 0 } else { 1 }))
                    );
                    assert!(
                        ((value)
                            == (if (hit != 0) {
                                9
                            } else {
                                if (choose != 0) { 5 } else { 6 }
                            }))
                    );
                    let mut owners: UnsafeMap<i32, (i32, Held)> = UnsafeMap::new();
                    {
                        let mut __tmp_103: Owner;
                        {
                            match (((unsafe {
                                Owner::value(
                                    &mut (*{
                                        __tmp_103 = Owner::new();
                                        &mut __tmp_103
                                    }),
                                )
                            }) as *const i32),)
                            {
                                (__tmp_102,) => {
                                    owners.try_emplace(key, || Held::new_1({ __tmp_102 }))
                                }
                            };
                            value = (*(*owners.subscript(&key, || <Held>::default())).ptr)
                        }
                    };
                    assert!(((value) == (5)));
                    choose.prefix_inc();
                }
            }
            hit.prefix_inc();
        }
    }
    let mut move_only: UnsafeMap<i32, (i32, (Option<Box<Noncopy>>, i32))> = UnsafeMap::new();
    let mut move_new: (
        UnsafeBidirectionalIterator<(i32, (Option<Box<Noncopy>>, i32))>,
        bool,
    ) = {
        let mut __tmp_105: Option<Box<Noncopy>>;
        match (({
            __tmp_105 = Some(Box::new(<Noncopy>::default()));
            &mut __tmp_105
        } as *mut Option<Box<Noncopy>>),)
        {
            (__tmp_104,) => move_only.try_emplace(key, || ((*__tmp_104).take(), 7)),
        }
    };
    let mut move_hit: (
        UnsafeBidirectionalIterator<(i32, (Option<Box<Noncopy>>, i32))>,
        bool,
    ) = {
        let mut __tmp_107: Option<Box<Noncopy>>;
        match (({
            __tmp_107 = Some(Box::new(<Noncopy>::default()));
            &mut __tmp_107
        } as *mut Option<Box<Noncopy>>),)
        {
            (__tmp_106,) => move_only.try_emplace(key, || ((*__tmp_106).take(), 9)),
        }
    };
    assert!((move_new.1) && (!(move_hit.1)));
    assert!(
        (((*(*move_new.0.iter_deref())
            .1
            .0
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const Noncopy
                as *mut Noncopy))
        .n) == (5))
            && (((*move_hit.0.iter_deref()).1.1) == (7))
    );
    let mut strings: UnsafeMap<i32, (i32, Vec<libc::c_char>)> = UnsafeMap::new();
    let mut text: Vec<libc::c_char> = std::slice::from_raw_parts(
        c"value".as_ptr(),
        (0..)
            .take_while(|&i| *c"value".as_ptr().add(i) != 0)
            .count()
            + 1,
    )
    .to_vec();
    strings.try_emplace(key, || {
        std::slice::from_raw_parts(
            c"original".as_ptr(),
            (0..)
                .take_while(|&i| *c"original".as_ptr().add(i) != 0)
                .count()
                + 1,
        )
        .to_vec()
    });
    let mut string_hit: (UnsafeBidirectionalIterator<(i32, Vec<libc::c_char>)>, bool) = match (({
        (*std::cell::LazyCell::force_mut(&mut *&raw mut visits_1)).prefix_inc();
        &mut text
    }
        as *mut Vec<libc::c_char>),)
    {
        (__tmp_108,) => strings.try_emplace(key, || std::mem::replace(&mut (*__tmp_108), vec![0])),
    };
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut visits_1)) == (3)) && (!(string_hit.1)))
            && (text == {
                std::slice::from_raw_parts(
                    c"value".as_ptr(),
                    (0..)
                        .take_while(|&i| *c"value".as_ptr().add(i) != 0)
                        .count()
                        + 1,
                )
                .to_vec()
            })
    );
    key.prefix_inc();
    let mut string_new: (UnsafeBidirectionalIterator<(i32, Vec<libc::c_char>)>, bool) = match (({
        (*std::cell::LazyCell::force_mut(&mut *&raw mut visits_1)).prefix_inc();
        &mut text
    }
        as *mut Vec<libc::c_char>),)
    {
        (__tmp_109,) => strings.try_emplace(key, || std::mem::replace(&mut (*__tmp_109), vec![0])),
    };
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut visits_1)) == (4)) && (string_new.1))
            && ((*string_new.0.iter_deref()).1 == {
                std::slice::from_raw_parts(
                    c"value".as_ptr(),
                    (0..)
                        .take_while(|&i| *c"value".as_ptr().add(i) != 0)
                        .count()
                        + 1,
                )
                .to_vec()
            })
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const made_0);
    std::cell::LazyCell::force(&*&raw const visits_1);
    std::cell::LazyCell::force(&*&raw const temporary_calls_4);
    std::cell::LazyCell::force(&*&raw const global_values_7);
    std::cell::LazyCell::force(&*&raw const global_key_8);
    std::cell::LazyCell::force(&*&raw const conditional_key_9);
    std::cell::LazyCell::force(&*&raw const owned_key_10);
    std::cell::LazyCell::force(&*&raw const global_a_11);
    std::cell::LazyCell::force(&*&raw const global_b_12);
    std::cell::LazyCell::force(&*&raw const global_choose_13);
    std::cell::LazyCell::force(&*&raw const global_temporary_14);
    std::cell::LazyCell::force(&*&raw const global_conditional_15);
    std::cell::LazyCell::force(&*&raw const global_owned_16);
}
