extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn cmp_int_0(mut a: AnyPtr, mut b: AnyPtr) -> i32 {
    let mut x: i32 = (a.reinterpret_cast::<i32>().read());
    let mut y: i32 = (b.reinterpret_cast::<i32>().read());
    return (((x > y) as i32) - ((x < y) as i32));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([5, 2, 9, 1, 7, 3, 8, 4])));
    {
        let __base = ((arr.as_pointer() as Ptr<i32>) as Ptr<i32>)
            .to_any()
            .reinterpret_cast::<u8>();
        for __i in 0..8_usize {
            let mut __min = __i;
            for __j in (__i + 1)..8_usize {
                if cmp_int_0(
                    __base.offset(__j * 4usize).to_any(),
                    __base.offset(__min * 4usize).to_any(),
                ) < 0
                {
                    __min = __j;
                }
            }
            if __min != __i {
                for __b in 0..4usize {
                    let __x = __base.offset(__i * 4usize + __b).read();
                    let __y = __base.offset(__min * 4usize + __b).read();
                    elem!(__base, __i * 4usize + __b).write(__y);
                    elem!(__base, __min * 4usize + __b).write(__x);
                }
            }
        }
    };
    let mut i: i32 = 0;
    'loop_: while (((i < 7) as i32) != 0) {
        assert!(
            ((((*arr.borrow())[(i) as usize] <= (*arr.borrow())[(i + 1) as usize]) as i32) != 0)
        );
        i.prefix_inc();
    }
    let key: Value<i32> = Rc::new(RefCell::new(7));
    let mut hit: Ptr<i32> = {
        let __base = ((arr.as_pointer() as Ptr<i32>) as Ptr<i32>)
            .to_any()
            .reinterpret_cast::<u8>();
        let mut __lo: isize = 0;
        let mut __hi: isize = 8_usize as isize - 1;
        let mut __found = AnyPtr::default();
        while __lo <= __hi && __found.is_null() {
            let __mid = __lo + (__hi - __lo) / 2;
            let __elem = __base.offset(__mid as usize * 4usize);
            let __r = cmp_int_0(
                ((key.as_pointer()) as Ptr<i32>).to_any().clone(),
                __elem.to_any(),
            );
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
    .reinterpret_cast::<i32>();
    assert!((((!((hit).is_null())) as i32) != 0));
    assert!(((((hit.read()) == 7) as i32) != 0));
    let miss_key: Value<i32> = Rc::new(RefCell::new(42));
    let mut miss: Ptr<i32> = {
        let __base = ((arr.as_pointer() as Ptr<i32>) as Ptr<i32>)
            .to_any()
            .reinterpret_cast::<u8>();
        let mut __lo: isize = 0;
        let mut __hi: isize = 8_usize as isize - 1;
        let mut __found = AnyPtr::default();
        while __lo <= __hi && __found.is_null() {
            let __mid = __lo + (__hi - __lo) / 2;
            let __elem = __base.offset(__mid as usize * 4usize);
            let __r = cmp_int_0(
                ((miss_key.as_pointer()) as Ptr<i32>).to_any().clone(),
                __elem.to_any(),
            );
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
    .reinterpret_cast::<i32>();
    assert!(((((miss).is_null()) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
