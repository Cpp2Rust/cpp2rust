// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::{AnyPtr, AsPointer, CFile, Ptr, Value};
use std::cell::{RefCell, UnsafeCell};
use std::io::{self, Read, Write};
use std::rc::Rc;

enum StreamHandle {
    File(std::fs::File),
    Standard(i32),
}

pub struct CppStream(StreamHandle);

impl From<std::fs::File> for CppStream {
    fn from(file: std::fs::File) -> Self {
        Self(StreamHandle::File(file))
    }
}

impl CppStream {
    fn standard(fd: i32) -> Self {
        Self(StreamHandle::Standard(fd))
    }

    pub fn try_clone(&self) -> io::Result<Self> {
        match &self.0 {
            StreamHandle::File(file) => file.try_clone().map(Self::from),
            StreamHandle::Standard(fd) => Ok(Self::standard(*fd)),
        }
    }
}

impl Read for &CppStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match &self.0 {
            StreamHandle::File(file) => (&*file).read(buf),
            StreamHandle::Standard(fd) => {
                // A standard descriptor may have been closed. Do not create an
                // owning File or a BorrowedFd, which requires an open descriptor.
                let n = unsafe { libc::read(*fd, buf.as_mut_ptr().cast(), buf.len()) };
                if n < 0 {
                    Err(io::Error::last_os_error())
                } else {
                    Ok(n as usize)
                }
            }
        }
    }
}

impl Read for CppStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (&*self).read(buf)
    }
}

impl Write for CppStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match &mut self.0 {
            StreamHandle::File(file) => file.write(buf),
            StreamHandle::Standard(fd) => {
                // The slice is valid for the duration of this unbuffered write.
                let n = unsafe { libc::write(*fd, buf.as_ptr().cast(), buf.len()) };
                if n < 0 {
                    Err(io::Error::last_os_error())
                } else {
                    Ok(n as usize)
                }
            }
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match &mut self.0 {
            StreamHandle::File(file) => file.flush(),
            StreamHandle::Standard(_) => Ok(()),
        }
    }
}

impl crate::ByteRepr for CppStream {}

thread_local! {
    static SAFE_STDIN: Value<CppStream> = Rc::new(RefCell::new(CppStream::standard(0)));
    static SAFE_STDOUT: Value<CppStream> = Rc::new(RefCell::new(CppStream::standard(1)));
    static SAFE_STDERR: Value<CppStream> = Rc::new(RefCell::new(CppStream::standard(2)));
    static UNSAFE_STDIN: UnsafeCell<CppStream> = UnsafeCell::new(CppStream::standard(0));
    static UNSAFE_STDOUT: UnsafeCell<CppStream> = UnsafeCell::new(CppStream::standard(1));
    static UNSAFE_STDERR: UnsafeCell<CppStream> = UnsafeCell::new(CppStream::standard(2));
}

thread_local! {
    static C_STDIN: Value<CFile> = Rc::new(RefCell::new(CFile::new(0)));
    static C_STDOUT: Value<CFile> = Rc::new(RefCell::new(CFile::new(1)));
    static C_STDERR: Value<CFile> = Rc::new(RefCell::new(CFile::new(2)));
}

pub fn c_stdin() -> Ptr<CFile> {
    C_STDIN.with(AsPointer::as_pointer)
}

pub fn c_stdout() -> Ptr<CFile> {
    C_STDOUT.with(AsPointer::as_pointer)
}

pub fn c_stderr() -> Ptr<CFile> {
    C_STDERR.with(AsPointer::as_pointer)
}

pub fn cin() -> Ptr<CppStream> {
    SAFE_STDIN.with(AsPointer::as_pointer)
}

pub fn cout() -> Ptr<CppStream> {
    SAFE_STDOUT.with(AsPointer::as_pointer)
}

pub fn cerr() -> Ptr<CppStream> {
    SAFE_STDERR.with(AsPointer::as_pointer)
}

/// # Safety
///
/// The caller must ensure that the returned pointer is not used after the
/// thread finishes.
pub unsafe fn cin_unsafe() -> *mut CppStream {
    UNSAFE_STDIN.with(UnsafeCell::get)
}

