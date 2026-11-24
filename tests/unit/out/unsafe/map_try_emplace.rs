extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut calls_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut made_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut copies_2: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut moves_3: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe)]
#[cpp_from(i32, new_1, value)]
pub struct Item {
    pub value: i32,
}
impl Item {
    pub unsafe fn new_2() -> Self {
        let mut this = Self { value: 0 };
        this
    }
    pub unsafe fn new_1(mut value: i32) -> Self {
        let mut this = Self { value: value };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut made_1)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(other: *const Item) -> Self {
        let mut this = Self {
            value: (*other).value,
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_2)).prefix_inc();
        this
    }
    pub unsafe fn move_from(other: *mut Item) -> Self {
        let mut this = Self {
            value: (*other).value,
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_3)).prefix_inc();
        (*other).value = -1_i32;
        this
    }
}
impl Clone for Item {
    fn clone(&self) -> Self {
        unsafe { Item::copy_from(self as *const Item) }
    }
}
impl Default for Item {
    fn default() -> Self {
        unsafe { Item::new_2() }
    }
}
pub unsafe fn argument_4(item: *mut Item) -> *mut Item {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut calls_0)).prefix_inc();
    return item;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct RefItem {
    pub value: i32,
}
impl RefItem {
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
        (*std::cell::LazyCell::force_mut(&mut *&raw mut made_1)).prefix_inc();
        this
    }
}
impl Default for RefItem {
    fn default() -> Self {
        unsafe { RefItem::new_1() }
    }
}
pub unsafe fn reference_5(counter: *mut i32) -> *mut i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut calls_0)).prefix_inc();
    return counter;
}
pub static mut global_map_6: std::cell::LazyCell<UnsafeMap<i32, (i32, i32)>> =
    std::cell::LazyCell::new(|| unsafe { UnsafeMap::new() });
