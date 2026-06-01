// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn t1<T1>() -> Option<Box<T1>> {
    None
}

fn t2<T1>() -> Option<Box<[T1]>> {
    None
}

unsafe fn f1<T1: Default>(a0: usize) -> Option<Box<[T1]>> {
    Some((0..a0).map(|_| <T1>::default()).collect::<Box<[_]>>())
}
unsafe fn f2<T1>(a0: &mut Option<Box<T1>>) -> *mut T1 {
    a0.as_deref_mut()
        .map_or(::std::ptr::null_mut(), |v| v as *mut T1)
}
unsafe fn f3<T1>(a0: *mut T1) -> Option<Box<T1>> {
    Some(Box::from_raw(a0))
}
unsafe fn f4<T1>(a0: *mut T1) -> Option<Box<T1>> {
    Some(Box::from_raw(a0))
}
unsafe fn f5<T1: Default>(a0: &mut Option<Box<T1>>, a1: *mut T1) {
    let _a0: *mut T1 = a1;
    *a0 = if _a0.is_null() {
        None
    } else {
        Some(Box::from_raw(_a0))
    }
}
unsafe fn f6<T1>(a0: &mut Option<Box<T1>>, a1: *mut T1) {
    *a0 = Some(Box::from_raw(a1))
}
unsafe fn f7<T1>(a0: &mut Option<Box<[T1]>>) -> *mut T1 {
    a0.as_deref_mut()
        .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr())
}

unsafe fn f8<T1>(a0: T1) -> Option<Box<T1>> {
    Some(Box::new(a0))
}

unsafe fn f9<T1>(a0: &mut Option<Box<[T1]>>) {
    *a0 = None
}

unsafe fn f10<T1: Default>() -> Option<Box<T1>> {
    None
}

unsafe fn f11<T1>() -> Option<Box<[T1]>> {
    None
}
