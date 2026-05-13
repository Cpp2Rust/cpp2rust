// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::io::{Seek, Write};

unsafe fn f1<T1: Ord>(a0: *mut T1, a1: *mut T1) {
    let len = a1.offset_from(a0) as usize;
    ::std::slice::from_raw_parts_mut(a0, len).sort()
}

unsafe fn f2<T1: Clone, T2: From<T1>>(a0: *const T1, a1: *const T1, a2: *mut T2) -> *mut T2 {
    let mut outptr = a2.clone();
    let mut curr = a0.clone();
    while curr < a1 {
        *outptr = (*curr).clone().into();
        curr = curr.offset(1);
        outptr = outptr.offset(1);
    }
    outptr
}
unsafe fn f3<T1: PartialEq>(a0: *mut T1, a1: *mut T1, a2: T1) -> *mut T1 {
    let mut it = a0;
    while it != a1 && *it != a2 {
        it = it.add(1);
    }
    it
}
unsafe fn f6<T1: Ord, T2>(a0: *mut T1, a1: *mut T1, a2: &mut T2)
where
    T2: FnMut(&T1, &T1) -> bool,
{
    let len = a1.offset_from(a0) as usize;
    ::std::slice::from_raw_parts_mut(a0, len).sort_by(|x, y| {
        if (a2)(x, y) {
            std::cmp::Ordering::Less
        } else if (a2)(y, x) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    })
}

unsafe fn f7<T1: Ord, T2>(a0: *mut T1, a1: *mut T1, a2: &mut T2)
where
    T2: FnMut(&T1, &T1) -> bool,
{
    let len = a1.offset_from(a0) as usize;
    ::std::slice::from_raw_parts_mut(a0, len).sort_by(|x, y| {
        if (a2)(x, y) {
            std::cmp::Ordering::Less
        } else if (a2)(y, x) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    })
}

unsafe fn f8<T1: PartialOrd>(a0: *mut T1, a1: *mut T1) -> *mut T1 {
    if a0 == a1 {
        a0
    } else {
        let mut __a0 = a0;
        let mut max_it = a0;
        __a0 = __a0.add(1);

        while __a0 != a1 {
            if *max_it < *__a0 {
                max_it = __a0;
            }
            __a0 = __a0.add(1);
        }
        max_it
    }
}

unsafe fn f9<T1>(a0: &mut T1, a1: &mut T1) {
    std::mem::swap(&mut *a0, &mut *a1)
}

unsafe fn f10<T1: PartialOrd + Clone>(a0: *mut T1, a1: *mut T1) -> *mut T1 {
    if a0 == a1 {
        a0
    } else {
        let mut write = a0;
        let mut prev = a0;
        let mut it = a0;
        it = it.add(1);

        while it != a1 {
            if *prev != *it {
                write = write.add(1);
                *write = (*it).clone();
                prev = write;
            }
            it = it.add(1);
        }

        write = write.add(1);
        write
    }
}

unsafe fn f12<T1: Clone>(a0: *mut T1, a1: *mut T1, a2: T1) {
    let mut __a0 = a0;
    while __a0 != a1 {
        *__a0 = a2.clone();
        __a0 = __a0.add(1);
    }
}

unsafe fn f13<T1>(a0: *const u8, a1: *const u8, a2: &mut ::std::fs::File) -> ::std::fs::File {
    let __start = a0 as *const u8;
    let __end = a1 as *const u8;
    let __len = __end.offset_from(__start) as usize;
    a2.write_all(::std::slice::from_raw_parts(__start, __len));
    a2.try_clone().unwrap()
}

unsafe fn f14<T1: Ord + Copy, T2>(a0: *mut T1, a1: *mut T1, a2: &mut T2)
where
    T2: FnMut(T1, T1) -> bool,
{
    let len = a1.offset_from(a0) as usize;
    ::std::slice::from_raw_parts_mut(a0, len).sort_by(|x, y| {
        if (a2)(*x, *y) {
            std::cmp::Ordering::Less
        } else if (a2)(*y, *x) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    })
}

unsafe fn f15<T1: Ord + Copy, T2>(a0: *mut T1, a1: *mut T1, a2: &mut T2)
where
    T2: FnMut(T1, T1) -> bool,
{
    let len = a1.offset_from(a0) as usize;
    ::std::slice::from_raw_parts_mut(a0, len).sort_by(|x, y| {
        if (a2)(*x, *y) {
            std::cmp::Ordering::Less
        } else if (a2)(*y, *x) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    })
}

unsafe fn f16<T1: PartialOrd>(a0: *const T1, a1: *const T1) -> *const T1 {
    if *a0 <= *a1 {
        (a0) as *const _
    } else {
        (a1) as *const _
    }
}

unsafe fn f17<T1: PartialOrd>(a0: *const T1, a1: *const T1) -> *const T1 {
    if *a0 >= *a1 {
        (a0) as *const _
    } else {
        (a1) as *const _
    }
}
