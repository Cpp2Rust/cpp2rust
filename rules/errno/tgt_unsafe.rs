// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1() -> *mut i32 {
    libcc2rs::cpp2rust_errno_unsafe()
}

unsafe fn f2() -> i32 {
    libc::E2BIG
}

unsafe fn f3() -> i32 {
    libc::EACCES
}

unsafe fn f4() -> i32 {
    libc::EADDRINUSE
}

unsafe fn f5() -> i32 {
    libc::EADDRNOTAVAIL
}

unsafe fn f6() -> i32 {
    libc::EAFNOSUPPORT
}

unsafe fn f7() -> i32 {
    libc::EAGAIN
}

unsafe fn f8() -> i32 {
    libc::EALREADY
}

unsafe fn f9() -> i32 {
    libc::EBADF
}

unsafe fn f10() -> i32 {
    libc::EBADMSG
}

unsafe fn f11() -> i32 {
    libc::EBUSY
}

unsafe fn f12() -> i32 {
    libc::ECANCELED
}

unsafe fn f13() -> i32 {
    libc::ECHILD
}

unsafe fn f14() -> i32 {
    libc::ECONNABORTED
}

unsafe fn f15() -> i32 {
    libc::ECONNREFUSED
}

unsafe fn f16() -> i32 {
    libc::ECONNRESET
}

unsafe fn f17() -> i32 {
    libc::EDEADLK
}

unsafe fn f18() -> i32 {
    libc::EDESTADDRREQ
}

unsafe fn f19() -> i32 {
    libc::EDOM
}

unsafe fn f20() -> i32 {
    libc::EDQUOT
}

unsafe fn f21() -> i32 {
    libc::EEXIST
}

unsafe fn f22() -> i32 {
    libc::EFAULT
}

unsafe fn f23() -> i32 {
    libc::EFBIG
}

unsafe fn f24() -> i32 {
    libc::EHOSTDOWN
}

unsafe fn f25() -> i32 {
    libc::EHOSTUNREACH
}

unsafe fn f26() -> i32 {
    libc::EIDRM
}

unsafe fn f27() -> i32 {
    libc::EILSEQ
}

unsafe fn f28() -> i32 {
    libc::EINPROGRESS
}

unsafe fn f29() -> i32 {
    libc::EINTR
}

unsafe fn f30() -> i32 {
    libc::EINVAL
}

unsafe fn f31() -> i32 {
    libc::EIO
}

unsafe fn f32() -> i32 {
    libc::EISCONN
}

unsafe fn f33() -> i32 {
    libc::EISDIR
}

unsafe fn f34() -> i32 {
    libc::ELOOP
}

unsafe fn f35() -> i32 {
    libc::EMFILE
}

unsafe fn f36() -> i32 {
    libc::EMLINK
}

unsafe fn f37() -> i32 {
    libc::EMSGSIZE
}

unsafe fn f38() -> i32 {
    libc::EMULTIHOP
}

unsafe fn f39() -> i32 {
    libc::ENAMETOOLONG
}

unsafe fn f40() -> i32 {
    libc::ENETDOWN
}

unsafe fn f41() -> i32 {
    libc::ENETRESET
}

unsafe fn f42() -> i32 {
    libc::ENETUNREACH
}

unsafe fn f43() -> i32 {
    libc::ENFILE
}

unsafe fn f44() -> i32 {
    libc::ENOBUFS
}

unsafe fn f45() -> i32 {
    libc::ENODATA
}

unsafe fn f46() -> i32 {
    libc::ENODEV
}

unsafe fn f47() -> i32 {
    libc::ENOENT
}

unsafe fn f48() -> i32 {
    libc::ENOEXEC
}

unsafe fn f49() -> i32 {
    libc::ENOLCK
}

unsafe fn f50() -> i32 {
    libc::ENOLINK
}

unsafe fn f51() -> i32 {
    libc::ENOMEM
}

unsafe fn f52() -> i32 {
    libc::ENOMSG
}

unsafe fn f53() -> i32 {
    libc::ENOPROTOOPT
}

unsafe fn f54() -> i32 {
    libc::ENOSPC
}

unsafe fn f55() -> i32 {
    libc::ENOSR
}

unsafe fn f56() -> i32 {
    libc::ENOSTR
}

unsafe fn f57() -> i32 {
    libc::ENOSYS
}

unsafe fn f58() -> i32 {
    libc::ENOTCONN
}

unsafe fn f59() -> i32 {
    libc::ENOTDIR
}

unsafe fn f60() -> i32 {
    libc::ENOTEMPTY
}

unsafe fn f61() -> i32 {
    libc::ENOTRECOVERABLE
}

unsafe fn f62() -> i32 {
    libc::ENOTSOCK
}

