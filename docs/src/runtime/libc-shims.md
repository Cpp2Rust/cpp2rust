# libc Shims

In the refcount model every struct member is wrapped in a `Value`, so a libc
struct cannot be used directly. Even without that wrapping the layouts would not
meet: libc structs hold raw pointers, which are incompatible with the refcounted
pointers the model uses. The `libc_shims` modules therefore define Rust
counterparts for the libc types translated programs use. A shim struct mirrors
its C struct member by member, with each field a `Value<T>`, and converts to or
from the underlying libc or nix type at the call boundary.

`Stat` is a typical shim:

```rust
#[derive(Default)]
pub struct Stat {
    pub st_dev: Value<u64>,
    pub st_ino: Value<u64>,
    // ...
    pub st_size: Value<i64>,
}

impl Stat {
    pub fn from_libc(s: &::libc::stat) -> Self { /* ... */ }
}
```

A `stat` call in the source program becomes a `nix::sys::stat::stat` call. On
success nix returns a raw `libc::stat`, so the result goes through
`Stat::from_libc` before it is written into the translated struct.

## The modules

| Module    | C types                                                                                          |
| --------- | ------------------------------------------------------------------------------------------------ |
| `cfile`   | `FILE` (`CFile`)                                                                                 |
| `dirent`  | `struct dirent`, `DIR` (`Dirent`, `CDir`)                                                        |
| `fdset`   | `fd_set` (`CFdSet`)                                                                              |
| `ifaddrs` | `struct ifaddrs` (`Ifaddrs`)                                                                     |
| `ip`      | `struct in_addr`, `struct in6_addr` (`InAddr`, `In6Addr`)                                        |
| `netdb`   | `struct addrinfo` (`Addrinfo`)                                                                   |
| `poll`    | `struct pollfd` (`Pollfd`)                                                                       |
| `pwd`     | `struct passwd` (`Passwd`)                                                                       |
| `socket`  | the `sockaddr` family (`Sockaddr`, `SockaddrIn`, `SockaddrIn6`, `SockaddrUn`, `SockaddrStorage`) |
| `stat`    | `struct stat` (`Stat`)                                                                           |
| `termios` | `struct termios`, `struct winsize` (`Termios`, `Winsize`)                                        |
| `time`    | `struct tm`, `struct timeval`, `struct timespec` (`Tm`, `Timeval`, `Timespec`)                   |

Most shims are plain data plus conversions like `Stat`. `CFile` carries the
stdio stream logic (see [I/O and Formatting](./io.md)), and the `time` shims
convert through the `jiff` crate. `CFdSet` and the `sockaddr` family need more
than a field-by-field mirror and are described in their own sections below.

Each shim module also gives the raw `libc` struct it mirrors an empty `ByteRepr`
impl (`impl ByteRepr for ::libc::stat {}`), whose methods panic. These exist so
that the generated `ByteRepr` implementation of a translated struct with a libc
struct member still compiles; reinterpreting such a struct is not supported at
present.

## CFdSet

nix has its own `FdSet`, but it is stricter than the C one: it ties the set to
the lifetimes of the descriptors it holds. A C `fd_set` is just a set of
integers that accepts anything; whether the descriptors are valid is only
checked by the `select` call that eventually receives the set. `CFdSet` keeps
the C behavior by storing plain integers, and the `select` rule builds the nix
`FdSet` from it at call time.

## The sockaddr family

C socket code reinterprets one address struct as another: the program fills in a
`struct sockaddr_in`, passes it to `bind` as a `struct sockaddr *`, and casts
back to the concrete type on the way out of `accept`. The address shims keep
this pattern working by implementing [`ByteRepr`](./reinterpret.md) with the
exact byte layout of their C structs: the family in the first two bytes, the
remaining members at their C offsets. A cast in the source program becomes a
[`reinterpret_cast`](./reinterpret.md) on the refcounted pointer, which reads
the struct through that byte layout as the target type, so any member of the
family can be viewed as any other, exactly as in C.

The call boundary works the same way. `Sockaddr::decode` reads the family from
the first two bytes and reinterprets the pointer as the concrete type before
handing nix a typed address:

```rust
pub fn decode(
    addr: &Ptr<Sockaddr>,
    _len: u32,
) -> Option<Box<dyn SockaddrLike>> {
    let family = addr.reinterpret_cast::<u16>().read();
    if family == libc::AF_INET as u16 {
        let m = addr.reinterpret_cast::<SockaddrIn>().read();
        Some(Box::new(nix::sys::socket::SockaddrIn::from(m.to_libc())))
    }
    // ... AF_INET6 and AF_UNIX in the same way ...
}
```

`Sockaddr::encode` goes the other way, writing an address returned by nix into
the caller's buffer through the concrete shim. `Ifaddrs` hands out its addresses
as `Ptr<Sockaddr>` values ready to be reinterpreted.

## Non-uniform fields

Some struct fields are not spelled the same on every platform. `struct stat`
keeps the modification time in a nested `struct timespec`, named `st_mtim` on
Linux and `st_mtimespec` on macOS, while the shim exposes a single `st_mtime`
field. `struct in6_addr` hides its bytes behind the internal `__in6_u` union on
Linux, while the shim exposes `s6_addr`. The shims pick one uniform field, and
the code generator meets them halfway: `replaceNonUniformLibcField` in the
converter rewrites the platform-specific member chain in the source, so
`st.st_mtim.tv_sec` becomes `st.st_mtime` in the translated code.
