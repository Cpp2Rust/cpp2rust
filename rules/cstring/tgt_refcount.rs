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
    let tmp: Vec<u8> = (0..a2)
        .map(|i| a1.reinterpret_cast::<u8>().offset(i).read())
        .collect();
    for i in 0..a2 {
        a0.reinterpret_cast::<u8>().offset(i).write(tmp[i]);
    }
    a0.clone()
}

fn f5(a0: Ptr<u8>, a1: i32) -> Ptr<u8> {
    let mut i: usize = 0;
    loop {
        let c = a0.offset(i).read();
        if c == a1 as u8 {
            break a0.offset(i);
        }
        if c == 0 {
            break Ptr::null();
        }
        i += 1;
    }
}

unsafe fn f7(a0: Ptr<u8>) -> usize {
    let mut i: usize = 0;
    while a0.offset(i).read() != 0 {
        i += 1;
    }
    i
}

fn f8(a0: Ptr<u8>, a1: Ptr<u8>) -> i32 {
    let mut i: usize = 0;
    loop {
        let c1 = a0.offset(i).read();
        let c2 = a1.offset(i).read();
        if c1 != c2 {
            break (c1 as i32) - (c2 as i32);
        }
        if c1 == 0 {
            break 0;
        }
        i += 1;
    }
}

fn f9(a0: Ptr<u8>, a1: Ptr<u8>, a2: usize) -> i32 {
    let mut i: usize = 0;
    loop {
        if i == a2 {
            break 0;
        }
        let c1 = a0.offset(i).read();
        let c2 = a1.offset(i).read();
        if c1 != c2 {
            break (c1 as i32) - (c2 as i32);
        }
        if c1 == 0 {
            break 0;
        }
        i += 1;
    }
}

fn f10(a0: AnyPtr, a1: i32, a2: usize) -> AnyPtr {
    match (0..a2).find(|&i| a0.reinterpret_cast::<u8>().offset(i).read() == a1 as u8) {
        Some(i) => a0.reinterpret_cast::<u8>().offset(i).to_any(),
        None => Ptr::<u8>::null().to_any(),
    }
}

fn f11(a0: Ptr<u8>, a1: i32) -> Ptr<u8> {
    let mut i: usize = 0;
    let mut found = Ptr::null();
    loop {
        let c = a0.offset(i).read();
        if c == a1 as u8 {
            found = a0.offset(i);
        }
        if c == 0 {
            break found;
        }
        i += 1;
    }
}

fn f15(a0: Ptr<u8>) -> Ptr<u8> {
    libcc2rs::strdup_refcount(a0)
}

fn f16(a0: Ptr<u8>, a1: Ptr<u8>) -> usize {
    let mut i: usize = 0;
    loop {
        let c = a0.offset(i).read();
        if c == 0 {
            break i;
        }
        let mut j: usize = 0;
        let hit = loop {
            let r = a1.offset(j).read();
            if r == 0 {
                break false;
            }
            if r == c {
                break true;
            }
            j += 1;
        };
        if hit {
            break i;
        }
        i += 1;
    }
}

fn f17(a0: Ptr<u8>, a1: Ptr<u8>) -> usize {
    let mut i: usize = 0;
    loop {
        let c = a0.offset(i).read();
        if c == 0 {
            break i;
        }
        let mut j: usize = 0;
        let hit = loop {
            let r = a1.offset(j).read();
            if r == 0 {
                break false;
            }
            if r == c {
                break true;
            }
            j += 1;
        };
        if !hit {
            break i;
        }
        i += 1;
    }
}

fn f18(a0: Ptr<u8>, a1: Ptr<u8>) -> Ptr<u8> {
    let mut s: usize = 0;
    loop {
        let mut i: usize = 0;
        let matched = loop {
            let n = a1.offset(i).read();
            if n == 0 {
                break true;
            }
            if a0.offset(s + i).read() != n {
                break false;
            }
            i += 1;
        };
        if matched {
            break a0.offset(s);
        }
        if a0.offset(s).read() == 0 {
            break Ptr::null();
        }
        s += 1;
    }
}

fn f21(a0: Ptr<u8>, a1: Ptr<u8>) -> Ptr<u8> {
    let mut i: usize = 0;
    loop {
        let c = a0.offset(i).read();
        if c == 0 {
            break Ptr::null();
        }
        let mut j: usize = 0;
        let hit = loop {
            let r = a1.offset(j).read();
            if r == 0 {
                break false;
            }
            if r == c {
                break true;
            }
            j += 1;
        };
        if hit {
            break a0.offset(i);
        }
        i += 1;
    }
}

#[cfg(target_os = "linux")]
fn f24(a0: AnyPtr, a1: i32, a2: usize) -> AnyPtr {
    match (0..a2)
        .rev()
        .find(|&i| a0.reinterpret_cast::<u8>().offset(i).read() == a1 as u8)
    {
        Some(i) => a0.reinterpret_cast::<u8>().offset(i).to_any(),
        None => Ptr::<u8>::null().to_any(),
    }
}

fn f27(a0: Ptr<u8>, a1: Ptr<u8>) -> i32 {
    let mut i: usize = 0;
    loop {
        let c1 = a0.offset(i).read().to_ascii_lowercase();
        let c2 = a1.offset(i).read().to_ascii_lowercase();
        if c1 != c2 {
            break (c1 as i32) - (c2 as i32);
        }
        if c1 == 0 {
            break 0;
        }
        i += 1;
    }
}

#[cfg(target_os = "linux")]
fn f28(a0: i32, a1: Ptr<u8>, a2: usize) -> Ptr<u8> {
    let msg = std::io::Error::from_raw_os_error(a0).to_string();
    let len = msg.len().min(a2.saturating_sub(1));
    for i in 0..len {
        a1.offset(i).write(msg.as_bytes()[i]);
    }
    if a2 > 0 {
        a1.offset(len).write(0);
    }
    a1
}

#[cfg(target_os = "macos")]
fn f28(a0: i32, a1: Ptr<u8>, a2: usize) -> i32 {
    let msg = std::io::Error::from_raw_os_error(a0).to_string();
    let len = msg.len().min(a2.saturating_sub(1));
    for i in 0..len {
        a1.offset(i).write(msg.as_bytes()[i]);
    }
    if a2 > 0 {
        a1.offset(len).write(0);
    }
    0
}
