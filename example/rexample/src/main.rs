use std::ffi::c_void;
use std::mem;

#[link(name = "Cilantro", kind = "static")] // this (static) doesnt matter (lib is dylib)
extern "C" {
    pub fn cpp_hello();
    pub fn cpp_use(draw: Option<unsafe extern "C" fn()>);
}

fn main() {
    println!("hello");

    println!("<rust_code>"); // could be wrapper
    unsafe{
        cpp_hello(); // interesting, when this is debug, gives asm. must be rust doing stuff, load special way
        cpp_use( Some({
            unsafe extern "C"
            fn cb ()
            {
                println!("hello from rust")
            }
            cb
        }))
    }
    println!("</rust_code>");
}