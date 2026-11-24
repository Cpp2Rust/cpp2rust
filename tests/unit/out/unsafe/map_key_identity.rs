extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut default_constructions_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Key {
    pub order: i32,
    pub identity: i32,
}
impl std::cmp::Ord for Key {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_1(self as *const Key, other as *const Key) {
                std::cmp::Ordering::Less
            } else if operator_lt_1(other as *const Key, self as *const Key) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_2(self as *const Key, other as *const Key) }
    }
}
impl std::cmp::Eq for Key {}
pub unsafe fn operator_lt_1(a: *const Key, b: *const Key) -> bool {
    return (((*a).order) < ((*b).order));
}
pub unsafe fn operator_eq_2(a: *const Key, b: *const Key) -> bool {
    return (((*a).identity) == ((*b).identity));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Mapped {
    pub value: i32,
}
impl Mapped {
    pub unsafe fn new() -> Self {
        let mut this = Self { value: 0 };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut default_constructions_0)).prefix_inc();
        this
    }
}
impl Default for Mapped {
    fn default() -> Self {
        unsafe { Mapped::new() }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut ok: bool = true;
    let distinct_a: Key = Key {
        order: 1,
        identity: 7,
    };
    let distinct_b: Key = Key {
        order: 2,
        identity: 7,
    };
    match (
        {
            ((unsafe {
                let _a: *const Key = &distinct_a;
                operator_eq_2(_a, &distinct_b)
            }) as i32)
        },
        &mut (ok),
    ) {
        (__tmp_0, __tmp_1) => {
            (*__tmp_1) = (((*__tmp_1) as i32) & (__tmp_0 as i32)) != 0;
        }
    };
    let mut distinct: UnsafeMap<Key, (Key, Mapped)> = UnsafeMap::new();
    let a: *mut Mapped = &mut (*distinct.subscript(&distinct_a, || <Mapped>::default()));
    (*a).value = 11;
    let mut a_it: UnsafeBidirectionalIterator<(Key, Mapped)> = distinct.find(&distinct_a);
    let mut a_value: *mut Mapped = (&mut (*a_it.iter_deref()).1 as *mut Mapped);
    match (
        {
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut default_constructions_0)) == (1))
                as i32)
        },
        &mut (ok),
    ) {
        (__tmp_2, __tmp_3) => {
            (*__tmp_3) = (((*__tmp_3) as i32) & (__tmp_2 as i32)) != 0;
        }
    };
    let b: *mut Mapped = &mut (*distinct.subscript(&distinct_b, || <Mapped>::default()));
    (*b).value = 22;
    let mut b_it: UnsafeBidirectionalIterator<(Key, Mapped)> = distinct.find(&distinct_b);
    match (
        {
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut default_constructions_0)) == (2))
                as i32)
        },
        &mut (ok),
    ) {
        (__tmp_4, __tmp_5) => {
            (*__tmp_5) = (((*__tmp_5) as i32) & (__tmp_4 as i32)) != 0;
        }
    };
    match ({ (((distinct.len()) == (2_usize)) as i32) }, &mut (ok)) {
        (__tmp_6, __tmp_7) => {
            (*__tmp_7) = (((*__tmp_7) as i32) & (__tmp_6 as i32)) != 0;
        }
    };
    match ({ (((a) != (b)) as i32) }, &mut (ok)) {
        (__tmp_8, __tmp_9) => {
            (*__tmp_9) = (((*__tmp_9) as i32) & (__tmp_8 as i32)) != 0;
        }
    };
    match (
        { (((((*a).value) == (11)) && (((*b).value) == (22))) as i32) },
        &mut (ok),
    ) {
        (__tmp_10, __tmp_11) => {
            (*__tmp_11) = (((*__tmp_11) as i32) & (__tmp_10 as i32)) != 0;
        }
    };
    match ({ ((a_it == distinct.find(&distinct_a)) as i32) }, &mut (ok)) {
        (__tmp_12, __tmp_13) => {
            (*__tmp_13) = (((*__tmp_13) as i32) & (__tmp_12 as i32)) != 0;
        }
    };
    match ({ ((b_it == distinct.find(&distinct_b)) as i32) }, &mut (ok)) {
        (__tmp_14, __tmp_15) => {
            (*__tmp_15) = (((*__tmp_15) as i32) & (__tmp_14 as i32)) != 0;
        }
    };
    match ({ ((b_it != distinct.end()) as i32) }, &mut (ok)) {
        (__tmp_16, __tmp_17) => {
            (*__tmp_17) = (((*__tmp_17) as i32) & (__tmp_16 as i32)) != 0;
        }
    };
    match (
        { (((&mut (*a_it.iter_deref()).1 as *mut Mapped) == (a_value)) as i32) },
        &mut (ok),
    ) {
        (__tmp_18, __tmp_19) => {
            (*__tmp_19) = (((*__tmp_19) as i32) & (__tmp_18 as i32)) != 0;
        }
    };
    match (
        { ((((*a_it.iter_deref()).1.value) == (11)) as i32) },
        &mut (ok),
    ) {
        (__tmp_20, __tmp_21) => {
            (*__tmp_21) = (((*__tmp_21) as i32) & (__tmp_20 as i32)) != 0;
        }
    };
    match (
        {
            ((unsafe {
                let _a: *const Key = &distinct_a;
                operator_eq_2(_a, &distinct_b)
            }) as i32)
        },
        &mut (ok),
    ) {
        (__tmp_22, __tmp_23) => {
            (*__tmp_23) = (((*__tmp_23) as i32) & (__tmp_22 as i32)) != 0;
        }
    };
    let mut distinct_count: usize = 0_usize;
    'loop_: for entry in IterRange::new(distinct.begin(), distinct.end()) {
        let entry: *const (Key, Mapped) = &(*entry.iter_deref());
        if ((distinct_count) == (0_usize)) {
            match (
                { (((((*entry).0.order) == (1)) && (((*entry).1.value) == (11))) as i32) },
                &mut (ok),
            ) {
                (__tmp_24, __tmp_25) => {
                    (*__tmp_25) = (((*__tmp_25) as i32) & (__tmp_24 as i32)) != 0;
                }
            };
        } else if ((distinct_count) == (1_usize)) {
            match (
                { (((((*entry).0.order) == (2)) && (((*entry).1.value) == (22))) as i32) },
                &mut (ok),
            ) {
                (__tmp_26, __tmp_27) => {
                    (*__tmp_27) = (((*__tmp_27) as i32) & (__tmp_26 as i32)) != 0;
                }
            };
        }
        distinct_count.prefix_inc();
    }
    match ({ (((distinct_count) == (2_usize)) as i32) }, &mut (ok)) {
        (__tmp_28, __tmp_29) => {
            (*__tmp_29) = (((*__tmp_29) as i32) & (__tmp_28 as i32)) != 0;
        }
    };
    let equivalent_a: Key = Key {
        order: 3,
        identity: 8,
    };
    let equivalent_b: Key = Key {
        order: 3,
        identity: 9,
    };
    match (
        {
            (!(unsafe {
                let _a: *const Key = &equivalent_a;
                operator_eq_2(_a, &equivalent_b)
            }) as i32)
        },
        &mut (ok),
    ) {
        (__tmp_30, __tmp_31) => {
            (*__tmp_31) = (((*__tmp_31) as i32) & (__tmp_30 as i32)) != 0;
        }
    };
    let mut equivalent: UnsafeMap<Key, (Key, Mapped)> = UnsafeMap::new();
    let before_equivalent: i32 =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut default_constructions_0));
    let first: *mut Mapped = &mut (*equivalent.subscript(&equivalent_a, || <Mapped>::default()));
    (*first).value = 31;
    let mut first_it: UnsafeBidirectionalIterator<(Key, Mapped)> = equivalent.find(&equivalent_a);
    let mut first_value: *mut Mapped = (&mut (*first_it.iter_deref()).1 as *mut Mapped);
    match (
        {
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut default_constructions_0))
                == ((before_equivalent) + (1))) as i32)
        },
        &mut (ok),
    ) {
        (__tmp_32, __tmp_33) => {
            (*__tmp_33) = (((*__tmp_33) as i32) & (__tmp_32 as i32)) != 0;
        }
    };
    let second: *mut Mapped = &mut (*equivalent.subscript(&equivalent_b, || <Mapped>::default()));
    let reused: bool = ((*std::cell::LazyCell::force_mut(&mut *&raw mut default_constructions_0))
        == ((before_equivalent) + (1)));
    match ({ (reused as i32) }, &mut (ok)) {
        (__tmp_34, __tmp_35) => {
            (*__tmp_35) = (((*__tmp_35) as i32) & (__tmp_34 as i32)) != 0;
        }
    };
    match ({ (((equivalent.len()) == (1_usize)) as i32) }, &mut (ok)) {
        (__tmp_36, __tmp_37) => {
            (*__tmp_37) = (((*__tmp_37) as i32) & (__tmp_36 as i32)) != 0;
        }
    };
    if reused {
        match ({ ((first_it != equivalent.end()) as i32) }, &mut (ok)) {
            (__tmp_38, __tmp_39) => {
                (*__tmp_39) = (((*__tmp_39) as i32) & (__tmp_38 as i32)) != 0;
            }
        };
        match ({ (((first) == (second)) as i32) }, &mut (ok)) {
            (__tmp_40, __tmp_41) => {
                (*__tmp_41) = (((*__tmp_41) as i32) & (__tmp_40 as i32)) != 0;
            }
        };
        match (
            { ((first_it == equivalent.find(&equivalent_a)) as i32) },
            &mut (ok),
        ) {
            (__tmp_42, __tmp_43) => {
                (*__tmp_43) = (((*__tmp_43) as i32) & (__tmp_42 as i32)) != 0;
            }
        };
        match (
            { ((first_it == equivalent.find(&equivalent_b)) as i32) },
            &mut (ok),
        ) {
            (__tmp_44, __tmp_45) => {
                (*__tmp_45) = (((*__tmp_45) as i32) & (__tmp_44 as i32)) != 0;
            }
        };
        match (
            { (((&mut (*first_it.iter_deref()).1 as *mut Mapped) == (first_value)) as i32) },
            &mut (ok),
        ) {
            (__tmp_46, __tmp_47) => {
                (*__tmp_47) = (((*__tmp_47) as i32) & (__tmp_46 as i32)) != 0;
            }
        };
        match (
            {
                (((((*first_it.iter_deref()).1.value) == (31)) && (((*second).value) == (31)))
                    as i32)
            },
            &mut (ok),
        ) {
            (__tmp_48, __tmp_49) => {
                (*__tmp_49) = (((*__tmp_49) as i32) & (__tmp_48 as i32)) != 0;
            }
        };
        let mut equivalent_count: usize = 0_usize;
        'loop_: for entry in IterRange::new(equivalent.begin(), equivalent.end()) {
            let entry: *const (Key, Mapped) = &(*entry.iter_deref());
            match (
                { (((((*entry).0.order) == (3)) && (((*entry).1.value) == (31))) as i32) },
                &mut (ok),
            ) {
                (__tmp_50, __tmp_51) => {
                    (*__tmp_51) = (((*__tmp_51) as i32) & (__tmp_50 as i32)) != 0;
                }
            };
            equivalent_count.prefix_inc();
        }
        match ({ (((equivalent_count) == (1_usize)) as i32) }, &mut (ok)) {
            (__tmp_52, __tmp_53) => {
                (*__tmp_53) = (((*__tmp_53) as i32) & (__tmp_52 as i32)) != 0;
            }
        };
    }
    match (
        {
            (!(unsafe {
                let _a: *const Key = &equivalent_a;
                operator_eq_2(_a, &equivalent_b)
            }) as i32)
        },
        &mut (ok),
    ) {
        (__tmp_54, __tmp_55) => {
            (*__tmp_55) = (((*__tmp_55) as i32) & (__tmp_54 as i32)) != 0;
        }
    };
    let later_key: Key = Key {
        order: 4,
        identity: 10,
    };
    let later: *mut Mapped = &mut (*equivalent.subscript(&later_key, || <Mapped>::default()));
    (*later).value = 41;
    match (
        {
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut default_constructions_0))
                == ((before_equivalent) + (2))) as i32)
        },
        &mut (ok),
    ) {
        (__tmp_56, __tmp_57) => {
            (*__tmp_57) = (((*__tmp_57) as i32) & (__tmp_56 as i32)) != 0;
        }
    };
    match ({ (((equivalent.len()) == (2_usize)) as i32) }, &mut (ok)) {
        (__tmp_58, __tmp_59) => {
            (*__tmp_59) = (((*__tmp_59) as i32) & (__tmp_58 as i32)) != 0;
        }
    };
    if reused {
        match (
            { ((first_it == equivalent.find(&equivalent_a)) as i32) },
            &mut (ok),
        ) {
            (__tmp_60, __tmp_61) => {
                (*__tmp_61) = (((*__tmp_61) as i32) & (__tmp_60 as i32)) != 0;
            }
        };
        match (
            { (((&mut (*first_it.iter_deref()).1 as *mut Mapped) == (first_value)) as i32) },
            &mut (ok),
        ) {
            (__tmp_62, __tmp_63) => {
                (*__tmp_63) = (((*__tmp_63) as i32) & (__tmp_62 as i32)) != 0;
            }
        };
        match (
            { ((((*first_it.iter_deref()).1.value) == (31)) as i32) },
            &mut (ok),
        ) {
            (__tmp_64, __tmp_65) => {
                (*__tmp_65) = (((*__tmp_65) as i32) & (__tmp_64 as i32)) != 0;
            }
        };
    }
    let matching_a: Key = Key {
        order: 5,
        identity: 5,
    };
    let matching_b: Key = Key {
        order: 5,
        identity: 5,
    };
    let matching_c: Key = Key {
        order: 6,
        identity: 6,
    };
    let before_matching: i32 =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut default_constructions_0));
    let mut matching: UnsafeMap<Key, (Key, Mapped)> = UnsafeMap::new();
    let matched: *mut Mapped = &mut (*matching.subscript(&matching_a, || <Mapped>::default()));
    (*matched).value = 51;
    let mut matched_it: UnsafeBidirectionalIterator<(Key, Mapped)> = matching.find(&matching_a);
    let mut matched_value: *mut Mapped = (&mut (*matched_it.iter_deref()).1 as *mut Mapped);
    match (
        {
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut default_constructions_0))
                == ((before_matching) + (1))) as i32)
        },
        &mut (ok),
    ) {
        (__tmp_66, __tmp_67) => {
            (*__tmp_67) = (((*__tmp_67) as i32) & (__tmp_66 as i32)) != 0;
        }
    };
    let matched_again: *mut Mapped =
        &mut (*matching.subscript(&matching_b, || <Mapped>::default()));
    match (
        {
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut default_constructions_0))
                == ((before_matching) + (1))) as i32)
        },
        &mut (ok),
    ) {
        (__tmp_68, __tmp_69) => {
            (*__tmp_69) = (((*__tmp_69) as i32) & (__tmp_68 as i32)) != 0;
        }
    };
    match ({ (((matching.len()) == (1_usize)) as i32) }, &mut (ok)) {
        (__tmp_70, __tmp_71) => {
            (*__tmp_71) = (((*__tmp_71) as i32) & (__tmp_70 as i32)) != 0;
        }
    };
    match ({ (((matched) == (matched_again)) as i32) }, &mut (ok)) {
        (__tmp_72, __tmp_73) => {
            (*__tmp_73) = (((*__tmp_73) as i32) & (__tmp_72 as i32)) != 0;
        }
    };
    match (
        { ((matched_it == matching.find(&matching_b)) as i32) },
        &mut (ok),
    ) {
        (__tmp_74, __tmp_75) => {
            (*__tmp_75) = (((*__tmp_75) as i32) & (__tmp_74 as i32)) != 0;
        }
    };
    match (
        { (((&mut (*matched_it.iter_deref()).1 as *mut Mapped) == (matched_value)) as i32) },
        &mut (ok),
    ) {
        (__tmp_76, __tmp_77) => {
            (*__tmp_77) = (((*__tmp_77) as i32) & (__tmp_76 as i32)) != 0;
        }
    };
    match (
        { ((((*matched_it.iter_deref()).1.value) == (51)) as i32) },
        &mut (ok),
    ) {
        (__tmp_78, __tmp_79) => {
            (*__tmp_79) = (((*__tmp_79) as i32) & (__tmp_78 as i32)) != 0;
        }
    };
    let other: *mut Mapped = &mut (*matching.subscript(&matching_c, || <Mapped>::default()));
    (*other).value = 61;
    match (
        {
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut default_constructions_0))
                == ((before_matching) + (2))) as i32)
        },
        &mut (ok),
    ) {
        (__tmp_80, __tmp_81) => {
            (*__tmp_81) = (((*__tmp_81) as i32) & (__tmp_80 as i32)) != 0;
        }
    };
    match ({ (((matching.len()) == (2_usize)) as i32) }, &mut (ok)) {
        (__tmp_82, __tmp_83) => {
            (*__tmp_83) = (((*__tmp_83) as i32) & (__tmp_82 as i32)) != 0;
        }
    };
    match (
        { ((matched_it == matching.find(&matching_a)) as i32) },
        &mut (ok),
    ) {
        (__tmp_84, __tmp_85) => {
            (*__tmp_85) = (((*__tmp_85) as i32) & (__tmp_84 as i32)) != 0;
        }
    };
    match (
        { (((&mut (*matched_it.iter_deref()).1 as *mut Mapped) == (matched_value)) as i32) },
        &mut (ok),
    ) {
        (__tmp_86, __tmp_87) => {
            (*__tmp_87) = (((*__tmp_87) as i32) & (__tmp_86 as i32)) != 0;
        }
    };
    match (
        { ((((*matched_it.iter_deref()).1.value) == (51)) as i32) },
        &mut (ok),
    ) {
        (__tmp_88, __tmp_89) => {
            (*__tmp_89) = (((*__tmp_89) as i32) & (__tmp_88 as i32)) != 0;
        }
    };
    return if ok { 0 } else { 1 };
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const default_constructions_0);
}
