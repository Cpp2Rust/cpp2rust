// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1() -> *mut i32 {
    libcc2rs::cpp2rust_errno_unsafe()
}

fn f2() -> i32 {
    libc::E2BIG
}

fn f3() -> i32 {
    libc::EACCES
}

fn f4() -> i32 {
    libc::EADDRINUSE
}

fn f5() -> i32 {
    libc::EADDRNOTAVAIL
}

fn f6() -> i32 {
    libc::EAFNOSUPPORT
}

fn f7() -> i32 {
    libc::EAGAIN
}

fn f8() -> i32 {
    libc::EALREADY
}

fn f9() -> i32 {
    libc::EBADF
}

fn f10() -> i32 {
    libc::EBADMSG
}

fn f11() -> i32 {
    libc::EBUSY
}

fn f12() -> i32 {
    libc::ECANCELED
}

fn f13() -> i32 {
    libc::ECHILD
}

fn f14() -> i32 {
    libc::ECONNABORTED
}

fn f15() -> i32 {
    libc::ECONNREFUSED
}

fn f16() -> i32 {
    libc::ECONNRESET
}

fn f17() -> i32 {
    libc::EDEADLK
}

fn f18() -> i32 {
    libc::EDESTADDRREQ
}

fn f19() -> i32 {
    libc::EDOM
}

fn f20() -> i32 {
    libc::EDQUOT
}

fn f21() -> i32 {
    libc::EEXIST
}

fn f22() -> i32 {
    libc::EFAULT
}

fn f23() -> i32 {
    libc::EFBIG
}

fn f24() -> i32 {
    libc::EHOSTDOWN
}

fn f25() -> i32 {
    libc::EHOSTUNREACH
}

fn f26() -> i32 {
    libc::EIDRM
}

fn f27() -> i32 {
    libc::EILSEQ
}

fn f28() -> i32 {
    libc::EINPROGRESS
}

fn f29() -> i32 {
    libc::EINTR
}

fn f30() -> i32 {
    libc::EINVAL
}

fn f31() -> i32 {
    libc::EIO
}

fn f32() -> i32 {
    libc::EISCONN
}

fn f33() -> i32 {
    libc::EISDIR
}

fn f34() -> i32 {
    libc::ELOOP
}

fn f35() -> i32 {
    libc::EMFILE
}

fn f36() -> i32 {
    libc::EMLINK
}

fn f37() -> i32 {
    libc::EMSGSIZE
}

fn f38() -> i32 {
    libc::EMULTIHOP
}

fn f39() -> i32 {
    libc::ENAMETOOLONG
}

fn f40() -> i32 {
    libc::ENETDOWN
}

fn f41() -> i32 {
    libc::ENETRESET
}

fn f42() -> i32 {
    libc::ENETUNREACH
}

fn f43() -> i32 {
    libc::ENFILE
}

fn f44() -> i32 {
    libc::ENOBUFS
}

fn f45() -> i32 {
    libc::ENODATA
}

fn f46() -> i32 {
    libc::ENODEV
}

fn f47() -> i32 {
    libc::ENOENT
}

fn f48() -> i32 {
    libc::ENOEXEC
}

fn f49() -> i32 {
    libc::ENOLCK
}

fn f50() -> i32 {
    libc::ENOLINK
}

fn f51() -> i32 {
    libc::ENOMEM
}

fn f52() -> i32 {
    libc::ENOMSG
}

fn f53() -> i32 {
    libc::ENOPROTOOPT
}

fn f54() -> i32 {
    libc::ENOSPC
}

fn f55() -> i32 {
    libc::ENOSR
}

fn f56() -> i32 {
    libc::ENOSTR
}

fn f57() -> i32 {
    libc::ENOSYS
}

fn f58() -> i32 {
    libc::ENOTCONN
}

fn f59() -> i32 {
    libc::ENOTDIR
}

fn f60() -> i32 {
    libc::ENOTEMPTY
}

fn f61() -> i32 {
    libc::ENOTRECOVERABLE
}

fn f62() -> i32 {
    libc::ENOTSOCK
}

fn f63() -> i32 {
    libc::ENOTSUP
}

fn f64() -> i32 {
    libc::ENOTBLK
}

