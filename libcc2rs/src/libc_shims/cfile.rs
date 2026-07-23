// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::FdRegistry;

pub struct CFile {
    pub fd: i32,
    pub eof: bool,
    pub err: bool,
}

impl CFile {
    pub fn new(fd: i32) -> Self {
        CFile {
            fd,
            eof: false,
            err: false,
        }
    }

    pub fn open(path: &str, mode: &str) -> Option<CFile> {
        let flags = match mode {
            "rb" => nix::fcntl::OFlag::O_RDONLY,
            "wb" => nix::fcntl::OFlag::O_WRONLY
                .union(nix::fcntl::OFlag::O_CREAT)
                .union(nix::fcntl::OFlag::O_TRUNC),
            m => panic!("fopen: unsupported mode {:?}", m),
        };
        match nix::fcntl::open(path, flags, nix::sys::stat::Mode::from_bits_truncate(0o666)) {
            Ok(ofd) => Some(CFile::new(FdRegistry::register(ofd))),
            Err(e) => {
                crate::cpp2rust_errno().write(e as i32);
                None
            }
        }
    }

    pub fn read(&mut self, buf: &mut [u8]) -> usize {
        let mut n = 0;
        while n < buf.len() {
            match FdRegistry::with_fd(self.fd, |b| nix::unistd::read(b, &mut buf[n..])) {
                Ok(0) => {
                    self.eof = true;
                    break;
                }
                Ok(k) => n += k,
                Err(nix::errno::Errno::EINTR) => {}
                Err(e) => {
                    self.err = true;
                    crate::cpp2rust_errno().write(e as i32);
                    break;
                }
            }
        }
        n
    }

    pub fn write(&mut self, buf: &[u8]) -> usize {
        let mut n = 0;
        while n < buf.len() {
            match FdRegistry::with_fd(self.fd, |b| nix::unistd::write(b, &buf[n..])) {
                Ok(0) => {
                    self.err = true;
                    break;
                }
                Ok(k) => n += k,
                Err(nix::errno::Errno::EINTR) => {}
                Err(e) => {
                    self.err = true;
                    crate::cpp2rust_errno().write(e as i32);
                    break;
                }
            }
        }
        n
    }

    pub fn seek(&mut self, offset: i64, whence: i32) -> i64 {
        let w = match whence {
            0 => nix::unistd::Whence::SeekSet,
            1 => nix::unistd::Whence::SeekCur,
            2 => nix::unistd::Whence::SeekEnd,
            other => panic!("fseek: unsupported whence {}", other),
        };
        match FdRegistry::with_fd(self.fd, |b| nix::unistd::lseek(b, offset, w)) {
            Ok(off) => {
                self.eof = false;
                off
            }
            Err(e) => {
                crate::cpp2rust_errno().write(e as i32);
                -1
            }
        }
    }

    pub fn tell(&self) -> i64 {
        match FdRegistry::with_fd(self.fd, |b| {
            nix::unistd::lseek(b, 0, nix::unistd::Whence::SeekCur)
        }) {
            Ok(off) => off,
            Err(e) => {
                crate::cpp2rust_errno().write(e as i32);
                -1
            }
        }
    }

    pub fn getc(&mut self) -> i32 {
        let mut b = [0u8; 1];
        match self.read(&mut b) {
            1 => b[0] as i32,
            _ => -1,
        }
    }

    pub fn close(&self) -> i32 {
        FdRegistry::close(self.fd)
    }
}

impl crate::ByteRepr for CFile {}
