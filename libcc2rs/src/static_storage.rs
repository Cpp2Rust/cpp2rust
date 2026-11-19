// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::sync::Mutex;

static STATIC_DESTRUCTORS: Mutex<Vec<fn()>> = Mutex::new(Vec::new());

pub fn register_static_destructor(destructor: fn()) {
    STATIC_DESTRUCTORS.lock().unwrap().push(destructor);
}

pub fn run_static_destructors() {
    loop {
        let destructor = STATIC_DESTRUCTORS.lock().unwrap().pop();
        match destructor {
            Some(destructor) => destructor(),
            None => return,
        }
    }
}

pub fn cpp2rust_exit(code: i32) -> ! {
    run_static_destructors();
    std::process::exit(code)
}
