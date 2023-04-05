use std::ffi::c_void;
use std::mem;
use std::time::Instant;

#[link(name = "Cilantro", kind = "static")] // this (static) doesnt matter (lib is dylib)
extern "C" {
    pub fn cpp_hello();
    pub fn cpp_use(draw: Option<unsafe extern "C" fn()>);
}

fn main() {
    println!("hello");

    let start = Instant::now();
    println!("<rust_code>"); // could be wrapper
    unsafe{
        cpp_hello(); // interesting, when this is debug, gives asm. because was seg-faulting
        cpp_use( Some({
            unsafe extern "C"
            fn cb () { println!("hello from rust") }
            cb
        }))
    }
    println!("</rust_code>");
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    let start2 = Instant::now();
    println!("hello");
    println!("hello");
    println!("hello");
    println!("hello");
    println!("hello");
    println!("hello");
    println!("hello");
    println!("hello");
    let duration2 = start2.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration2);
}