// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f2(a0: AnyPtr) {
    libcc2rs::free_refcount(a0)
}

fn f3(a0: usize) -> AnyPtr {
    libcc2rs::malloc_refcount(a0)
}

fn f4(a0: AnyPtr, a1: usize) -> AnyPtr {
    libcc2rs::realloc_refcount(a0, a1)
}

fn f5(a0: usize, a1: usize) -> AnyPtr {
    libcc2rs::calloc_refcount(a0, a1)
}

fn f8(a0: AnyPtr, a1: AnyPtr, a2: usize, a3: usize, a4: fn(AnyPtr, AnyPtr) -> i32) -> AnyPtr {
    let __base = a1.reinterpret_cast::<u8>();
    let mut __lo: isize = 0;
    let mut __hi: isize = a2 as isize - 1;
    let mut __found = AnyPtr::default();
    while __lo <= __hi && __found.is_null() {
        let __mid = __lo + (__hi - __lo) / 2;
        let __elem = __base.offset(__mid as usize * a3);
        let __r = a4(a0.clone(), __elem.to_any());
        if __r == 0 {
            __found = __elem.to_any();
        } else if __r < 0 {
            __hi = __mid - 1;
        } else {
            __lo = __mid + 1;
        }
    }
    __found
}

fn f9(a0: AnyPtr, a1: usize, a2: usize, a3: fn(AnyPtr, AnyPtr) -> i32) {
    let __base = a0.reinterpret_cast::<u8>();
    for __i in 0..a1 {
        let mut __min = __i;
        for __j in (__i + 1)..a1 {
            if a3(
                __base.offset(__j * a2).to_any(),
                __base.offset(__min * a2).to_any(),
            ) < 0
            {
                __min = __j;
            }
        }
        if __min != __i {
            for __b in 0..a2 {
                let __x = __base.offset(__i * a2 + __b).read();
                let __y = __base.offset(__min * a2 + __b).read();
                __base.offset(__i * a2 + __b).write(__y);
                __base.offset(__min * a2 + __b).write(__x);
            }
        }
    }
}
