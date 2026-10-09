extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fn_0(mut v: Vec<i8>) -> Vec<i8> {
    return {
        let mut r = (v).clone();
        r.pop();
        Ptr::<i8>::from_string_literal(b" str").with_c_str(|__s| r.extend_from_slice(__s));
        r.push(0);
        r
    };
}
pub fn fn2_1(v: Ptr<Vec<i8>>) -> Ptr<Vec<i8>> {
    return (v).clone();
}
pub fn log_to_2(mut out: Ptr<CFile>, mut fmt: Ptr<i8>, mut args_2: Ptr<i8>, mut args_3: i32) {
    {
        let __s = libcc2rs::format_c(
            &fmt.to_rust_string(),
            &[((args_2).clone()).into(), (args_3).into()],
        );
        out.with_mut(|__f| __f.write(__s.as_bytes())) as i32
    };
}
pub fn log_to_3(mut out: Ptr<CFile>, mut fmt: Ptr<i8>, mut args: i32) {
    {
        let __s = libcc2rs::format_c(&fmt.to_rust_string(), &[(args).into()]);
        out.with_mut(|__f| __f.write(__s.as_bytes())) as i32
    };
}
pub fn log_fmt_4(mut fmt: Ptr<i8>, mut v: i32) {
    {
        let __s = libcc2rs::format_c(&fmt.to_rust_string(), &[(v).into()]);
        libcc2rs::c_stdout().with_mut(|__f| __f.write(__s.as_bytes())) as i32
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    println!("{}", Ptr::<i8>::from_string_literal(b"fprintf stdout"));
    println!("{} {} {}", 1, 2_u32, 3_i64);
    print!("hello world");
    let mut in_: Ptr<CFile> = libcc2rs::c_stdin();
    assert!(!((in_).is_null()));
    println!("{}", Ptr::<i8>::from_string_literal(b"printf"));
    print!("hello world");
    let s: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"a string").to_c_bytes();
        __bytes.push(0);
        __bytes
    }));
    println!("{}", (s.as_pointer() as Ptr<i8>));
    println!(
        "{}",
        (Rc::new(RefCell::new(
            ({
                fn_0({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"foo").to_c_bytes();
                    __bytes.push(0);
                    __bytes
                })
            })
        ))
        .as_pointer() as Ptr<i8>)
    );
    println!(
        "{}",
        (Ptr::<Vec<i8>>::decay(&({ fn2_1(s.as_pointer(),) })) as Ptr<i8>)
    );
    ({
        log_to_2(
            libcc2rs::c_stdout(),
            Ptr::<i8>::from_string_literal(b"%s %d\n"),
            Ptr::<i8>::from_string_literal(b"runtime"),
            4,
        )
    });
    ({
        log_to_3(
            libcc2rs::c_stderr(),
            Ptr::<i8>::from_string_literal(b"%d\n"),
            5,
        )
    });
    ({ log_fmt_4(Ptr::<i8>::from_string_literal(b"%d\n"), 6) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
