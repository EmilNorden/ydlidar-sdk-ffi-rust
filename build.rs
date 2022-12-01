extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=/usr/local/lib");

    //println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=ydlidar_sdk");

    println!("cargo:rerun-if-changed=wrapper.hpp");

    // Link to libstdc++ on GNU
    let target = env::var("TARGET").unwrap();
    if target.contains("gnu") {
        println!("cargo:rustc-link-lib=stdc++");
    } else if target.contains("apple") {
        println!("cargo:rustc-link-lib=c++");
    }

    let bindings = bindgen::Builder::default()
        .header("wrapper.hpp")
        //.clang_arg("-x")
        //.clang_arg("c++")
        .clang_arg("-std=c++14")
        .allowlist_type("CYdLidar")
        .allowlist_type("LidarProp.*") // Allow all LidarProp constants. Hopefully this works
        .opaque_type("std::.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
