extern crate cc;
use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    cc::Build::new()
        .file("src/dabort.s")
        .file("src/cpustack.s")
        .compile("libtms570");

    if target.contains("eabihf") {
        println!("cargo:rustc-cfg=vfp");
    }
    
    println!("cargo:rerun-if-changed=bsp.ld");
    println!("cargo:rerun-if-changed=tms570ls31xx.ld");
}
