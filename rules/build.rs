// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::{
    env, fs,
    io::Write,
    path::{Path, PathBuf},
};

fn main() {
    let crate_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let modules_rs = crate_root.join("src").join("modules.rs");

    // Collect all tgt_*.rs files
    let mut files = Vec::new();
    visit(&crate_root, &mut |p| {
        if let Some(name) = p.file_name().and_then(|s| s.to_str()) {
            if name.starts_with("tgt_") && name.ends_with(".rs") {
                files.push(p.to_path_buf());
            }
        }
    });
    files.sort();

    // Rebuild when any target file or this build script changes
    for f in &files {
        println!("cargo:rerun-if-changed={}", f.display());
    }
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");

    // Generate modules that include absolute paths
    let mut buf = String::new();
    for f in files {
        let abs = f.canonicalize().unwrap_or(f.clone());

        // Stable, valid Rust module name from path
        let mut module_name = abs
            .strip_prefix(&crate_root)
            .unwrap_or(&abs)
            .to_string_lossy()
            .replace('\\', "/")
            .trim_start_matches("./")
            .replace('/', "_")
            .replace('.', "_")
            .replace('-', "_");

        if let Some(stripped) = module_name.strip_suffix("_rs") {
            module_name = stripped.to_string();
        }
        if !module_name
            .chars()
            .next()
            .map(|c| c.is_ascii_alphabetic() || c == '_')
            .unwrap_or(false)
        {
            module_name = format!("m_{}", module_name);
        }

        let rel = f.strip_prefix(&crate_root).unwrap_or(&f);
        buf.push_str(&format!(
            "#[path = r#\"../{}\"#]\npub mod {};\n",
            rel.display(),
            module_name
        ));
    }

    fs::create_dir_all(modules_rs.parent().unwrap()).unwrap();
    let mut f = fs::File::create(&modules_rs).unwrap();
    f.write_all(buf.as_bytes()).unwrap();
}

fn visit(dir: &Path, f: &mut impl FnMut(&Path)) {
    if let Ok(entries) = fs::read_dir(dir) {
        for e in entries.flatten() {
            let p = e.path();

            // Skip noisy/build dirs
            if p.is_dir() {
                let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
                if matches!(name, "target" | ".git") {
                    continue;
                }
                visit(&p, f);
            } else {
                f(&p);
            }
        }
    }
}
