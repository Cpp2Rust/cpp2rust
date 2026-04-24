// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::io::prelude::*;
use std::os::fd::AsFd;

fn types() -> Result<(), Box<dyn std::error::Error>> {
    let t1: Ptr<::std::fs::File> = Ptr::null();
    Ok(())
}

fn f1(a0: Ptr<u8>, a1: Ptr<u8>) -> Ptr<::std::fs::File> {
    match a1.to_rust_string() {
        v if v == "rb" => std::fs::OpenOptions::new()
            .read(true)
            .open(a0.to_rust_string())
            .ok()
            .map_or(Ptr::null(), |f| Ptr::alloc(f)),
        v if v == "wb" => std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(a0.to_rust_string())
            .ok()
            .map_or(Ptr::null(), |f| Ptr::alloc(f)),
        _ => panic!("unsupported mode"),
    }
}

fn f2(a0: Ptr<::std::fs::File>) -> i32 {
    a0.delete();
    0
}

fn f3(a0: Ptr<::std::fs::File>) -> i64 {
    a0.with_mut(|v| match v.stream_position() {
        Ok(pos) => pos as i64,
        Err(_) => -1,
    })
}

fn f4(a0: &mut ::std::fs::File, a1: i64, a2: i32) -> i32 {
    if (match a2 {
        0 => a0.seek(std::io::SeekFrom::Start(a1 as u64)),
        1 => a0.seek(std::io::SeekFrom::Current(a1)),
        2 => a0.seek(std::io::SeekFrom::End(a1)),
        _ => Err(std::io::Error::other("unsupported whence for fseek.")),
    })
    .is_ok()
    {
        0
    } else {
        -1
    }
}

fn f5(a0: AnyPtr, a1: u64, a2: u64, a3: Ptr<::std::fs::File>) -> u64 {
    libcc2rs::fread_refcount(a0, a1, a2, a3)
}

fn f6(a0: AnyPtr, a1: u64, a2: u64, a3: Ptr<::std::fs::File>) -> u64 {
    let total = a1.saturating_mul(a2) as usize;
    let mut src = a0
        .cast::<u8>()
        .expect("fwrite: only supporting u8 pointers")
        .clone();

    let mut f = (*a3.upgrade().deref())
        .try_clone()
        .expect("try_clone failed");
    let mut writer = std::io::BufWriter::with_capacity(64 * 1024, f);

    let mut written_bytes: usize = 0;
    let mut buffer: [u8; 8192] = [0; 8192];

    while written_bytes < total {
        let remaining = total - written_bytes;
        let to_fill = std::cmp::min(buffer.len(), remaining);

        for i in 0..to_fill {
            buffer[i] = src.read();
            src = src.offset(1);
        }

        let mut off = 0;
        while off < to_fill {
            match std::io::Write::write(&mut writer, &buffer[off..to_fill]) {
                Ok(0) => break,
                Ok(n) => off += n,
                Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(e) => panic!("Unhandled error in fwrite: {e}"),
            }
        }

        if off == 0 {
            break;
        }

        written_bytes += off;
    }

    std::io::Write::flush(&mut writer).expect("flush failed");

    (written_bytes / a1 as usize) as u64
}

fn f7(a0: Ptr<::std::fs::File>) -> i32 {
    if !a0.is_null() {
        match a0.with_mut(|v| v.sync_all()) {
            Ok(_) => 0,
            Err(_) => -1,
        }
    } else {
        ::std::io::stdout().flush().unwrap();
        ::std::io::stderr().flush().unwrap();
        0
    }
}

fn f8() -> Ptr<::std::fs::File> {
    libcc2rs::cout()
}

fn f9() -> Ptr<::std::fs::File> {
    libcc2rs::cerr()
}
