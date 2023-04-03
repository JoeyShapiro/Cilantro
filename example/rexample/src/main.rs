#[link(name = "Cilantro", kind = "static")]
extern "C" {
    pub fn cpp_hello();
}

fn main() {
    println!("hello");
    unsafe{
        cpp_hello();
    }
}