extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn read_0(value: *const i32) -> i32 {
    return (*value);
}
pub static mut selected_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe {
    {
        let mut __tmp_0: i32;
        let mut __tmp_1: i32;
        (unsafe {
            read_0({
                __tmp_0 = 0;
                __tmp_1 = 0;
                {
                    let __a: *const _ = &mut __tmp_0;
                    let __b: *const _ = &mut __tmp_1;
                    if *__b < *__a { __b } else { __a }
                }
            })
        })
    }
});
pub static mut x_2: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut y_3: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut assigned_4: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe {
    {
        let mut __tmp_2: i32;
        let mut __tmp_3: i32;
        ({
            (*std::cell::LazyCell::force_mut(&mut *&raw mut x_2)) = {
                __tmp_2 = 5;
                __tmp_3 = 6;
                (*{
                    let __a: *const _ = &mut __tmp_2;
                    let __b: *const _ = &mut __tmp_3;
                    if *__b < *__a { __b } else { __a }
                })
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut x_2))
        })
    }
});
pub static mut accumulated_5: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe {
    {
        let mut __tmp_4: i32;
        let mut __tmp_5: i32;
        ({
            (*std::cell::LazyCell::force_mut(&mut *&raw mut y_3)) += {
                __tmp_4 = 1;
                __tmp_5 = 2;
                (*{
                    let __a: *const _ = &mut __tmp_4;
                    let __b: *const _ = &mut __tmp_5;
                    if *__a < *__b { __b } else { __a }
                })
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut y_3))
        })
    }
});
pub static mut call_assigned_6: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe {
    {
        let mut __tmp_6: i32;
        ({
            (*std::cell::LazyCell::force_mut(&mut *&raw mut x_2)) = (unsafe {
                __tmp_6 = 5;
                read_0(&mut __tmp_6)
            });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut x_2))
        })
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Item {
    pub value: i32,
}
pub unsafe fn read_array_7(value: *const [i32; 1]) -> i32 {
    return (*value)[(0) as usize];
}
pub unsafe fn read_record_8(value: *const Item) -> i32 {
    return (*value).value;
}
pub static mut aggregate_selected_9: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe {
        {
            let mut __tmp_8: i32;
            let mut __tmp_9: i32;
            let mut __tmp_7: [i32; 1];
            let mut __tmp_11: i32;
            let mut __tmp_12: i32;
            let mut __tmp_10: Item;
            ((unsafe {
                __tmp_7 = [{
                    __tmp_8 = 3;
                    __tmp_9 = 4;
                    (*{
                        let __a: *const _ = &mut __tmp_8;
                        let __b: *const _ = &mut __tmp_9;
                        if *__b < *__a { __b } else { __a }
                    })
                }];
                read_array_7(&mut __tmp_7)
            }) + (unsafe {
                __tmp_10 = Item {
                    value: {
                        __tmp_11 = 5;
                        __tmp_12 = 6;
                        (*{
                            let __a: *const _ = &mut __tmp_11;
                            let __b: *const _ = &mut __tmp_12;
                            if *__a < *__b { __b } else { __a }
                        })
                    },
                };
                read_record_8(&mut __tmp_10)
            }))
        }
    });
pub static mut left_10: std::cell::LazyCell<u64> = std::cell::LazyCell::new(|| unsafe { 3_u64 });
pub static mut right_11: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 7_usize });
pub static mut mapped_selected_12: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe {
        {
            let mut __tmp_13: u64;
            (({
                __tmp_13 = ((*std::cell::LazyCell::force_mut(&mut *&raw mut right_11)) as u64);
                (*{
                    let __a: *const _ =
                        &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut left_10));
                    let __b: *const _ = &mut __tmp_13;
                    if *__b < *__a { __b } else { __a }
                })
            }) as usize)
        }
    });
pub unsafe fn get_13() -> usize {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut value_14));
}
static mut value_14: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    {
        let mut __tmp_14: u64;
        (({
            __tmp_14 = ((*std::cell::LazyCell::force_mut(&mut *&raw mut right_11)) as u64);
            (*{
                let __a: *const _ = &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut left_10));
                let __b: *const _ = &mut __tmp_14;
                if *__a < *__b { __b } else { __a }
            })
        }) as usize)
    }
});
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut selected_1)) == (0)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut assigned_4)) == (5)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut accumulated_5)) == (2)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut call_assigned_6)) == (5)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut aggregate_selected_9)) == (9)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut aggregate_static_15)) == (17)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut mapped_selected_12)) == (3_usize)));
    assert!(((unsafe { get_13() }) == (7_usize)));
    return 0;
}
static mut aggregate_static_15: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe {
    {
        let mut __tmp_16: i32;
        let mut __tmp_17: i32;
        let mut __tmp_15: [i32; 1];
        let mut __tmp_19: i32;
        let mut __tmp_20: i32;
        let mut __tmp_18: Item;
        ((unsafe {
            __tmp_15 = [{
                __tmp_16 = 7;
                __tmp_17 = 8;
                (*{
                    let __a: *const _ = &mut __tmp_16;
                    let __b: *const _ = &mut __tmp_17;
                    if *__a < *__b { __b } else { __a }
                })
            }];
            read_array_7(&mut __tmp_15)
        }) + (unsafe {
            __tmp_18 = Item {
                value: {
                    __tmp_19 = 9;
                    __tmp_20 = 10;
                    (*{
                        let __a: *const _ = &mut __tmp_19;
                        let __b: *const _ = &mut __tmp_20;
                        if *__b < *__a { __b } else { __a }
                    })
                },
            };
            read_record_8(&mut __tmp_18)
        }))
    }
});
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const selected_1);
    std::cell::LazyCell::force(&*&raw const x_2);
    std::cell::LazyCell::force(&*&raw const y_3);
    std::cell::LazyCell::force(&*&raw const assigned_4);
    std::cell::LazyCell::force(&*&raw const accumulated_5);
    std::cell::LazyCell::force(&*&raw const call_assigned_6);
    std::cell::LazyCell::force(&*&raw const aggregate_selected_9);
    std::cell::LazyCell::force(&*&raw const left_10);
    std::cell::LazyCell::force(&*&raw const right_11);
    std::cell::LazyCell::force(&*&raw const mapped_selected_12);
}
