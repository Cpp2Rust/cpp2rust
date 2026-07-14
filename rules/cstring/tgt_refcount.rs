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
    let mut __src = a1.reinterpret_cast::<u8>();
    let mut __tmp: Vec<u8> = Vec::with_capacity(a2);
    for _ in 0..a2 {
        __tmp.push(__src.read());
        __src += 1;
    }
    let mut __dst = a0.reinterpret_cast::<u8>();
    for __i in 0..a2 {
        __dst.write(__tmp[__i]);
        __dst += 1;
    }
    a0.clone()
}

fn f5(a0: Ptr<u8>, a1: i32) -> Ptr<u8> {
    let mut __p = a0.clone();
    loop {
        let __c = __p.read();
        if __c == a1 as u8 {
            break __p;
        }
        if __c == 0 {
            break Ptr::null();
        }
        __p += 1;
    }
}

unsafe fn f7(a0: Ptr<u8>) -> usize {
    let mut __p = a0.clone();
    let mut __i: usize = 0;
    while __p.read() != 0 {
        __p += 1;
        __i += 1;
    }
    __i
}

fn f8(a0: Ptr<u8>, a1: Ptr<u8>) -> i32 {
    let mut __p1 = a0.clone();
    let mut __p2 = a1.clone();
    loop {
        let __c1 = __p1.read();
        let __c2 = __p2.read();
        if __c1 != __c2 {
            break (__c1 as i32) - (__c2 as i32);
        }
        if __c1 == 0 {
            break 0;
        }
        __p1 += 1;
        __p2 += 1;
    }
}

