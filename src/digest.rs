use std::{fs, os::unix::fs::MetadataExt, path::Path};

use hash_digest::blake3;
use serde::Serialize;

pub(crate) fn gen_digest(arch: &str) {
    let file = format!("zsh-{arch}.tar.zst");
    eprintln!("File: {file}");
    let hash = blake3::get(&file).expect("Failed to get hash");
    let size = Path::new(&file)
        .metadata()
        .expect("Invalid file")
        .size();
    let digest = Digest::new(arch, file, size, hash.as_str());
    let mut toml = toml::to_string_pretty(&DigestArr { digest: [digest] })
        .expect("Failed to ser to toml");
    toml.push('\n');
    fs::write(format!("{arch}.toml"), toml).expect("Failed to write digest toml");
}

#[derive(Serialize, Debug)]
struct DigestArr<'a> {
    digest: [Digest<'a>; 1],
}

#[derive(Serialize, Debug)]
struct Digest<'a> {
    arch: &'a str,
    file: String,
    size: u64,
    algorithm: &'a str,
    hex: String,
}

impl<'a> Digest<'a> {
    fn new<S: Into<String>, F: Into<String>>(
        arch: &'a str,
        file: F,
        size: u64,
        hex: S,
    ) -> Self {
        Self {
            arch,
            file: file.into(),
            size,
            algorithm: "blake3",
            hex: hex.into(),
        }
    }
}
