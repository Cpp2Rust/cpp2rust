extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let list: Value<Ptr<libcc2rs::Ifaddrs>> =
        Rc::new(RefCell::new(Ptr::<libcc2rs::Ifaddrs>::null()));
    assert!(
        ((({
            let __out = (list.as_pointer()).clone();
            match nix::ifaddrs::getifaddrs() {
                Ok(__ifas) => {
                    let __list: Vec<nix::ifaddrs::InterfaceAddress> = __ifas.collect();
                    let mut __next = Ptr::<Ifaddrs>::null();
                    for __ifa in __list.iter().rev() {
                        let __node = Ifaddrs::from_interface_address(__ifa);
                        *__node.ifa_next.borrow_mut() = __next.clone();
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
        assert!(
            (((!((*(*(*ifa.borrow()).upgrade().deref()).ifa_name.borrow()).is_null())) as i32)
                != 0)
        );
        if ((((*(*(*ifa.borrow()).upgrade().deref()).ifa_addr.borrow()).is_null()) as i32) != 0) {
            let __rhs = (*(*(*ifa.borrow()).upgrade().deref()).ifa_next.borrow()).clone();
            (*ifa.borrow_mut()) = __rhs;
            continue 'loop_;
        }
        if (((((*(*(*(*(*ifa.borrow()).upgrade().deref()).ifa_addr.borrow())
            .upgrade()
            .deref())
        .sa_family
        .borrow()) as i32)
            != libc::AF_INET) as i32)
            != 0)
        {
            let __rhs = (*(*(*ifa.borrow()).upgrade().deref()).ifa_next.borrow()).clone();
            (*ifa.borrow_mut()) = __rhs;
            continue 'loop_;
        }
        let sin: Value<Ptr<libcc2rs::SockaddrIn>> = Rc::new(RefCell::new(
            (*(*(*ifa.borrow()).upgrade().deref()).ifa_addr.borrow())
                .reinterpret_cast::<libcc2rs::SockaddrIn>(),
        ));
        let lo_be: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([127_u8, 0_u8, 0_u8, 1_u8])));
        if ((((((*(*sin.borrow()).upgrade().deref()).sin_addr.as_pointer())
            as Ptr<libcc2rs::InAddr>)
            .to_any()
            .memcmp(
                &((lo_be.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                4_usize,
            )
            == 0) as i32)
            != 0)
        {
            (*found_loopback.borrow_mut()) = 1;
            assert!(
                ((((*(*(*ifa.borrow()).upgrade().deref()).ifa_flags.borrow()) != 0_u32) as i32)
                    != 0)
            );
            assert!(
                (((!((*(*(*ifa.borrow()).upgrade().deref()).ifa_netmask.borrow()).is_null()))
                    as i32)
                    != 0)
            );
            let mask: Value<Ptr<libcc2rs::SockaddrIn>> = Rc::new(RefCell::new(
                (*(*(*ifa.borrow()).upgrade().deref()).ifa_netmask.borrow())
                    .reinterpret_cast::<libcc2rs::SockaddrIn>(),
            ));
            let mask_be: Value<Box<[u8]>> =
                Rc::new(RefCell::new(Box::new([255_u8, 0_u8, 0_u8, 0_u8])));
            assert!(
                ((((((*(*mask.borrow()).upgrade().deref()).sin_addr.as_pointer())
                    as Ptr<libcc2rs::InAddr>)
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
                    (*(*(*ifa.borrow()).upgrade().deref()).ifa_name.borrow())
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
        let __rhs = (*(*(*ifa.borrow()).upgrade().deref()).ifa_next.borrow()).clone();
        (*ifa.borrow_mut()) = __rhs;
    }
    assert!(((*found_loopback.borrow()) != 0));
    {
        let mut __cur = (*list.borrow()).clone();
        while !__cur.is_null() {
            let __next = __cur.with(|__i| {
                let __name = __i.ifa_name.borrow();
                if !__name.is_null() {
                    __name.delete_array();
                }
                let __addr = __i.ifa_addr.borrow();
                if !__addr.is_null() {
                    __addr.delete();
                }
                let __mask = __i.ifa_netmask.borrow();
                if !__mask.is_null() {
                    __mask.delete();
                }
                (*__i.ifa_next.borrow()).clone()
            });
            __cur.delete();
            __cur = __next;
        }
    };
    assert!(
        (((match nix::net::if_::if_nametoindex(
            Ptr::from_string_literal(b"cpp2rust_no_such_if")
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
