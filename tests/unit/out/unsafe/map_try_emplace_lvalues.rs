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
            value: {
                (*counter) += amount;
                (*counter)
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
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut m: UnsafeMap<i32, (i32, Item)> = UnsafeMap::new();
    let mut key: i32 = 0;
    let mut counter: i32 = 0;
    {
        let __inserted = m.get(&key).is_none();
        if __inserted {
            *m.subscript(&key) = Item::new_2({ &mut counter }, { 3 });
        }
        (m.find(&key), __inserted)
    };
    (*std::cell::LazyCell::force_mut(&mut *&raw mut made_0)) = 0;
    let mut hit: (UnsafeBidirectionalIterator<(i32, Item)>, bool) = match (
        ((unsafe { receiver_2(&mut m) }) as *mut UnsafeMap<i32, (i32, Item)>),
        ({
            let __tmp_2 = (unsafe { place_3(&mut counter) });
            (*__tmp_2).prefix_inc();
            __tmp_2
        } as *mut i32),
    ) {
        (__tmp_0, __tmp_1) => {
            let __inserted = (*__tmp_0).get(&key).is_none();
            if __inserted {
                *(*__tmp_0).subscript(&key) = Item::new_2({ __tmp_1 }, { 2 });
            }
            ((*__tmp_0).find(&key), __inserted)
        }
    };
    assert!(
        ((((counter) == (4)) && ((*std::cell::LazyCell::force_mut(&mut *&raw mut made_0)) == (0)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut visits_1)) == (2)))
            && (!(hit.1))
    );
    key.prefix_inc();
    let mut inserted: (UnsafeBidirectionalIterator<(i32, Item)>, bool) = match (({
        let __tmp_4 = &mut counter;
        (*__tmp_4).prefix_inc();
        __tmp_4
    } as *mut i32),)
    {
        (__tmp_3,) => {
            let __inserted = m.get(&key).is_none();
            if __inserted {
                *m.subscript(&key) = Item::new_2({ __tmp_3 }, { 2 });
            }
            (m.find(&key), __inserted)
        }
    };
    assert!(
        (((counter) == (7)) && ((*std::cell::LazyCell::force_mut(&mut *&raw mut made_0)) == (1)))
            && (inserted.1)
    );
    let mut assigned: (UnsafeBidirectionalIterator<(i32, Item)>, bool) =
        match ((match ({ 9 }, &mut counter) {
            (__tmp_7, __tmp_6) => {
                (*__tmp_6) = __tmp_7;
                __tmp_6
            }
        } as *mut i32),)
        {
            (__tmp_5,) => {
                let __inserted = m.get(&key).is_none();
                if __inserted {
                    *m.subscript(&key) = Item::new_2({ __tmp_5 }, { 2 });
                }
                (m.find(&key), __inserted)
            }
        };
    assert!(
        (((counter) == (9)) && ((*std::cell::LazyCell::force_mut(&mut *&raw mut made_0)) == (1)))
            && (!(assigned.1))
    );
    key.prefix_inc();
    let mut compound: (UnsafeBidirectionalIterator<(i32, Item)>, bool) =
        match ((match ({ 4 }, &mut counter) {
            (__tmp_10, __tmp_9) => {
                (*__tmp_9) += __tmp_10;
                __tmp_9
            }
        } as *mut i32),)
        {
            (__tmp_8,) => {
                let __inserted = m.get(&key).is_none();
                if __inserted {
                    *m.subscript(&key) = Item::new_2({ __tmp_8 }, { 2 });
                }
                (m.find(&key), __inserted)
            }
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
        let __tmp_12 = &mut array[(index.postfix_inc()) as usize];
        (*__tmp_12).prefix_inc();
        __tmp_12
    } as *mut i32),)
    {
        (__tmp_11,) => {
            let __inserted = m.get(&key).is_none();
            if __inserted {
                *m.subscript(&key) = Item::new_2({ __tmp_11 }, { 3 });
            }
            (m.find(&key), __inserted)
        }
    };
    assert!(
        (((index) == (1)) && ((array[(0) as usize]) == (4)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut made_0)) == (3))
    );
    key.prefix_inc();
    match ((match ({ 5 }, &mut array[(index.postfix_inc()) as usize]) {
        (__tmp_15, __tmp_14) => {
            (*__tmp_14) = __tmp_15;
            __tmp_14
        }
    } as *mut i32),)
    {
        (__tmp_13,) => {
            let __inserted = m.get(&key).is_none();
            if __inserted {
                *m.subscript(&key) = Item::new_2({ __tmp_13 }, { 3 });
            }
            (m.find(&key), __inserted)
        }
    };
    assert!(
        (((index) == (2)) && ((array[(1) as usize]) == (8)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut made_0)) == (4))
    );
    match ((match ({ ((counter) + (1)) }, &mut counter) {
        (__tmp_18, __tmp_17) => {
            (*__tmp_17) = __tmp_18;
            __tmp_17
        }
    } as *mut i32),)
    {
        (__tmp_16,) => {
            let __inserted = m.get(&key).is_none();
            if __inserted {
                *m.subscript(&key) = Item::new_2({ __tmp_16 }, { 3 });
            }
            (m.find(&key), __inserted)
        }
    };
    assert!(
        ((counter) == (16)) && ((*std::cell::LazyCell::force_mut(&mut *&raw mut made_0)) == (4))
    );
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    let mut choose: bool = false;
    match ((if choose {
        (&mut a.prefix_inc() as *mut i32)
    } else {
        (&mut b.prefix_inc() as *mut i32)
    } as *mut i32),)
    {
        (__tmp_19,) => {
            let __inserted = m.get(&key).is_none();
            if __inserted {
                *m.subscript(&key) = Item::new_2({ __tmp_19 }, { 3 });
            }
            (m.find(&key), __inserted)
        }
    };
    assert!(
        (((a) == (0)) && ((b) == (1)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut made_0)) == (4))
    );
    choose = true;
    match ((if choose {
        (&mut a.prefix_inc() as *mut i32)
    } else {
        (&mut b.prefix_inc() as *mut i32)
    } as *mut i32),)
    {
        (__tmp_20,) => {
            let __inserted = m.get(&key).is_none();
            if __inserted {
                *m.subscript(&key) = Item::new_2({ __tmp_20 }, { 3 });
            }
            (m.find(&key), __inserted)
        }
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
        (__tmp_21,) => {
            let __inserted = m.get(&key).is_none();
            if __inserted {
                *m.subscript(&key) = Item::new_2({ __tmp_21 }, { 3 });
            }
            (m.find(&key), __inserted)
        }
    };
    assert!(
        ((counter) == (17)) && ((*std::cell::LazyCell::force_mut(&mut *&raw mut made_0)) == (4))
    );
    let mut unsigned_map: UnsafeMap<i32, (i32, u32)> = UnsafeMap::new();
    let mut n: u32 = !0_u32;
    let mut wrapped: (UnsafeBidirectionalIterator<(i32, u32)>, bool) =
        match ((match ({ 1_u32 }, &mut n) {
            (__tmp_24, __tmp_23) => {
                (*__tmp_23) = (*__tmp_23).wrapping_add(__tmp_24);
                __tmp_23
            }
        } as *mut u32),)
        {
            (__tmp_22,) => {
                let __inserted = unsigned_map.get(&key).is_none();
                if __inserted {
                    *unsigned_map.subscript(&key) = (*__tmp_22);
                }
                (unsigned_map.find(&key), __inserted)
            }
        };
    assert!(((n) == (0_u32)) && (((*wrapped.0.iter_deref()).1) == (0_u32)));
    let mut bytes: [u8; 2] = [255_u8, 0_u8];
    index = 0;
    key.prefix_inc();
    let mut narrowed: (UnsafeBidirectionalIterator<(i32, u32)>, bool) =
        match ((match ({ 2 }, &mut bytes[(index.postfix_inc()) as usize]) {
            (__tmp_27, __tmp_26) => {
                (*__tmp_26) = (((*__tmp_26) as i32) + __tmp_27) as u8;
                __tmp_26
            }
        } as *mut u8),)
        {
            (__tmp_25,) => {
                let __inserted = unsigned_map.get(&key).is_none();
                if __inserted {
                    *unsigned_map.subscript(&key) = ((*__tmp_25) as u32);
                }
                (unsigned_map.find(&key), __inserted)
            }
        };
    assert!(
        (((index) == (1)) && ((bytes[(0) as usize] as i32) == (1)))
            && (((*narrowed.0.iter_deref()).1) == (1_u32))
    );
    let mut pointers: UnsafeMap<i32, (i32, *mut i32)> = UnsafeMap::new();
    let mut p: *mut i32 = array.as_mut_ptr();
    let mut pointer: (UnsafeBidirectionalIterator<(i32, *mut i32)>, bool) =
        match ((match ({ 1 }, &mut p) {
            (__tmp_30, __tmp_29) => {
                (*__tmp_29) = (*__tmp_29).wrapping_add(__tmp_30 as usize);
                __tmp_29
            }
        } as *mut *mut i32),)
        {
            (__tmp_28,) => {
                let __inserted = pointers.get(&key).is_none();
                if __inserted {
                    *pointers.subscript(&key) = (*__tmp_28);
                }
                (pointers.find(&key), __inserted)
            }
        };
    assert!(
        ((p) == (array.as_mut_ptr().offset((1) as isize)))
            && (((*pointer.0.iter_deref()).1) == (p))
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
    {
        let __inserted = strings.get(&key).is_none();
        if __inserted {
            *strings.subscript(&key) = std::slice::from_raw_parts(
                c"original".as_ptr(),
                (0..)
                    .take_while(|&i| *c"original".as_ptr().add(i) != 0)
                    .count()
                    + 1,
            )
            .to_vec();
        }
        (strings.find(&key), __inserted)
    };
    let mut string_hit: (UnsafeBidirectionalIterator<(i32, Vec<libc::c_char>)>, bool) = match (({
        (*std::cell::LazyCell::force_mut(&mut *&raw mut visits_1)).prefix_inc();
        &mut text
    }
        as *mut Vec<libc::c_char>),)
    {
        (__tmp_31,) => {
            let __inserted = strings.get(&key).is_none();
            if __inserted {
                *strings.subscript(&key) = std::mem::take(&mut (*__tmp_31));
            }
            (strings.find(&key), __inserted)
        }
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
        (__tmp_32,) => {
            let __inserted = strings.get(&key).is_none();
            if __inserted {
                *strings.subscript(&key) = std::mem::take(&mut (*__tmp_32));
            }
            (strings.find(&key), __inserted)
        }
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
}