pub static mut global_key_7: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut global_counter_8: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut seed_9: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe {
    {
        (*(*std::cell::LazyCell::force_mut(&mut *&raw mut global_map_6)).subscript(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut global_key_7)),
            || 0_i32,
        )) = 10;
        0
    }
});
pub static mut global_hit_10: std::cell::LazyCell<(UnsafeBidirectionalIterator<(i32, i32)>, bool)> =
    std::cell::LazyCell::new(|| unsafe {
        match (({
            let __tmp_3 = &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut global_counter_8));
            (*__tmp_3).prefix_inc();
            __tmp_3
        } as *mut i32),)
        {
            (__tmp_2,) => (*std::cell::LazyCell::force_mut(&mut *&raw mut global_map_6))
                .try_emplace(
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut global_key_7)),
                    || (*__tmp_2),
                ),
        }
    });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut ok: bool = (((*std::cell::LazyCell::force_mut(&mut *&raw mut global_counter_8))
        == (1))
        && (!((*std::cell::LazyCell::force_mut(&mut *&raw mut global_hit_10)).1)))
        && (((*(*std::cell::LazyCell::force_mut(&mut *&raw mut global_hit_10))
            .0
            .iter_deref())
        .1) == (10));
    let mut rows: i32 = 0;
    {
        let mut subset: i32 = 0;
        'loop_: while ((subset) < (8)) {
            {
                let mut key: i32 = 0;
                'loop_: while ((key) < (3)) {
                    {
                        let mut initial: i32 = 0;
                        'loop_: while ((initial) < (3)) {
                            {
                                let mut post: i32 = 0;
                                'loop_: while ((post) < (2)) {
                                    let mut m: UnsafeMap<i32, (i32, i32)> = UnsafeMap::new();
                                    let mut size: i32 = 0;
                                    {
                                        let mut k: i32 = 0;
                                        'loop_: while ((k) < (3)) {
                                            if (((subset) & ((1) << (k))) != 0) {
                                                (*m.subscript(&k, || 0_i32)) = ((10) + (k));
                                                size.prefix_inc();
                                            }
                                            k.prefix_inc();
                                        }
                                    }
                                    let mut counter: i32 = initial;
                                    let mut result: (
                                        UnsafeBidirectionalIterator<(i32, i32)>,
                                        bool,
                                    ) = {
                                        let mut __tmp_5: i32;
                                        if (post != 0) {
                                            match (({
                                                __tmp_5 = counter.postfix_inc();
                                                &mut __tmp_5
                                            }
                                                as *mut i32),)
                                            {
                                                (__tmp_4,) => m.try_emplace(key, || (*__tmp_4)),
                                            }
                                        } else {
                                            match (({
                                                let __tmp_7 = &mut counter;
                                                (*__tmp_7).prefix_inc();
                                                __tmp_7
                                            }
                                                as *mut i32),)
                                            {
                                                (__tmp_6,) => m.try_emplace(key, || (*__tmp_6)),
                                            }
                                        }
                                    };
                                    let mut hit: bool = (((subset) & ((1) << (key))) != (0));
                                    match ({ (((counter) == ((initial) + (1))) as i32) }, &mut (ok))
                                    {
                                        (__tmp_8, __tmp_9) => {
                                            (*__tmp_9) =
                                                (((*__tmp_9) as i32) & (__tmp_8 as i32)) != 0;
                                        }
                                    };
                                    match (
                                        { ((((result.1) as i32) == (!(hit) as i32)) as i32) },
                                        &mut (ok),
                                    ) {
                                        (__tmp_10, __tmp_11) => {
                                            (*__tmp_11) =
                                                (((*__tmp_11) as i32) & (__tmp_10 as i32)) != 0;
                                        }
                                    };
                                    let mut found: bool = result.0 == m.find(&key);
                                    match ({ (found as i32) }, &mut (ok)) {
                                        (__tmp_12, __tmp_13) => {
                                            (*__tmp_13) =
                                                (((*__tmp_13) as i32) & (__tmp_12 as i32)) != 0;
                                        }
                                    };
                                    match (
                                        { ((((*result.0.iter_deref()).0) == (key)) as i32) },
                                        &mut (ok),
                                    ) {
                                        (__tmp_14, __tmp_15) => {
                                            (*__tmp_15) =
                                                (((*__tmp_15) as i32) & (__tmp_14 as i32)) != 0;
                                        }
                                    };
                                    match (
                                        {
                                            ((((*result.0.iter_deref()).1)
                                                == (if hit {
                                                    ((10) + (key))
                                                } else {
                                                    ((initial) + (!(post != 0) as i32))
                                                }))
                                                as i32)
                                        },
                                        &mut (ok),
                                    ) {
                                        (__tmp_16, __tmp_17) => {
                                            (*__tmp_17) =
                                                (((*__tmp_17) as i32) & (__tmp_16 as i32)) != 0;
                                        }
                                    };
                                    match (
                                        {
                                            (((m.len()) == (((size) + (!(hit) as i32)) as usize))
                                                as i32)
                                        },
                                        &mut (ok),
                                    ) {
                                        (__tmp_18, __tmp_19) => {
                                            (*__tmp_19) =
                                                (((*__tmp_19) as i32) & (__tmp_18 as i32)) != 0;
                                        }
                                    };
                                    rows.prefix_inc();
                                    post.prefix_inc();
                                }
                            }
                            initial.prefix_inc();
                        }
                    }
                    key.prefix_inc();
                }
            }
            subset.prefix_inc();
        }
    }
    let mut items: UnsafeMap<i32, (i32, Item)> = UnsafeMap::new();
    let mut key: i32 = 0;
    items.try_emplace(key, || Item::new_1({ 10 }));
    let mut source: Item = Item::new_1({ 20 });
    (*std::cell::LazyCell::force_mut(&mut *&raw mut made_1)) = match (
        {
            match (
                { 0 },
                &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_3)),
            ) {
                (__tmp_22, __tmp_23) => {
                    (*__tmp_23) = __tmp_22;
                    (*__tmp_23).clone()
                }
            }
        },
        &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_2)),
    ) {
        (__tmp_20, __tmp_21) => {
            (*__tmp_21) = __tmp_20;
            (*__tmp_21).clone()
        }
    };
    let mut hit: (UnsafeBidirectionalIterator<(i32, Item)>, bool) =
        match (((unsafe { argument_4(&mut source) }) as *mut Item),) {
            (__tmp_24,) => items.try_emplace(key, || Item::copy_from({ __tmp_24 })),
        };
    match (
        {
            ((((((*std::cell::LazyCell::force_mut(&mut *&raw mut calls_0)) == (1))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut made_1)) == (0)))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_2)) == (0)))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_3)) == (0)))
                as i32)
        },
        &mut (ok),
    ) {
        (__tmp_25, __tmp_26) => {
            (*__tmp_26) = (((*__tmp_26) as i32) & (__tmp_25 as i32)) != 0;
        }
    };
    match (
        { (((!(hit.1)) && (((*hit.0.iter_deref()).1.value) == (10))) as i32) },
        &mut (ok),
    ) {
        (__tmp_27, __tmp_28) => {
            (*__tmp_28) = (((*__tmp_28) as i32) & (__tmp_27 as i32)) != 0;
        }
    };
    let mut move_hit: (UnsafeBidirectionalIterator<(i32, Item)>, bool) =
        match (((unsafe { argument_4(&mut source) }) as *mut Item),) {
            (__tmp_29,) => items.try_emplace(key, || Item::move_from({ __tmp_29 })),
        };
    match (
        {
            ((((((*std::cell::LazyCell::force_mut(&mut *&raw mut calls_0)) == (2))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut made_1)) == (0)))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_2)) == (0)))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_3)) == (0)))
                as i32)
        },
        &mut (ok),
    ) {
        (__tmp_30, __tmp_31) => {
            (*__tmp_31) = (((*__tmp_31) as i32) & (__tmp_30 as i32)) != 0;
        }
    };
    match (
        { (((!(move_hit.1)) && ((source.value) == (20))) as i32) },
        &mut (ok),
    ) {
        (__tmp_32, __tmp_33) => {
            (*__tmp_33) = (((*__tmp_33) as i32) & (__tmp_32 as i32)) != 0;
        }
    };
    key.prefix_inc();
    let mut copied: (UnsafeBidirectionalIterator<(i32, Item)>, bool) =
        match (((unsafe { argument_4(&mut source) }) as *mut Item),) {
            (__tmp_34,) => items.try_emplace(key, || Item::copy_from({ __tmp_34 })),
        };
    match (
        {
            ((((((*std::cell::LazyCell::force_mut(&mut *&raw mut calls_0)) == (3))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut made_1)) == (0)))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_2)) == (1)))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_3)) == (0)))
                as i32)
        },
        &mut (ok),
    ) {
        (__tmp_35, __tmp_36) => {
            (*__tmp_36) = (((*__tmp_36) as i32) & (__tmp_35 as i32)) != 0;
        }
    };
    match (
        { (((copied.1) && (((*copied.0.iter_deref()).1.value) == (20))) as i32) },
        &mut (ok),
    ) {
        (__tmp_37, __tmp_38) => {
            (*__tmp_38) = (((*__tmp_38) as i32) & (__tmp_37 as i32)) != 0;
        }
    };
    key.prefix_inc();
    let mut moved: (UnsafeBidirectionalIterator<(i32, Item)>, bool) =
        match (((unsafe { argument_4(&mut source) }) as *mut Item),) {
            (__tmp_39,) => items.try_emplace(key, || Item::move_from({ __tmp_39 })),
        };
    match (
        {
            ((((((*std::cell::LazyCell::force_mut(&mut *&raw mut calls_0)) == (4))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut made_1)) == (0)))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_2)) == (1)))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_3)) == (1)))
                as i32)
        },
        &mut (ok),
    ) {
        (__tmp_40, __tmp_41) => {
            (*__tmp_41) = (((*__tmp_41) as i32) & (__tmp_40 as i32)) != 0;
        }
    };
    match (
        {
            ((((moved.1) && (((*moved.0.iter_deref()).1.value) == (20)))
                && ((source.value) == (-1_i32))) as i32)
        },
        &mut (ok),
    ) {
        (__tmp_42, __tmp_43) => {
            (*__tmp_43) = (((*__tmp_43) as i32) & (__tmp_42 as i32)) != 0;
        }
    };
    let mut refs: UnsafeMap<i32, (i32, RefItem)> = UnsafeMap::new();
    key = 0;
    let mut counter: i32 = 0;
    refs.try_emplace(key, || RefItem::new_2({ &mut counter }, { 5 }));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut made_1)) = 0;
    let mut ref_hit: (UnsafeBidirectionalIterator<(i32, RefItem)>, bool) =
        match (((unsafe { reference_5(&mut counter) }) as *mut i32),) {
            (__tmp_44,) => refs.try_emplace(key, || RefItem::new_2({ __tmp_44 }, { 2 })),
        };
    match (
        {
            ((((((*std::cell::LazyCell::force_mut(&mut *&raw mut calls_0)) == (5))
                && ((counter) == (5)))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut made_1)) == (0)))
                && (!(ref_hit.1))) as i32)
        },
        &mut (ok),
    ) {
        (__tmp_45, __tmp_46) => {
            (*__tmp_46) = (((*__tmp_46) as i32) & (__tmp_45 as i32)) != 0;
        }
    };
    key.prefix_inc();
    let mut ref_new: (UnsafeBidirectionalIterator<(i32, RefItem)>, bool) =
        match (((unsafe { reference_5(&mut counter) }) as *mut i32),) {
            (__tmp_47,) => refs.try_emplace(key, || RefItem::new_2({ __tmp_47 }, { 2 })),
        };
    match (
        {
            ((((((*std::cell::LazyCell::force_mut(&mut *&raw mut calls_0)) == (6))
                && ((counter) == (7)))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut made_1)) == (1)))
                && (ref_new.1)) as i32)
        },
        &mut (ok),
    ) {
        (__tmp_48, __tmp_49) => {
            (*__tmp_49) = (((*__tmp_49) as i32) & (__tmp_48 as i32)) != 0;
        }
    };
    match (
        {
            (((((*ref_hit.0.iter_deref()).1.value) == (5))
                && (((*ref_new.0.iter_deref()).1.value) == (7))) as i32)
        },
        &mut (ok),
    ) {
        (__tmp_50, __tmp_51) => {
            (*__tmp_51) = (((*__tmp_51) as i32) & (__tmp_50 as i32)) != 0;
        }
    };
    key = 0;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut made_1)) = match (
        {
            match (
                { 0 },
                &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_3)),
            ) {
                (__tmp_54, __tmp_55) => {
                    (*__tmp_55) = __tmp_54;
                    (*__tmp_55).clone()
                }
            }
        },
        &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_2)),
    ) {
        (__tmp_52, __tmp_53) => {
            (*__tmp_53) = __tmp_52;
            (*__tmp_53).clone()
        }
    };
    let mut temporary_hit: (UnsafeBidirectionalIterator<(i32, Item)>, bool) = {
        let mut __tmp_57: Item;
        match (({
            __tmp_57 = Item::new_1({ 30 });
            &mut __tmp_57
        } as *mut Item),)
        {
            (__tmp_56,) => items.try_emplace(key, || Item::move_from({ __tmp_56 })),
        }
    };
    match (
        {
            ((((((*std::cell::LazyCell::force_mut(&mut *&raw mut made_1)) == (1))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_2)) == (0)))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_3)) == (0)))
                && (!(temporary_hit.1))) as i32)
        },
        &mut (ok),
    ) {
        (__tmp_58, __tmp_59) => {
            (*__tmp_59) = (((*__tmp_59) as i32) & (__tmp_58 as i32)) != 0;
        }
    };
    key.prefix_inc();
    key.prefix_inc();
    key.prefix_inc();
    let mut temporary_new: (UnsafeBidirectionalIterator<(i32, Item)>, bool) = {
        let mut __tmp_61: Item;
        match (({
            __tmp_61 = Item::new_1({ 40 });
            &mut __tmp_61
        } as *mut Item),)
        {
            (__tmp_60,) => items.try_emplace(key, || Item::move_from({ __tmp_60 })),
        }
    };
    match (
        {
            ((((((*std::cell::LazyCell::force_mut(&mut *&raw mut made_1)) == (2))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_2)) == (0)))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_3)) == (1)))
                && (temporary_new.1)) as i32)
        },
        &mut (ok),
    ) {
        (__tmp_62, __tmp_63) => {
            (*__tmp_63) = (((*__tmp_63) as i32) & (__tmp_62 as i32)) != 0;
        }
    };
    match (
        { ((((*temporary_new.0.iter_deref()).1.value) == (40)) as i32) },
        &mut (ok),
    ) {
        (__tmp_64, __tmp_65) => {
            (*__tmp_65) = (((*__tmp_65) as i32) & (__tmp_64 as i32)) != 0;
        }
    };
    (unsafe {
        libc::printf(
            c"%d %d %d %d %d\n".as_ptr() as *const libc::c_char,
            (rows),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut calls_0)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut made_1)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_2)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_3)),
        )
    });
    return if ok { 0 } else { 1 };
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const calls_0);
    std::cell::LazyCell::force(&*&raw const made_1);
    std::cell::LazyCell::force(&*&raw const copies_2);
    std::cell::LazyCell::force(&*&raw const moves_3);
    std::cell::LazyCell::force(&*&raw const global_map_6);
    std::cell::LazyCell::force(&*&raw const global_key_7);
    std::cell::LazyCell::force(&*&raw const global_counter_8);
    std::cell::LazyCell::force(&*&raw const seed_9);
    std::cell::LazyCell::force(&*&raw const global_hit_10);
}
