// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f1(a0: AnyPtr, a1: AnyPtr, a2: usize) -> AnyPtr {
    a0.memcpy(&a1, a2 as usize);
    a0.clone()
}

fn f2(a0: AnyPtr, a1: u8, a2: usize) -> AnyPtr {
    a0.memset((a1) as u8, a2 as usize);
    a0.clone()
}

fn f3(a0: AnyPtr, a1: AnyPtr, a2: usize) -> i32 {
    a0.memcmp(&a1, a2)
}

fn f4(a0: AnyPtr, a1: AnyPtr, a2: usize) -> AnyPtr {
    a0.memcpy(&a1, a2 as usize);
    a0.clone()
}

fn f5(a0: Ptr<u8>, a1: i32) -> Ptr<u8> {
    let __s = a0.clone();
    let __t = a1 as u8;
    match __s.to_c_string_iterator().position(|__c| __c == __t) {
        Some(__i) => __s.offset(__i),
        None => {
            if __t == 0 {
                __s.offset(__s.to_c_string_iterator().count())
            } else {
                Ptr::null()
            }
        }
    }
}

fn f7(a0: Ptr<u8>) -> usize {
    a0.to_c_string_iterator().count()
}

fn f8(a0: Ptr<u8>, a1: Ptr<u8>) -> i32 {
    let mut __it1 = a0.to_c_string_iterator();
    let mut __it2 = a1.to_c_string_iterator();
    loop {
        let __c1 = __it1.next();
        let __c2 = __it2.next();
        if __c1 != __c2 {
            break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
        }
        if __c1.is_none() {
            break 0;
        }
    }
}

fn f9(a0: Ptr<u8>, a1: Ptr<u8>, a2: usize) -> i32 {
    let __n = a2;
    let mut __it1 = a0.to_c_string_iterator().take(__n);
    let mut __it2 = a1.to_c_string_iterator().take(__n);
    loop {
        let __c1 = __it1.next();
        let __c2 = __it2.next();
        if __c1 != __c2 {
            break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
        }
        if __c1.is_none() {
            break 0;
        }
    }
}

fn f10(a0: AnyPtr, a1: i32, a2: usize) -> AnyPtr {
    let mut __p = a0.reinterpret_cast::<u8>();
    let mut __i: usize = 0;
    loop {
        if __i == a2 {
            break Ptr::<u8>::null().to_any();
        }
        if __p.read() == a1 as u8 {
            break __p.to_any();
        }
        __p += 1;
        __i += 1;
    }
}

fn f11(a0: Ptr<u8>, a1: i32) -> Ptr<u8> {
    let __s = a0.clone();
    let __t = a1 as u8;
    match __s
        .to_c_string_iterator()
        .enumerate()
        .filter(|__e| __e.1 == __t)
        .last()
    {
        Some((__i, _)) => __s.offset(__i),
        None => {
            if __t == 0 {
                __s.offset(__s.to_c_string_iterator().count())
            } else {
                Ptr::null()
            }
        }
    }
}

fn f15(a0: Ptr<u8>) -> Ptr<u8> {
    libcc2rs::strdup_refcount(a0.clone())
}

fn f16(a0: Ptr<u8>, a1: Ptr<u8>) -> usize {
    let __set = a1.clone();
    a0.to_c_string_iterator()
        .take_while(|__c| !__set.to_c_string_iterator().any(|__r| __r == *__c))
        .count()
}

fn f17(a0: Ptr<u8>, a1: Ptr<u8>) -> usize {
    let __set = a1.clone();
    a0.to_c_string_iterator()
        .take_while(|__c| __set.to_c_string_iterator().any(|__r| __r == *__c))
        .count()
}

fn f18(a0: Ptr<u8>, a1: Ptr<u8>) -> Ptr<u8> {
    let __needle = a1.clone();
    let mut __p = a0.clone();
    loop {
        let mut __h = __p.to_c_string_iterator();
        if __needle
            .to_c_string_iterator()
            .all(|__c| __h.next() == Some(__c))
        {
            break __p;
        }
        if __p.read() == 0 {
            break Ptr::null();
        }
        __p += 1;
    }
}

fn f21(a0: Ptr<u8>, a1: Ptr<u8>) -> Ptr<u8> {
    let __s = a0.clone();
    let __set = a1.clone();
    match __s
        .to_c_string_iterator()
        .position(|__c| __set.to_c_string_iterator().any(|__r| __r == __c))
    {
        Some(__i) => __s.offset(__i),
        None => Ptr::null(),
    }
}

#[cfg(target_os = "linux")]
fn f24(a0: AnyPtr, a1: i32, a2: usize) -> AnyPtr {
    let mut __p = a0.reinterpret_cast::<u8>().offset(a2);
    let mut __i: usize = a2;
    loop {
        if __i == 0 {
            break Ptr::<u8>::null().to_any();
        }
        __p -= 1;
        __i -= 1;
        if __p.read() == a1 as u8 {
            break __p.to_any();
        }
    }
}

fn f27(a0: Ptr<u8>, a1: Ptr<u8>) -> i32 {
    let mut __it1 = a0
        .to_c_string_iterator()
        .map(|__c| __c.to_ascii_lowercase());
    let mut __it2 = a1
        .to_c_string_iterator()
        .map(|__c| __c.to_ascii_lowercase());
    loop {
        let __c1 = __it1.next();
        let __c2 = __it2.next();
        if __c1 != __c2 {
            break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
        }
        if __c1.is_none() {
            break 0;
        }
    }
}