unsafe fn f63() -> i32 {
    libc::ENOTSUP
}

unsafe fn f64() -> i32 {
    libc::ENOTBLK
}

unsafe fn f65() -> i32 {
    libc::ENOTTY
}

unsafe fn f66() -> i32 {
    libc::ENXIO
}

unsafe fn f67() -> i32 {
    libc::EOPNOTSUPP
}

unsafe fn f68() -> i32 {
    libc::EOVERFLOW
}

unsafe fn f69() -> i32 {
    libc::EOWNERDEAD
}

unsafe fn f70() -> i32 {
    libc::EPERM
}

unsafe fn f71() -> i32 {
    libc::EPFNOSUPPORT
}

unsafe fn f72() -> i32 {
    libc::EPIPE
}

unsafe fn f73() -> i32 {
    libc::EPROTO
}

unsafe fn f74() -> i32 {
    libc::EPROTONOSUPPORT
}

unsafe fn f75() -> i32 {
    libc::EPROTOTYPE
}

unsafe fn f76() -> i32 {
    libc::ERANGE
}

unsafe fn f77() -> i32 {
    libc::EREMOTE
}

unsafe fn f78() -> i32 {
    libc::EROFS
}

unsafe fn f79() -> i32 {
    libc::ESHUTDOWN
}

unsafe fn f80() -> i32 {
    libc::ESOCKTNOSUPPORT
}

unsafe fn f81() -> i32 {
    libc::ESPIPE
}

unsafe fn f82() -> i32 {
    libc::ESRCH
}

unsafe fn f83() -> i32 {
    libc::ESTALE
}

unsafe fn f84() -> i32 {
    libc::ETIME
}

unsafe fn f85() -> i32 {
    libc::ETIMEDOUT
}

unsafe fn f86() -> i32 {
    libc::ETOOMANYREFS
}

unsafe fn f87() -> i32 {
    libc::ETXTBSY
}

unsafe fn f88() -> i32 {
    libc::EUSERS
}

unsafe fn f89() -> i32 {
    libc::EWOULDBLOCK
}

unsafe fn f90() -> i32 {
    libc::EXDEV
}

#[cfg(target_os = "linux")]
unsafe fn f91() -> i32 {
    libc::EADV
}

#[cfg(target_os = "linux")]
unsafe fn f92() -> i32 {
    libc::EBADE
}

#[cfg(target_os = "linux")]
unsafe fn f93() -> i32 {
    libc::EBADFD
}

#[cfg(target_os = "linux")]
unsafe fn f94() -> i32 {
    libc::EBADR
}

#[cfg(target_os = "linux")]
unsafe fn f95() -> i32 {
    libc::EBADRQC
}

#[cfg(target_os = "linux")]
unsafe fn f96() -> i32 {
    libc::EBADSLT
}

#[cfg(target_os = "linux")]
unsafe fn f97() -> i32 {
    libc::EBFONT
}

#[cfg(target_os = "linux")]
unsafe fn f98() -> i32 {
    libc::ECHRNG
}

#[cfg(target_os = "linux")]
unsafe fn f99() -> i32 {
    libc::ECOMM
}

#[cfg(target_os = "linux")]
unsafe fn f100() -> i32 {
    libc::EDEADLOCK
}

#[cfg(target_os = "linux")]
unsafe fn f101() -> i32 {
    libc::EDOTDOT
}

#[cfg(target_os = "linux")]
unsafe fn f102() -> i32 {
    libc::EHWPOISON
}

#[cfg(target_os = "linux")]
unsafe fn f103() -> i32 {
    libc::EISNAM
}

#[cfg(target_os = "linux")]
unsafe fn f104() -> i32 {
    libc::EKEYEXPIRED
}

#[cfg(target_os = "linux")]
unsafe fn f105() -> i32 {
    libc::EKEYREJECTED
}

#[cfg(target_os = "linux")]
unsafe fn f106() -> i32 {
    libc::EKEYREVOKED
}

#[cfg(target_os = "linux")]
unsafe fn f107() -> i32 {
    libc::EL2HLT
}

#[cfg(target_os = "linux")]
unsafe fn f108() -> i32 {
    libc::EL2NSYNC
}

#[cfg(target_os = "linux")]
unsafe fn f109() -> i32 {
    libc::EL3HLT
}

#[cfg(target_os = "linux")]
unsafe fn f110() -> i32 {
    libc::EL3RST
}

#[cfg(target_os = "linux")]
unsafe fn f111() -> i32 {
    libc::ELIBACC
}

#[cfg(target_os = "linux")]
unsafe fn f112() -> i32 {
    libc::ELIBBAD
}

