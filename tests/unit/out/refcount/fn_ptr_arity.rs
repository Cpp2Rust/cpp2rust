extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(
    mut a1: i32,
    mut a2: i32,
    mut a3: i32,
    mut a4: i32,
    mut a5: i32,
    mut a6: i32,
    mut a7: i32,
    mut a8: i32,
    mut a9: i32,
    mut a10: i32,
    mut a11: i32,
    mut a12: i32,
    mut a13: i32,
    mut a14: i32,
) -> i32 {
    return 22;
}
pub fn wide_1(
    mut a1: i32,
    mut a2: i32,
    mut a3: i32,
    mut a4: i32,
    mut a5: i32,
    mut a6: i32,
    mut a7: i32,
    mut a8: i32,
    mut a9: i32,
    mut a10: i32,
    mut a11: i32,
    mut a12: i32,
    mut a13: i32,
    mut a14: i32,
    mut a15: i32,
    mut a16: i32,
    mut a17: i32,
    mut a18: i32,
    mut a19: i32,
    mut a20: i32,
    mut a21: i32,
    mut a22: i32,
) -> i32 {
    return (a1 + a22);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut f : FnPtr<fn(i32  ,i32  ,i32  ,i32  ,i32  ,i32  ,i32  ,i32  ,i32  ,i32  ,i32  ,i32  ,i32  ,i32  ,) -> i32  >  = ( FnPtr::<fn(i32  ,i32  ,i32  ,i32  ,i32  ,i32  ,i32  ,i32  ,i32  ,i32  ,i32  ,i32  ,i32  ,i32  ,) -> i32  >::new(foo_0)  )  ;
    assert!((({ f.call(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14,) }) == 22));
    let mut w: FnPtr<
        fn(
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
        ) -> i32,
    > = (FnPtr::<
        fn(
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
        ) -> i32,
    >::new(wide_1));
    assert!(
        (({
            w.call(
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
            )
        }) == 23)
    );
    let l: Value<
        FnPtr<
            fn(
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
            ) -> i32,
        >,
    > = Rc::new(RefCell::new(FnPtr::<
        fn(
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
        ) -> i32,
    >::new(
        |mut a1: i32,
         mut a2: i32,
         mut a3: i32,
         mut a4: i32,
         mut a5: i32,
         mut a6: i32,
         mut a7: i32,
         mut a8: i32,
         mut a9: i32,
         mut a10: i32,
         mut a11: i32,
         mut a12: i32,
         mut a13: i32,
         mut a14: i32,
         mut a15: i32,
         mut a16: i32,
         mut a17: i32,
         mut a18: i32|
         -> i32 {
            {
                return (a1 * a18);
            }
        },
    )));
    assert!(
        (({ (*l.borrow()).call(2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9,) }) == 18)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
