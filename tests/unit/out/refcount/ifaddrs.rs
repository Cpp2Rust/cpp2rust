extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let list: Value<Ptr<libcc2rs::Ifaddrs>> =
        Rc::new(RefCell::new(Ptr::<libcc2rs::Ifaddrs>::null()));
    assert!(
        ((({
            let __out = (list.as_pointer());
            match nix::ifaddrs::getifaddrs() {
                Ok(__ifas) => {
                    let __list: Vec<nix::ifaddrs::InterfaceAddress> = __ifas.collect();
                    let mut __next = Ptr::<Ifaddrs>::null();
                    for __ifa in __list.iter().rev() {
                        let mut __node = Ifaddrs::from_interface_address(__ifa);
                        __node.ifa_next = __next.clone();
                        __next = Ptr::alloc(__node);
                    }
                    __out.write(__next);
                    0
                }
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!((((!((*list.borrow()).is_null())) as i32) != 0));
    let found_loopback: Value<i32> = Rc::new(RefCell::new(0));
    let ifa: Value<Ptr<libcc2rs::Ifaddrs>> =
        Rc::new(RefCell::new(Ptr::<libcc2rs::Ifaddrs>::null()));
    (*ifa.borrow_mut()) = (*list.borrow()).clone();
    'loop_: while (((!((*ifa.borrow()).is_null())) as i32) != 0) {
        assert!((((!(((*ifa.borrow()).with(|__s| __s.ifa_name.clone())).is_null())) as i32) != 0));
        if (((((*ifa.borrow()).with(|__s| __s.ifa_addr.clone())).is_null()) as i32) != 0) {
            (*ifa.borrow_mut()) = { (*ifa.borrow()).with(|__s| __s.ifa_next.clone()) };
            continue 'loop_;
        }
        if (((((*ifa.borrow())
            .with(|__s| __s.ifa_addr.clone())
            .with(|__s| __s.sa_family) as i32)
            != libc::AF_INET) as i32)
            != 0)
        {
            (*ifa.borrow_mut()) = { (*ifa.borrow()).with(|__s| __s.ifa_next.clone()) };
            continue 'loop_;
        }
        let sin: Value<Ptr<libcc2rs::SockaddrIn>> = Rc::new(RefCell::new(
            (*ifa.borrow())
                .with(|__s| __s.ifa_addr.clone())
                .reinterpret_cast::<libcc2rs::SockaddrIn>(),
        ));
        let lo_be: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([127_u8, 0_u8, 0_u8, 1_u8])));
        if (((((field_ptr!((*sin.borrow()), sin_addr)) as Ptr<libcc2rs::InAddr>)
            .to_any()
            .memcmp(
                &((lo_be.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                4_usize,
            )
            == 0) as i32)
            != 0)
        {
            (*found_loopback.borrow_mut()) = 1;
            assert!(((((*ifa.borrow()).with(|__s| __s.ifa_flags) != 0_u32) as i32) != 0));
            assert!(
                (((!(((*ifa.borrow()).with(|__s| __s.ifa_netmask.clone())).is_null())) as i32)
                    != 0)
            );
            let mask: Value<Ptr<libcc2rs::SockaddrIn>> = Rc::new(RefCell::new(
                (*ifa.borrow())
                    .with(|__s| __s.ifa_netmask.clone())
                    .reinterpret_cast::<libcc2rs::SockaddrIn>(),
            ));
            let mask_be: Value<Box<[u8]>> =
                Rc::new(RefCell::new(Box::new([255_u8, 0_u8, 0_u8, 0_u8])));
            assert!(
                (((((field_ptr!((*mask.borrow()), sin_addr)) as Ptr<libcc2rs::InAddr>)
                    .to_any()
                    .memcmp(
                        &((mask_be.as_pointer() as Ptr::<u8>) as Ptr::<u8>).to_any(),
                        4_usize
                    )
                    == 0) as i32)
                    != 0)
            );
            assert!(
                (((match nix::net::if_::if_nametoindex(
                    (*ifa.borrow())
                        .with(|__s| __s.ifa_name.clone())
                        .to_rust_string()
                        .as_str()
                ) {
                    Ok(__i) => __i,
                    Err(__e) => {
                        libcc2rs::cpp2rust_errno().write(__e as i32);
                        0
                    }
                } > 0_u32) as i32)
                    != 0)
            );
        }
        (*ifa.borrow_mut()) = { (*ifa.borrow()).with(|__s| __s.ifa_next.clone()) };
    }
    assert!(((*found_loopback.borrow()) != 0));
    {
        let mut __cur = (*list.borrow()).clone();
        while !__cur.is_null() {
            let __next = __cur.with(|__i| {
                __i.ifa_name.delete();
                __i.ifa_addr.delete();
                __i.ifa_netmask.delete();
                __i.ifa_next.clone()
            });
            __cur.delete();
            __cur = __next;
        }
    };
    assert!(
        (((match nix::net::if_::if_nametoindex(
            Ptr::<u8>::from_string_literal(b"cpp2rust_no_such_if")
                .to_rust_string()
                .as_str()
        ) {
            Ok(__i) => __i,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                0
            }
        } == 0_u32) as i32)
            != 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
