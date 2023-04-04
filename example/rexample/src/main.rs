#[link(name = "Cilantro", kind = "static")]
extern "C" {
    pub fn cpp_hello();
}

fn main() {
    println!("hello");
    unsafe{
        cpp_hello(); // interesting, when this is debug, gives asm. must be rust doing stuff, load special way
    }
}