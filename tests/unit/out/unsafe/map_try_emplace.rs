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
            value: {
                (*counter) += amount;
                (*counter)
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
        (*(*std::cell::LazyCell::force_mut(&mut *&raw mut global_map_6))
            .subscript(&(*std::cell::LazyCell::force_mut(&mut *&raw mut global_key_7)))) = 10;
        0
    }
});
pub static mut global_hit_10: std::cell::LazyCell<(UnsafeBidirectionalIterator<(i32, i32)>, bool)> =
    std::cell::LazyCell::new(|| unsafe {
        match (({
            let __tmp_1 = &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut global_counter_8));
            (*__tmp_1).prefix_inc();
            __tmp_1
        } as *mut i32),)
        {
            (__tmp_0,) => {
                let __inserted = (*std::cell::LazyCell::force_mut(&mut *&raw mut global_map_6))
                    .get(&(*std::cell::LazyCell::force_mut(&mut *&raw mut global_key_7)))
                    .is_none();
                if __inserted {
                    *(*std::cell::LazyCell::force_mut(&mut *&raw mut global_map_6)).subscript(
                        &(*std::cell::LazyCell::force_mut(&mut *&raw mut global_key_7)),
                    ) = (*__tmp_0);
                }
                (
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut global_map_6))
                        .find(&(*std::cell::LazyCell::force_mut(&mut *&raw mut global_key_7))),
                    __inserted,
                )
            }
        }
    });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
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
    let mut subset: i32 = 0;
    'loop_: while ((subset) < (8)) {
        let mut key: i32 = 0;
        'loop_: while ((key) < (3)) {
            let mut initial: i32 = 0;
            'loop_: while ((initial) < (3)) {
                let mut post: i32 = 0;
                'loop_: while ((post) < (2)) {
                    let mut m: UnsafeMap<i32, (i32, i32)> = UnsafeMap::new();
                    let mut size: i32 = 0;
                    let mut k: i32 = 0;
                    'loop_: while ((k) < (3)) {
                        if (((subset) & ((1) << (k))) != 0) {
                            (*m.subscript(&k)) = ((10) + (k));
                            size.prefix_inc();
                        }
                        k.prefix_inc();
                    }
                    let mut counter: i32 = initial;
                    let mut result: (UnsafeBidirectionalIterator<(i32, i32)>, bool) = if (post != 0)
                    {
                        match ((&mut counter.postfix_inc() as *mut i32),) {
                            (__tmp_2,) => {
                                let __inserted = m.get(&key).is_none();
                                if __inserted {
                                    *m.subscript(&key) = (*__tmp_2);
                                }
                                (m.find(&key), __inserted)
                            }
                        }
                    } else {
                        match (({
                            let __tmp_4 = &mut counter;
                            (*__tmp_4).prefix_inc();
                            __tmp_4
                        } as *mut i32),)
                        {
                            (__tmp_3,) => {
                                let __inserted = m.get(&key).is_none();
                                if __inserted {
                                    *m.subscript(&key) = (*__tmp_3);
                                }
                                (m.find(&key), __inserted)
                            }
                        }
                    };
                    let mut hit: bool = (((subset) & ((1) << (key))) != (0));
                    ok = ((ok as i32) & (((counter) == ((initial) + (1))) as i32)) != 0;
                    ok = ((ok as i32) & (((result.1 as i32) == (!(hit) as i32)) as i32)) != 0;
                    let mut found: bool = result.0 == m.find(&key);
                    ok = ((ok as i32) & (found as i32)) != 0;
                    ok = ((ok as i32) & ((((*result.0.iter_deref()).0) == (key)) as i32)) != 0;
                    ok = ((ok as i32)
                        & ((((*result.0.iter_deref()).1)
                            == (if hit {
                                ((10) + (key))
                            } else {
                                ((initial) + (!(post != 0) as i32))
                            })) as i32))
                        != 0;
                    ok = ((ok as i32)
                        & (((m.len()) == (((size) + (!(hit) as i32)) as usize)) as i32))
                        != 0;
                    rows.prefix_inc();
                    post.prefix_inc();
                }
                initial.prefix_inc();
            }
            key.prefix_inc();
        }
        subset.prefix_inc();
    }
    let mut items: UnsafeMap<i32, (i32, Item)> = UnsafeMap::new();
    let mut key: i32 = 0;
    {
        let __inserted = items.get(&key).is_none();
        if __inserted {
            *items.subscript(&key) = Item::new_1({ 10 });
        }
        (items.find(&key), __inserted)
    };
    let mut source: Item = Item::new_1({ 20 });
    (*std::cell::LazyCell::force_mut(&mut *&raw mut made_1)) = {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_2)) = {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_3)) = 0;
            (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_3))
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_2))
    };
    let mut hit: (UnsafeBidirectionalIterator<(i32, Item)>, bool) =
        match (((unsafe { argument_4(&mut source) }) as *mut Item),) {
            (__tmp_5,) => {
                let __inserted = items.get(&key).is_none();
                if __inserted {
                    *items.subscript(&key) = Item::copy_from({ __tmp_5 });
                }
                (items.find(&key), __inserted)
            }
        };
    ok = ((ok as i32)
        & ((((((*std::cell::LazyCell::force_mut(&mut *&raw mut calls_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut made_1)) == (0)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_2)) == (0)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_3)) == (0)))
            as i32))
        != 0;
    ok = ((ok as i32) & (((!(hit.1)) && (((*hit.0.iter_deref()).1.value) == (10))) as i32)) != 0;
    let mut move_hit: (UnsafeBidirectionalIterator<(i32, Item)>, bool) =
        match (((unsafe { argument_4(&mut source) }) as *mut Item),) {
            (__tmp_6,) => {
                let __inserted = items.get(&key).is_none();
                if __inserted {
                    *items.subscript(&key) = Item::move_from({ __tmp_6 });
                }
                (items.find(&key), __inserted)
            }
        };
    ok = ((ok as i32)
        & ((((((*std::cell::LazyCell::force_mut(&mut *&raw mut calls_0)) == (2))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut made_1)) == (0)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_2)) == (0)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_3)) == (0)))
            as i32))
        != 0;
    ok = ((ok as i32) & (((!(move_hit.1)) && ((source.value) == (20))) as i32)) != 0;
    key.prefix_inc();
    let mut copied: (UnsafeBidirectionalIterator<(i32, Item)>, bool) =
        match (((unsafe { argument_4(&mut source) }) as *mut Item),) {
            (__tmp_7,) => {
                let __inserted = items.get(&key).is_none();
                if __inserted {
                    *items.subscript(&key) = Item::copy_from({ __tmp_7 });
                }
                (items.find(&key), __inserted)
            }
        };
    ok = ((ok as i32)
        & ((((((*std::cell::LazyCell::force_mut(&mut *&raw mut calls_0)) == (3))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut made_1)) == (0)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_2)) == (1)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_3)) == (0)))
            as i32))
        != 0;
    ok = ((ok as i32) & (((copied.1) && (((*copied.0.iter_deref()).1.value) == (20))) as i32)) != 0;
    key.prefix_inc();
    let mut moved: (UnsafeBidirectionalIterator<(i32, Item)>, bool) =
        match (((unsafe { argument_4(&mut source) }) as *mut Item),) {
            (__tmp_8,) => {
                let __inserted = items.get(&key).is_none();
                if __inserted {
                    *items.subscript(&key) = Item::move_from({ __tmp_8 });
                }
                (items.find(&key), __inserted)
            }
        };
    ok = ((ok as i32)
        & ((((((*std::cell::LazyCell::force_mut(&mut *&raw mut calls_0)) == (4))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut made_1)) == (0)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_2)) == (1)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_3)) == (1)))
            as i32))
        != 0;
    ok = ((ok as i32)
        & ((((moved.1) && (((*moved.0.iter_deref()).1.value) == (20)))
            && ((source.value) == (-1_i32))) as i32))
        != 0;
    let mut refs: UnsafeMap<i32, (i32, RefItem)> = UnsafeMap::new();
    key = 0;
    let mut counter: i32 = 0;
    {
        let __inserted = refs.get(&key).is_none();
        if __inserted {
            *refs.subscript(&key) = RefItem::new_2({ &mut counter }, { 5 });
        }
        (refs.find(&key), __inserted)
    };
    (*std::cell::LazyCell::force_mut(&mut *&raw mut made_1)) = 0;
    let mut ref_hit: (UnsafeBidirectionalIterator<(i32, RefItem)>, bool) =
        match (((unsafe { reference_5(&mut counter) }) as *mut i32),) {
            (__tmp_9,) => {
                let __inserted = refs.get(&key).is_none();
                if __inserted {
                    *refs.subscript(&key) = RefItem::new_2({ __tmp_9 }, { 2 });
                }
                (refs.find(&key), __inserted)
            }
        };
    ok = ((ok as i32)
        & ((((((*std::cell::LazyCell::force_mut(&mut *&raw mut calls_0)) == (5))
            && ((counter) == (5)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut made_1)) == (0)))
            && (!(ref_hit.1))) as i32))
        != 0;
    key.prefix_inc();
    let mut ref_new: (UnsafeBidirectionalIterator<(i32, RefItem)>, bool) =
        match (((unsafe { reference_5(&mut counter) }) as *mut i32),) {
            (__tmp_10,) => {
                let __inserted = refs.get(&key).is_none();
                if __inserted {
                    *refs.subscript(&key) = RefItem::new_2({ __tmp_10 }, { 2 });
                }
                (refs.find(&key), __inserted)
            }
        };
    ok = ((ok as i32)
        & ((((((*std::cell::LazyCell::force_mut(&mut *&raw mut calls_0)) == (6))
            && ((counter) == (7)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut made_1)) == (1)))
            && (ref_new.1)) as i32))
        != 0;
    ok = ((ok as i32)
        & (((((*ref_hit.0.iter_deref()).1.value) == (5))
            && (((*ref_new.0.iter_deref()).1.value) == (7))) as i32))
        != 0;
    key = 0;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut made_1)) = {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_2)) = {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_3)) = 0;
            (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_3))
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_2))
    };
    let mut temporary_hit: (UnsafeBidirectionalIterator<(i32, Item)>, bool) =
        match ((&mut Item::new_1({ 30 }) as *mut Item),) {
            (__tmp_11,) => {
                let __inserted = items.get(&key).is_none();
                if __inserted {
                    *items.subscript(&key) = Item::move_from({ __tmp_11 });
                }
                (items.find(&key), __inserted)
            }
        };
    ok = ((ok as i32)
        & ((((((*std::cell::LazyCell::force_mut(&mut *&raw mut made_1)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_2)) == (0)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_3)) == (0)))
            && (!(temporary_hit.1))) as i32))
        != 0;
    key.prefix_inc();
    key.prefix_inc();
    key.prefix_inc();
    let mut temporary_new: (UnsafeBidirectionalIterator<(i32, Item)>, bool) =
        match ((&mut Item::new_1({ 40 }) as *mut Item),) {
            (__tmp_12,) => {
                let __inserted = items.get(&key).is_none();
                if __inserted {
                    *items.subscript(&key) = Item::move_from({ __tmp_12 });
                }
                (items.find(&key), __inserted)
            }
        };
    ok = ((ok as i32)
        & ((((((*std::cell::LazyCell::force_mut(&mut *&raw mut made_1)) == (2))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_2)) == (0)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_3)) == (1)))
            && (temporary_new.1)) as i32))
        != 0;
    ok = ((ok as i32) & ((((*temporary_new.0.iter_deref()).1.value) == (40)) as i32)) != 0;
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