fn f65() -> i32 {
    libc::ENOTTY
}

fn f66() -> i32 {
    libc::ENXIO
}

fn f67() -> i32 {
    libc::EOPNOTSUPP
}

fn f68() -> i32 {
    libc::EOVERFLOW
}

fn f69() -> i32 {
    libc::EOWNERDEAD
}

fn f70() -> i32 {
    libc::EPERM
}

fn f71() -> i32 {
    libc::EPFNOSUPPORT
}

fn f72() -> i32 {
    libc::EPIPE
}

fn f73() -> i32 {
    libc::EPROTO
}

fn f74() -> i32 {
    libc::EPROTONOSUPPORT
}

fn f75() -> i32 {
    libc::EPROTOTYPE
}

fn f76() -> i32 {
    libc::ERANGE
}

fn f77() -> i32 {
    libc::EREMOTE
}

fn f78() -> i32 {
    libc::EROFS
}

fn f79() -> i32 {
    libc::ESHUTDOWN
}

fn f80() -> i32 {
    libc::ESOCKTNOSUPPORT
}

fn f81() -> i32 {
    libc::ESPIPE
}

fn f82() -> i32 {
    libc::ESRCH
}

fn f83() -> i32 {
    libc::ESTALE
}

fn f84() -> i32 {
    libc::ETIME
}

fn f85() -> i32 {
    libc::ETIMEDOUT
}

fn f86() -> i32 {
    libc::ETOOMANYREFS
}

fn f87() -> i32 {
    libc::ETXTBSY
}

fn f88() -> i32 {
    libc::EUSERS
}

fn f89() -> i32 {
    libc::EWOULDBLOCK
}

fn f90() -> i32 {
    libc::EXDEV
}

#[cfg(target_os = "linux")]
fn f91() -> i32 {
    libc::EADV
}

#[cfg(target_os = "linux")]
fn f92() -> i32 {
    libc::EBADE
}

#[cfg(target_os = "linux")]
fn f93() -> i32 {
    libc::EBADFD
}

#[cfg(target_os = "linux")]
fn f94() -> i32 {
    libc::EBADR
}

#[cfg(target_os = "linux")]
fn f95() -> i32 {
    libc::EBADRQC
}

#[cfg(target_os = "linux")]
fn f96() -> i32 {
    libc::EBADSLT
}

#[cfg(target_os = "linux")]
fn f97() -> i32 {
    libc::EBFONT
}

#[cfg(target_os = "linux")]
fn f98() -> i32 {
    libc::ECHRNG
}

#[cfg(target_os = "linux")]
fn f99() -> i32 {
    libc::ECOMM
}

#[cfg(target_os = "linux")]
fn f100() -> i32 {
    libc::EDEADLOCK
}

#[cfg(target_os = "linux")]
fn f101() -> i32 {
    libc::EDOTDOT
}

#[cfg(target_os = "linux")]
fn f102() -> i32 {
    libc::EHWPOISON
}

#[cfg(target_os = "linux")]
fn f103() -> i32 {
    libc::EISNAM
}

#[cfg(target_os = "linux")]
fn f104() -> i32 {
    libc::EKEYEXPIRED
}

#[cfg(target_os = "linux")]
fn f105() -> i32 {
    libc::EKEYREJECTED
}

#[cfg(target_os = "linux")]
fn f106() -> i32 {
    libc::EKEYREVOKED
}

#[cfg(target_os = "linux")]
fn f107() -> i32 {
    libc::EL2HLT
}

#[cfg(target_os = "linux")]
fn f108() -> i32 {
    libc::EL2NSYNC
}

#[cfg(target_os = "linux")]
fn f109() -> i32 {
    libc::EL3HLT
}

#[cfg(target_os = "linux")]
fn f110() -> i32 {
    libc::EL3RST
}

#[cfg(target_os = "linux")]
fn f111() -> i32 {
    libc::ELIBACC
}

#[cfg(target_os = "linux")]
fn f112() -> i32 {
    libc::ELIBBAD
}

#[cfg(target_os = "linux")]
fn f113() -> i32 {
    libc::ELIBEXEC
}

#[cfg(target_os = "linux")]
fn f114() -> i32 {
    libc::ELIBMAX
}

