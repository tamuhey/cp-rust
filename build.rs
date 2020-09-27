extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=acl");
    println!("cargo:rustc-link-search=./build");
    println!("cargo:rustc-link-search=./acl_cpp");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.hpp");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.hpp")
        .clang_arg("-F/Library/Developer/CommandLineTools/usr/include/c++/v1")
        .clang_arg("-I/Library/Developer/CommandLineTools/usr/include/c++/v1")
        .clang_arg("-I/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include")
        .clang_arg("-I.")
        .clang_arg("-std=c++17")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}
