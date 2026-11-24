//! Configure linker scripts for the RP2350 firmware application.

use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(
        std::env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is set")
    );

    let workspace_root = manifest_dir.join("../..");
    let out_dir = PathBuf::from(
        std::env::var_os("OUT_DIR").expect("OUT_DIR is set")
    );

    println!("cargo:rustc-link-search={}", out_dir.display());

    let memory_x = workspace_root.join("memory.x");
    fs::copy(&memory_x, out_dir.join("memory.x"))
        .expect("Failed to copy memory.x");
    println!("cargo:rerun-if-changed={}", memory_x.display());

    let riscv_x = workspace_root.join("rp235x_riscv.x");
    fs::copy(&riscv_x, out_dir.join("rp235x_riscv.x"))
        .expect("Failed to copy rp235x_riscv.x");
    println!("cargo:rerun-if-changed={}", riscv_x.display());

    println!("cargo:rerun-if-changed=build.rs");
}