#[cfg(target_os = "linux")]
fn f115() -> i32 {
    libc::ELIBSCN
}

#[cfg(target_os = "linux")]
fn f116() -> i32 {
    libc::ELNRNG
}

#[cfg(target_os = "linux")]
fn f117() -> i32 {
    libc::EMEDIUMTYPE
}

#[cfg(target_os = "linux")]
fn f118() -> i32 {
    libc::ENAVAIL
}

#[cfg(target_os = "linux")]
fn f119() -> i32 {
    libc::ENOANO
}

#[cfg(target_os = "linux")]
fn f120() -> i32 {
    libc::ENOCSI
}

#[cfg(target_os = "linux")]
fn f121() -> i32 {
    libc::ENOKEY
}

#[cfg(target_os = "linux")]
fn f122() -> i32 {
    libc::ENOMEDIUM
}

#[cfg(target_os = "linux")]
fn f123() -> i32 {
    libc::ENONET
}

#[cfg(target_os = "linux")]
fn f124() -> i32 {
    libc::ENOPKG
}

#[cfg(target_os = "linux")]
fn f125() -> i32 {
    libc::ENOTNAM
}

#[cfg(target_os = "linux")]
fn f126() -> i32 {
    libc::ENOTUNIQ
}

#[cfg(target_os = "linux")]
fn f127() -> i32 {
    libc::EREMCHG
}

#[cfg(target_os = "linux")]
fn f128() -> i32 {
    libc::EREMOTEIO
}

#[cfg(target_os = "linux")]
fn f129() -> i32 {
    libc::ERESTART
}

#[cfg(target_os = "linux")]
fn f130() -> i32 {
    libc::ERFKILL
}

#[cfg(target_os = "linux")]
fn f131() -> i32 {
    libc::ESRMNT
}

#[cfg(target_os = "linux")]
fn f132() -> i32 {
    libc::ESTRPIPE
}

#[cfg(target_os = "linux")]
fn f133() -> i32 {
    libc::EUCLEAN
}

#[cfg(target_os = "linux")]
fn f134() -> i32 {
    libc::EUNATCH
}

#[cfg(target_os = "linux")]
fn f135() -> i32 {
    libc::EXFULL
}

#[cfg(target_os = "macos")]
fn f136() -> i32 {
    libc::EAUTH
}

#[cfg(target_os = "macos")]
fn f137() -> i32 {
    libc::EBADARCH
}

#[cfg(target_os = "macos")]
fn f138() -> i32 {
    libc::EBADEXEC
}

#[cfg(target_os = "macos")]
fn f139() -> i32 {
    libc::EBADMACHO
}

#[cfg(target_os = "macos")]
fn f140() -> i32 {
    libc::EBADRPC
}

#[cfg(target_os = "macos")]
fn f141() -> i32 {
    libc::EDEVERR
}

#[cfg(target_os = "macos")]
fn f142() -> i32 {
    libc::EFTYPE
}

#[cfg(target_os = "macos")]
fn f143() -> i32 {
    libc::ENEEDAUTH
}

#[cfg(target_os = "macos")]
fn f144() -> i32 {
    libc::ENOATTR
}

#[cfg(target_os = "macos")]
fn f145() -> i32 {
    libc::ENOPOLICY
}

#[cfg(target_os = "macos")]
fn f146() -> i32 {
    libc::EPROCLIM
}

#[cfg(target_os = "macos")]
fn f147() -> i32 {
    libc::EPROCUNAVAIL
}

#[cfg(target_os = "macos")]
fn f148() -> i32 {
    libc::EPROGMISMATCH
}

#[cfg(target_os = "macos")]
fn f149() -> i32 {
    libc::EPROGUNAVAIL
}

#[cfg(target_os = "macos")]
fn f150() -> i32 {
    libc::EPWROFF
}

#[cfg(target_os = "macos")]
fn f151() -> i32 {
    libc::EQFULL
}

#[cfg(target_os = "macos")]
fn f152() -> i32 {
    libc::ERPCMISMATCH
}

#[cfg(target_os = "macos")]
fn f153() -> i32 {
    libc::ESHLIBVERS
}
