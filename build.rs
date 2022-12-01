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
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_function("lidarCreate")
        .allowlist_function("lidarDestroy")
        .allowlist_function("setlidaropt")
        .allowlist_function("getlidaropt")
        .allowlist_function("GetSdkVersion")
        .allowlist_function("initialize")
        .allowlist_function("GetLidarVersion")
        .allowlist_function("turnOn")
        .allowlist_function("doProcessSimple")
        .allowlist_function("turnOff")
        .allowlist_function("disconnecting")
        .allowlist_function("DescribeError")
        .allowlist_function("os_init")
        .allowlist_function("os_isOk")
        .allowlist_function("os_shutdown")
        .allowlist_function("lidarPortList")
        .allowlist_type("LidarProp.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