#[cfg(target_os = "linux")]
unsafe fn f113() -> i32 {
    libc::ELIBEXEC
}

#[cfg(target_os = "linux")]
unsafe fn f114() -> i32 {
    libc::ELIBMAX
}

#[cfg(target_os = "linux")]
unsafe fn f115() -> i32 {
    libc::ELIBSCN
}

#[cfg(target_os = "linux")]
unsafe fn f116() -> i32 {
    libc::ELNRNG
}

#[cfg(target_os = "linux")]
unsafe fn f117() -> i32 {
    libc::EMEDIUMTYPE
}

#[cfg(target_os = "linux")]
unsafe fn f118() -> i32 {
    libc::ENAVAIL
}

#[cfg(target_os = "linux")]
unsafe fn f119() -> i32 {
    libc::ENOANO
}

#[cfg(target_os = "linux")]
unsafe fn f120() -> i32 {
    libc::ENOCSI
}

#[cfg(target_os = "linux")]
unsafe fn f121() -> i32 {
    libc::ENOKEY
}

#[cfg(target_os = "linux")]
unsafe fn f122() -> i32 {
    libc::ENOMEDIUM
}

#[cfg(target_os = "linux")]
unsafe fn f123() -> i32 {
    libc::ENONET
}

#[cfg(target_os = "linux")]
unsafe fn f124() -> i32 {
    libc::ENOPKG
}

#[cfg(target_os = "linux")]
unsafe fn f125() -> i32 {
    libc::ENOTNAM
}

#[cfg(target_os = "linux")]
unsafe fn f126() -> i32 {
    libc::ENOTUNIQ
}

#[cfg(target_os = "linux")]
unsafe fn f127() -> i32 {
    libc::EREMCHG
}

#[cfg(target_os = "linux")]
unsafe fn f128() -> i32 {
    libc::EREMOTEIO
}

#[cfg(target_os = "linux")]
unsafe fn f129() -> i32 {
    libc::ERESTART
}

#[cfg(target_os = "linux")]
unsafe fn f130() -> i32 {
    libc::ERFKILL
}

#[cfg(target_os = "linux")]
unsafe fn f131() -> i32 {
    libc::ESRMNT
}

#[cfg(target_os = "linux")]
unsafe fn f132() -> i32 {
    libc::ESTRPIPE
}

#[cfg(target_os = "linux")]
unsafe fn f133() -> i32 {
    libc::EUCLEAN
}

#[cfg(target_os = "linux")]
unsafe fn f134() -> i32 {
    libc::EUNATCH
}

#[cfg(target_os = "linux")]
unsafe fn f135() -> i32 {
    libc::EXFULL
}

#[cfg(target_os = "macos")]
unsafe fn f136() -> i32 {
    libc::EAUTH
}

#[cfg(target_os = "macos")]
unsafe fn f137() -> i32 {
    libc::EBADARCH
}

#[cfg(target_os = "macos")]
unsafe fn f138() -> i32 {
    libc::EBADEXEC
}

#[cfg(target_os = "macos")]
unsafe fn f139() -> i32 {
    libc::EBADMACHO
}

#[cfg(target_os = "macos")]
unsafe fn f140() -> i32 {
    libc::EBADRPC
}

#[cfg(target_os = "macos")]
unsafe fn f141() -> i32 {
    libc::EDEVERR
}

#[cfg(target_os = "macos")]
unsafe fn f142() -> i32 {
    libc::EFTYPE
}

#[cfg(target_os = "macos")]
unsafe fn f143() -> i32 {
    libc::ENEEDAUTH
}

#[cfg(target_os = "macos")]
unsafe fn f144() -> i32 {
    libc::ENOATTR
}

#[cfg(target_os = "macos")]
unsafe fn f145() -> i32 {
    libc::ENOPOLICY
}

#[cfg(target_os = "macos")]
unsafe fn f146() -> i32 {
    libc::EPROCLIM
}

#[cfg(target_os = "macos")]
unsafe fn f147() -> i32 {
    libc::EPROCUNAVAIL
}

#[cfg(target_os = "macos")]
unsafe fn f148() -> i32 {
    libc::EPROGMISMATCH
}

#[cfg(target_os = "macos")]
unsafe fn f149() -> i32 {
    libc::EPROGUNAVAIL
}

#[cfg(target_os = "macos")]
unsafe fn f150() -> i32 {
    libc::EPWROFF
}

#[cfg(target_os = "macos")]
unsafe fn f151() -> i32 {
    libc::EQFULL
}

#[cfg(target_os = "macos")]
unsafe fn f152() -> i32 {
    libc::ERPCMISMATCH
}

#[cfg(target_os = "macos")]
unsafe fn f153() -> i32 {
    libc::ESHLIBVERS
}
