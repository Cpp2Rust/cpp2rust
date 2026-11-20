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
    libcc2rs::fprintf(
        (out).clone(),
        (fmt).clone(),
        &Box::<[VaArg]>::from([((args_2).clone()).into(), (args_3).into()]),
    );
}
pub fn log_to_3(mut out: Ptr<CFile>, mut fmt: Ptr<i8>, mut args: i32) {
    libcc2rs::fprintf(
        (out).clone(),
        (fmt).clone(),
        &Box::<[VaArg]>::from([(args).into()]),
    );
}
pub fn log_fmt_4(mut fmt: Ptr<i8>, mut v: i32) {
    libcc2rs::fprintf(
        libcc2rs::c_stdout(),
        (fmt).clone(),
        &Box::<[VaArg]>::from([(v).into()]),
    );
}
pub fn put_5(out: Ptr<CFile>, mut s: Ptr<i8>) {
    {
        let mut __tmp_0 = { (out).clone() };
        match s.with_c_bytes(|__bytes| __tmp_0.with_mut(|__f| __f.write(__bytes)) == __bytes.len())
        {
            true => 0,
            false => -1,
        }
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    fprintf!(
        libcc2rs::c_stdout(),
        b"%s\n",
        Ptr::<i8>::from_string_literal(b"fprintf stdout")
    );
    fprintf!(libcc2rs::c_stdout(), b"%d %u %ld\n", 1, 2_u32, 3_i64);
    fprintf!(libcc2rs::c_stdout(), b"hello world");
    let mut in_: Ptr<CFile> = libcc2rs::c_stdin();
    assert!(!((in_).is_null()));
    printf!(b"%s\n", Ptr::<i8>::from_string_literal(b"printf"));
    printf!(b"hello world");
    let s: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"a string").to_c_bytes();
        __bytes.push(0);
        __bytes
    }));
    printf!(b"%s\n", s.decay());
    printf!(
        b"%s\n",
        Rc::new(RefCell::new(
            ({
                fn_0({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"foo").to_c_bytes();
                    __bytes.push(0);
                    __bytes
                })
            })
        ))
        .decay()
    );
    printf!(b"%s\n", ({ fn2_1(s.as_pointer(),) }).decay());
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
    printf!(b"{} %ju %jd\n", 5_u64, (-6_i32 as i64));
    let mut fp: Ptr<CFile> = libcc2rs::c_stdout();
    ({ put_5((fp).clone(), Ptr::<i8>::from_string_literal(b"value\n")) });
    let mut bytes: Ptr<i8> = Ptr::<i8>::from_string_literal(b"\xe9\xc3\xa9");
    printf!(
        b"\xff%s|%c|%.1s|%5s\n",
        (bytes).clone(),
        ((elem!(bytes, 0).read()) as i32),
        bytes.offset((1) as isize),
        (bytes).clone()
    );
    fprintf!(libcc2rs::c_stderr(), b"\xff%s\n", (bytes).clone());
    printf!(b"no newline, ");
    ({
        put_5(
            (fp).clone(),
            Ptr::<i8>::from_string_literal(b"then fputs\n"),
        )
    });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