fn f9(a0: Ptr<u8>, a1: Ptr<u8>, a2: usize) -> i32 {
    let mut __p1 = a0.clone();
    let mut __p2 = a1.clone();
    let mut __i: usize = 0;
    loop {
        if __i == a2 {
            break 0;
        }
        let __c1 = __p1.read();
        let __c2 = __p2.read();
        if __c1 != __c2 {
            break (__c1 as i32) - (__c2 as i32);
        }
        if __c1 == 0 {
            break 0;
        }
        __p1 += 1;
        __p2 += 1;
        __i += 1;
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
    let mut __p = a0.clone();
    let mut __found = Ptr::null();
    loop {
        let __c = __p.read();
        if __c == a1 as u8 {
            __found = __p.clone();
        }
        if __c == 0 {
            break __found;
        }
        __p += 1;
    }
}

fn f15(a0: Ptr<u8>) -> Ptr<u8> {
    libcc2rs::strdup_refcount(a0.clone())
}

fn f16(a0: Ptr<u8>, a1: Ptr<u8>) -> usize {
    let mut __p = a0.clone();
    let mut __i: usize = 0;
    loop {
        let __c = __p.read();
        if __c == 0 {
            break __i;
        }
        let mut __q = a1.clone();
        let __hit = loop {
            let __r = __q.read();
            if __r == 0 {
                break false;
            }
            if __r == __c {
                break true;
            }
            __q += 1;
        };
        if __hit {
            break __i;
        }
        __p += 1;
        __i += 1;
    }
}

fn f17(a0: Ptr<u8>, a1: Ptr<u8>) -> usize {
    let mut __p = a0.clone();
    let mut __i: usize = 0;
    loop {
        let __c = __p.read();
        if __c == 0 {
            break __i;
        }
        let mut __q = a1.clone();
        let __hit = loop {
            let __r = __q.read();
            if __r == 0 {
                break false;
            }
            if __r == __c {
                break true;
            }
            __q += 1;
        };
        if !__hit {
            break __i;
        }
        __p += 1;
        __i += 1;
    }
}

fn f18(a0: Ptr<u8>, a1: Ptr<u8>) -> Ptr<u8> {
    let mut __p = a0.clone();
    loop {
        let mut __h = __p.clone();
        let mut __n = a1.clone();
        let __matched = loop {
            let __c = __n.read();
            if __c == 0 {
                break true;
            }
            if __h.read() != __c {
                break false;
            }
            __h += 1;
            __n += 1;
        };
        if __matched {
            break __p;
        }
        if __p.read() == 0 {
            break Ptr::null();
        }
        __p += 1;
    }
}

fn f21(a0: Ptr<u8>, a1: Ptr<u8>) -> Ptr<u8> {
    let mut __p = a0.clone();
    loop {
        let __c = __p.read();
        if __c == 0 {
            break Ptr::null();
        }
        let mut __q = a1.clone();
        let __hit = loop {
            let __r = __q.read();
            if __r == 0 {
                break false;
            }
            if __r == __c {
                break true;
            }
            __q += 1;
        };
        if __hit {
            break __p;
        }
        __p += 1;
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
    let mut __p1 = a0.clone();
    let mut __p2 = a1.clone();
    loop {
        let __c1 = __p1.read().to_ascii_lowercase();
        let __c2 = __p2.read().to_ascii_lowercase();
        if __c1 != __c2 {
            break (__c1 as i32) - (__c2 as i32);
        }
        if __c1 == 0 {
            break 0;
        }
        __p1 += 1;
        __p2 += 1;
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
    let mut __p = a0.clone();
    loop {
        let __c = __p.read();
        if __c == a1 as u8 {
            break __p;
        }
        if __c == 0 {
            break Ptr::null();
        }
        __p += 1;
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
    let mut __p = a0.clone();
    let mut __found = Ptr::null();
    loop {
        let __c = __p.read();
        if __c == a1 as u8 {
            __found = __p.clone();
        }
        if __c == 0 {
            break __found;
        }
        __p += 1;
    }
}

fn f14(a0: Ptr<u8>, a1: i32) -> Ptr<u8> {
    let mut __p = a0.clone();
    let mut __found = Ptr::null();
    loop {
        let __c = __p.read();
        if __c == a1 as u8 {
            __found = __p.clone();
        }
        if __c == 0 {
            break __found;
        }
        __p += 1;
    }
}

fn f19(a0: Ptr<u8>, a1: Ptr<u8>) -> Ptr<u8> {
    let mut __p = a0.clone();
    loop {
        let mut __h = __p.clone();
        let mut __n = a1.clone();
        let __matched = loop {
            let __c = __n.read();
            if __c == 0 {
                break true;
            }
            if __h.read() != __c {
                break false;
            }
            __h += 1;
            __n += 1;
        };
        if __matched {
            break __p;
        }
        if __p.read() == 0 {
            break Ptr::null();
        }
        __p += 1;
    }
}

fn f20(a0: Ptr<u8>, a1: Ptr<u8>) -> Ptr<u8> {
    let mut __p = a0.clone();
    loop {
        let mut __h = __p.clone();
        let mut __n = a1.clone();
        let __matched = loop {
            let __c = __n.read();
            if __c == 0 {
                break true;
            }
            if __h.read() != __c {
                break false;
            }
            __h += 1;
            __n += 1;
        };
        if __matched {
            break __p;
        }
        if __p.read() == 0 {
            break Ptr::null();
        }
        __p += 1;
    }
}

fn f22(a0: Ptr<u8>, a1: Ptr<u8>) -> Ptr<u8> {
    let mut __p = a0.clone();
    loop {
        let __c = __p.read();
        if __c == 0 {
            break Ptr::null();
        }
        let mut __q = a1.clone();
        let __hit = loop {
            let __r = __q.read();
            if __r == 0 {
                break false;
            }
            if __r == __c {
                break true;
            }
            __q += 1;
        };
        if __hit {
            break __p;
        }
        __p += 1;
    }
}

fn f23(a0: Ptr<u8>, a1: Ptr<u8>) -> Ptr<u8> {
    let mut __p = a0.clone();
    loop {
        let __c = __p.read();
        if __c == 0 {
            break Ptr::null();
        }
        let mut __q = a1.clone();
        let __hit = loop {
            let __r = __q.read();
            if __r == 0 {
                break false;
            }
            if __r == __c {
                break true;
            }
            __q += 1;
        };
        if __hit {
            break __p;
        }
        __p += 1;
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
