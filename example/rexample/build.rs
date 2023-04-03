fn main() {
    // tell rustc to link with some libhello.a library
    println!("cargo:rustc-link-lib=Cilantro");
    // and it should search the Cargo.toml directory for that library
    println!("cargo:rustc-link-search=.");
}