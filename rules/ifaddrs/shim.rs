// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::{ByteRepr, Ptr, Sockaddr, SockaddrIn, SockaddrIn6, SockaddrStorage, Value};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct Ifaddrs {
    pub ifa_next: Value<Ptr<Ifaddrs>>,
    pub ifa_name: Value<Ptr<u8>>,
    pub ifa_flags: Value<u32>,
    pub ifa_addr: Value<Ptr<Sockaddr>>,
    pub ifa_netmask: Value<Ptr<Sockaddr>>,
}

impl Ifaddrs {
    pub fn from_interface_address(ifa: &nix::ifaddrs::InterfaceAddress) -> Self {
        fn mk_addr(ss: Option<&nix::sys::socket::SockaddrStorage>) -> Ptr<Sockaddr> {
            match ss {
                None => Ptr::null(),
                Some(a) => match (a.as_sockaddr_in(), a.as_sockaddr_in6()) {
                    (Some(v4), _) => {
                        let l = ::libc::sockaddr_in::from(*v4);
                        let st = Ptr::alloc(SockaddrStorage::default());
                        st.reinterpret_cast::<SockaddrIn>()
                            .write(SockaddrIn::from_libc(&l));
                        st.reinterpret_cast::<Sockaddr>()
                    }
                    (None, Some(v6)) => {
                        let l = ::libc::sockaddr_in6::from(*v6);
                        let st = Ptr::alloc(SockaddrStorage::default());
                        st.reinterpret_cast::<SockaddrIn6>()
                            .write(SockaddrIn6::from_libc(&l));
                        st.reinterpret_cast::<Sockaddr>()
                    }
                    (None, None) => Ptr::null(),
                },
            }
        }
        let node = Ifaddrs::default();
        let mut name = ifa.interface_name.clone().into_bytes();
        name.push(0);
        *node.ifa_name.borrow_mut() = Ptr::alloc_array(name.into_boxed_slice());
        *node.ifa_flags.borrow_mut() = ifa.flags.bits() as u32;
        *node.ifa_addr.borrow_mut() = mk_addr(ifa.address.as_ref());
        *node.ifa_netmask.borrow_mut() = mk_addr(ifa.netmask.as_ref());
        node
    }
}

impl Clone for Ifaddrs {
    fn clone(&self) -> Self {
        Self {
            ifa_next: Rc::new(RefCell::new(self.ifa_next.borrow().clone())),
            ifa_name: Rc::new(RefCell::new(self.ifa_name.borrow().clone())),
            ifa_flags: Rc::new(RefCell::new(*self.ifa_flags.borrow())),
            ifa_addr: Rc::new(RefCell::new(self.ifa_addr.borrow().clone())),
            ifa_netmask: Rc::new(RefCell::new(self.ifa_netmask.borrow().clone())),
        }
    }
}

impl ByteRepr for Ifaddrs {}

impl ByteRepr for ::libc::ifaddrs {}