#[cfg(target_os = "linux")]
fn f28(a0: i32, a1: Ptr<u8>, a2: usize) -> Ptr<u8> {
    let __msg = std::io::Error::from_raw_os_error(a0).to_string();
    let __len = __msg.len().min(a2.saturating_sub(1));
    let mut __p = a1.clone();
    for __i in 0..__len {
        __p.write(__msg.as_bytes()[__i]);
        __p += 1;
    }
    if a2 > 0 {
        a1.offset(__len).write(0);
    }
    a1
}

#[cfg(target_os = "macos")]
fn f28(a0: i32, a1: Ptr<u8>, a2: usize) -> i32 {
    let __msg = std::io::Error::from_raw_os_error(a0).to_string();
    let __len = __msg.len().min(a2.saturating_sub(1));
    let mut __p = a1.clone();
    for __i in 0..__len {
        __p.write(__msg.as_bytes()[__i]);
        __p += 1;
    }
    if a2 > 0 {
        a1.offset(__len).write(0);
    }
    0
}

fn f6(a0: Ptr<u8>, a1: i32) -> Ptr<u8> {
    let __s = a0.clone();
    let __t = a1 as u8;
    match __s.to_c_string_iterator().position(|__c| __c == __t) {
        Some(__i) => __s.offset(__i),
        None => {
            if __t == 0 {
                __s.offset(__s.to_c_string_iterator().count())
            } else {
                Ptr::null()
            }
        }
    }
}

fn f12(a0: AnyPtr, a1: i32, a2: usize) -> AnyPtr {
    let mut __p = a0.reinterpret_cast::<u8>();
    let mut __i: usize = 0;
    loop {
        if __i == a2 {
            break Ptr::<u8>::null().to_any();
        }
        if __p.read() == a1 as u8 {
            break __p.to_any();
        }
        __p += 1;
        __i += 1;
    }
}

fn f13(a0: Ptr<u8>, a1: i32) -> Ptr<u8> {
    let __s = a0.clone();
    let __t = a1 as u8;
    match __s
        .to_c_string_iterator()
        .enumerate()
        .filter(|__e| __e.1 == __t)
        .last()
    {
        Some((__i, _)) => __s.offset(__i),
        None => {
            if __t == 0 {
                __s.offset(__s.to_c_string_iterator().count())
            } else {
                Ptr::null()
            }
        }
    }
}

fn f14(a0: Ptr<u8>, a1: i32) -> Ptr<u8> {
    let __s = a0.clone();
    let __t = a1 as u8;
    match __s
        .to_c_string_iterator()
        .enumerate()
        .filter(|__e| __e.1 == __t)
        .last()
    {
        Some((__i, _)) => __s.offset(__i),
        None => {
            if __t == 0 {
                __s.offset(__s.to_c_string_iterator().count())
            } else {
                Ptr::null()
            }
        }
    }
}

fn f19(a0: Ptr<u8>, a1: Ptr<u8>) -> Ptr<u8> {
    let __needle = a1.clone();
    let mut __p = a0.clone();
    loop {
        let mut __h = __p.to_c_string_iterator();
        if __needle
            .to_c_string_iterator()
            .all(|__c| __h.next() == Some(__c))
        {
            break __p;
        }
        if __p.read() == 0 {
            break Ptr::null();
        }
        __p += 1;
    }
}

fn f20(a0: Ptr<u8>, a1: Ptr<u8>) -> Ptr<u8> {
    let __needle = a1.clone();
    let mut __p = a0.clone();
    loop {
        let mut __h = __p.to_c_string_iterator();
        if __needle
            .to_c_string_iterator()
            .all(|__c| __h.next() == Some(__c))
        {
            break __p;
        }
        if __p.read() == 0 {
            break Ptr::null();
        }
        __p += 1;
    }
}

fn f22(a0: Ptr<u8>, a1: Ptr<u8>) -> Ptr<u8> {
    let __s = a0.clone();
    let __set = a1.clone();
    match __s
        .to_c_string_iterator()
        .position(|__c| __set.to_c_string_iterator().any(|__r| __r == __c))
    {
        Some(__i) => __s.offset(__i),
        None => Ptr::null(),
    }
}

fn f23(a0: Ptr<u8>, a1: Ptr<u8>) -> Ptr<u8> {
    let __s = a0.clone();
    let __set = a1.clone();
    match __s
        .to_c_string_iterator()
        .position(|__c| __set.to_c_string_iterator().any(|__r| __r == __c))
    {
        Some(__i) => __s.offset(__i),
        None => Ptr::null(),
    }
}

#[cfg(target_os = "linux")]
fn f25(a0: AnyPtr, a1: i32, a2: usize) -> AnyPtr {
    let mut __p = a0.reinterpret_cast::<u8>().offset(a2);
    let mut __i: usize = a2;
    loop {
        if __i == 0 {
            break Ptr::<u8>::null().to_any();
        }
        __p -= 1;
        __i -= 1;
        if __p.read() == a1 as u8 {
            break __p.to_any();
        }
    }
}

#[cfg(target_os = "linux")]
fn f26(a0: AnyPtr, a1: i32, a2: usize) -> AnyPtr {
    let mut __p = a0.reinterpret_cast::<u8>().offset(a2);
    let mut __i: usize = a2;
    loop {
        if __i == 0 {
            break Ptr::<u8>::null().to_any();
        }
        __p -= 1;
        __i -= 1;
        if __p.read() == a1 as u8 {
            break __p.to_any();
        }
    }
}
