# I/O and Formatting

The `io`, `format`, and `fd` modules support the stdio stream functions,
`printf`-style formatting, and descriptor-based I/O.

## C Streams

A C `FILE` is more than a file handle: it carries sticky end-of-file and error
flags that `feof` and `ferror` report long after the read that set them.
`std::fs::File` keeps no such state, so the refcount model translates `FILE *`
as a `Ptr<CFile>`, a [libc shim](./libc-shims.md) that holds the file descriptor
together with these two flags. The standard streams are thread-local `CFile`
values over descriptors 0, 1, and 2, returned by `c_stdin`, `c_stdout`, and
`c_stderr`. `CFile` does no buffering at present: every read or write on it is a
system call on the descriptor.

In the unsafe model streams stay raw: `stdin_unsafe`, `stdout_unsafe`, and
`stderr_unsafe` return the process's `*mut libc::FILE` handles, whose symbol
names differ per platform (`stdin` on Linux, `__stdinp` on macOS).

`fread` and `fwrite` exist in both models as named functions, because translated
programs take their address:

```rust
pub fn fread_refcount(
    a0: AnyPtr,
    a1: usize,
    a2: usize,
    a3: Ptr<CFile>,
) -> usize;
pub unsafe fn fread_unsafe(
    a0: *mut c_void,
    a1: usize,
    a2: usize,
    a3: *mut libc::FILE,
) -> usize;
```

The refcount variant reinterprets the destination as a byte array and reads
through the `CFile`; the unsafe variant forwards to `libc::fread`.

## C++ Streams

In the refcount model `cin`, `cout`, and `cerr` are translated as
`Ptr<std::fs::File>` values over duplicates of the standard descriptors,
returned by the `cin`, `cout`, and `cerr` functions. `Ptr<T>` implements
`write_fmt` and `write_all` whenever `T: Write`, forwarding to the pointee
through `with_mut`, so `cout << x` becomes `write!(cout(), "{}", x)`, while
characters and strings, which are bytes, are written with
`cout().write_all(..)`. In the unsafe model `cin_unsafe`, `cout_unsafe`, and
`cerr_unsafe` return raw pointers to thread-local `std::fs::File` values. C++
streams do not map fully onto `std::fs::File`, so this translation may change in
the future.

## Formatting

C strings and formats are bytes, which need not be UTF-8, so the `printf` family
is not translated into `print!` and `println!`, which write a `str`. In the
refcount model, it is formatted by `format_c`, and `printf` and `fprintf` write
the result to the `CFile`, as `fputs` does; the unsafe model calls libc
directly. A call to `printf` or `fprintf` with a literal format becomes the
`printf!` or `fprintf!` macro, which reads like `print!`:

```rust
printf!(b"%d: %s\n", i, name);
fprintf!(libcc2rs::c_stderr(), b"error %d\n", code);
```

Other calls, and the other functions of the family, go through their rules.

`format_c` evaluates a C format string against a slice of
[variadic arguments](./va-args.md) and returns the formatted bytes:

```rust
pub fn format_c(fmt: &[u8], va: &[VaArg]) -> Vec<u8>;
```

`%s` and `%c` are formatted by `format_c` itself, which copies the bytes and
counts widths and precisions in bytes, as C does. The numeric conversions are
rendered by the `sprintf` crate, one conversion at a time. A malformed format
string or an argument of the wrong kind is a panic. Three things are not
supported yet:

1. `%p` renders through the pointer's [integer cast](./rc.md#integer-casts),
   which currently panics.
2. `%n` is not handled.
3. A `*` width or precision (`%*d`, `%.*s`) is a panic.

## File descriptors

Rust tracks descriptor ownership in the type system: an `OwnedFd` closes the
descriptor when dropped, and a `BorrowedFd` grants temporary access to one. C
has no such distinction: a descriptor is a plain `int`, mixed freely with
integer arithmetic, so the translator cannot tell which `int` values are
descriptors. The refcount model therefore leaves descriptors as integers in the
translated program and keeps the ownership in one place, the thread-local
`FdRegistry`, a table from each integer to the open descriptor it names.

The registry follows the descriptor's life. When a rule opens a file,
`FdRegistry::register` stores the resulting `OwnedFd` and hands the program its
raw number. When a rule performs I/O on that number, `FdRegistry::with_fd` looks
the entry up and lends it out as a `BorrowedFd` for the duration of the call.
When the program calls `close`, `FdRegistry::close` removes the entry, which
closes the descriptor. The registry starts out holding the standard descriptors
0, 1, and 2.

In the `fstat` rule, the descriptor argument goes through `with_fd`:

```rust
fn f2(a0: i32, a1: Ptr<Stat>) -> i32 {
    match FdRegistry::with_fd(a0, |fd: BorrowedFd<'_>| {
        nix::sys::stat::fstat(fd)
    }) {
        // ...
    }
}
```

`with_fds` borrows several descriptors at once for `select`-style calls. The
`select` rule collects every descriptor set in the `fd_set` arguments and
borrows them all for the duration of the call:

```rust
let wanted: Vec<i32> = /* the descriptors set in the fd_set arguments */;
FdRegistry::with_fds(&wanted, |borrowed: &[BorrowedFd<'_>]| {
    let mut read_set = nix::sys::select::FdSet::new();
    for fd in &borrowed[..read_count] {
        read_set.insert(*fd);
    }
    // ... build the write and except sets the same way ...
    nix::sys::select::select(nfds, &mut read_set, /* ... */)
})
```

Using a descriptor that was never opened, or using it after it was closed, is a
bug in the original program. The registry turns such a use into a panic (with a
message prefixed `ub:`) so the bug surfaces instead of going unnoticed.
