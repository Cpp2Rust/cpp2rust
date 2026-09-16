extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn cmp_int_0(a: AnyPtr, b: AnyPtr) -> i32 {
    let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
    let b: Value<AnyPtr> = Rc::new(RefCell::new(b));
    let x: Value<i32> = Rc::new(RefCell::new(
        ((*a.borrow()).reinterpret_cast::<i32>().read()),
    ));
    let y: Value<i32> = Rc::new(RefCell::new(
        ((*b.borrow()).reinterpret_cast::<i32>().read()),
    ));
    return ((((*x.borrow()) > (*y.borrow())) as i32) - (((*x.borrow()) < (*y.borrow())) as i32));
}
pub fn main() {
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
                    __base.offset(__j * ::std::mem::size_of::<i32>()).to_any(),
                    __base.offset(__min * ::std::mem::size_of::<i32>()).to_any(),
                ) < 0
                {
                    __min = __j;
                }
            }
            if __min != __i {
                for __b in 0..::std::mem::size_of::<i32>() {
                    let __x = __base
                        .offset(__i * ::std::mem::size_of::<i32>() + __b)
                        .read();
                    let __y = __base
                        .offset(__min * ::std::mem::size_of::<i32>() + __b)
                        .read();
                    __base
                        .offset(__i * ::std::mem::size_of::<i32>() + __b)
                        .write(__y);
                    __base
                        .offset(__min * ::std::mem::size_of::<i32>() + __b)
                        .write(__x);
                }
            }
        }
    };
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((((*i.borrow()) < 7) as i32) != 0) {
        assert!(
            ((((*arr.borrow())[(*i.borrow()) as usize]
                <= (*arr.borrow())[((*i.borrow()) + 1) as usize]) as i32)
                != 0)
        );
        (*i.borrow_mut()).prefix_inc();
    }
    let key: Value<i32> = Rc::new(RefCell::new(7));
    let hit: Value<Ptr<i32>> = Rc::new(RefCell::new(
        {
            let __base = ((arr.as_pointer() as Ptr<i32>) as Ptr<i32>)
                .to_any()
                .reinterpret_cast::<u8>();
            let mut __lo: isize = 0;
            let mut __hi: isize = 8_usize as isize - 1;
            let mut __found = AnyPtr::default();
            while __lo <= __hi && __found.is_null() {
                let __mid = __lo + (__hi - __lo) / 2;
                let __elem = __base.offset(__mid as usize * ::std::mem::size_of::<i32>());
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
        .reinterpret_cast::<i32>(),
    ));
    assert!((((!((*hit.borrow()).is_null())) as i32) != 0));
    assert!((((((*hit.borrow()).read()) == 7) as i32) != 0));
    let miss_key: Value<i32> = Rc::new(RefCell::new(42));
    let miss: Value<Ptr<i32>> = Rc::new(RefCell::new(
        {
            let __base = ((arr.as_pointer() as Ptr<i32>) as Ptr<i32>)
                .to_any()
                .reinterpret_cast::<u8>();
            let mut __lo: isize = 0;
            let mut __hi: isize = 8_usize as isize - 1;
            let mut __found = AnyPtr::default();
            while __lo <= __hi && __found.is_null() {
                let __mid = __lo + (__hi - __lo) / 2;
                let __elem = __base.offset(__mid as usize * ::std::mem::size_of::<i32>());
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
        .reinterpret_cast::<i32>(),
    ));
    assert!(((((*miss.borrow()).is_null()) as i32) != 0));
    return 0;
}