/// # Safety
///
/// The caller must ensure that the returned pointer is not used after the
/// thread finishes.
pub unsafe fn cout_unsafe() -> *mut CppStream {
    UNSAFE_STDOUT.with(UnsafeCell::get)
}

/// # Safety
///
/// The caller must ensure that the returned pointer is not used after the
/// thread finishes.
pub unsafe fn cerr_unsafe() -> *mut CppStream {
    UNSAFE_STDERR.with(UnsafeCell::get)
}

pub fn fread_refcount(a0: AnyPtr, a1: usize, a2: usize, a3: Ptr<CFile>) -> usize {
    let total = a1.saturating_mul(a2);
    if total == 0 {
        return 0;
    }
    let dst = a0.reinterpret_cast::<u8>();
    let read_bytes = dst.with_slice_mut(total, |buf| a3.with_mut(|f| f.read(buf)));
    read_bytes / a1
}

pub fn fwrite_refcount(a0: AnyPtr, a1: usize, a2: usize, a3: Ptr<CFile>) -> usize {
    let total = a1.saturating_mul(a2);
    if total == 0 {
        return 0;
    }
    let src = a0.reinterpret_cast::<u8>();
    let written = src.with_slice(total, |bytes| a3.with_mut(|f| f.write(bytes)));
    written / a1
}

unsafe extern "C" {
    #[cfg(target_os = "linux")]
    #[link_name = "stdin"]
    static mut LIBC_STDIN: *mut libc::FILE;
    #[cfg(target_os = "linux")]
    #[link_name = "stdout"]
    static mut LIBC_STDOUT: *mut libc::FILE;
    #[cfg(target_os = "linux")]
    #[link_name = "stderr"]
    static mut LIBC_STDERR: *mut libc::FILE;

    #[cfg(target_os = "macos")]
    #[link_name = "__stdinp"]
    static mut LIBC_STDIN: *mut libc::FILE;
    #[cfg(target_os = "macos")]
    #[link_name = "__stdoutp"]
    static mut LIBC_STDOUT: *mut libc::FILE;
    #[cfg(target_os = "macos")]
    #[link_name = "__stderrp"]
    static mut LIBC_STDERR: *mut libc::FILE;
}

/// # Safety
///
/// Returns the libc `stdin` handle. The pointer is valid for the process
/// lifetime.
pub unsafe fn stdin_unsafe() -> *mut libc::FILE {
    unsafe { LIBC_STDIN }
}

/// # Safety
///
/// Returns the libc `stdout` handle.
pub unsafe fn stdout_unsafe() -> *mut libc::FILE {
    unsafe { LIBC_STDOUT }
}

/// # Safety
///
/// Returns the libc `stderr` handle.
pub unsafe fn stderr_unsafe() -> *mut libc::FILE {
    unsafe { LIBC_STDERR }
}

/// # Safety
///
/// Same contract as C's `fwrite`.
pub unsafe fn fwrite_unsafe(
    a0: *const ::std::ffi::c_void,
    a1: usize,
    a2: usize,
    a3: *mut libc::FILE,
) -> usize {
    unsafe { libc::fwrite(a0, a1, a2, a3) }
}

/// # Safety
///
/// Same contract as C's `fread`.
pub unsafe fn fread_unsafe(
    a0: *mut ::std::ffi::c_void,
    a1: usize,
    a2: usize,
    a3: *mut libc::FILE,
) -> usize {
    unsafe { libc::fread(a0, a1, a2, a3) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_stream_io() {
        let (read, write) = nix::unistd::pipe().unwrap();
        let mut input = CppStream::from(std::fs::File::from(read));
        let output = CppStream::from(std::fs::File::from(write));
        let mut copy = output.try_clone().unwrap();
        drop(output);
        copy.write_all(&[0, 127, 255]).unwrap();
        copy.flush().unwrap();
        drop(copy);

        let mut first = [9];
        input.read_exact(&mut first).unwrap();
        assert_eq!(first, [0]);
        let mut rest = Vec::new();
        (&input).read_to_end(&mut rest).unwrap();
        assert_eq!(rest, [127, 255]);
    }
}
