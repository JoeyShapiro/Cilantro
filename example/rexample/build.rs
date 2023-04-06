fn main() {
    // tell rustc to link with some libhello.a library
    println!("cargo:rustc-link-lib=Cilantro");
    // and it should search the Cargo.toml directory for that library
    println!("cargo:rustc-link-search=.");
    println!("cargo:rustc-link-lib=c++"); // gcc needs stdc++, but rust needs c++ (mac uses libc++.a)
    // not sure, it seems like static is a little bigger (455K -> 462K)
}