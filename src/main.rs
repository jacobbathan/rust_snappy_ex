extern crate libc; // Imports C types [use case size_t]
use libc::size_t;

#[link(name = "snappy")] // Name of lib for linking
extern {
    // write function to import
    fn snappy_max_compressed_length(source_length: size_t) -> size_t;
}


fn main() {
    let x = unsafe { snappy_max_compressed_length(100) };
    println!("max compressed length of a 100 byte buffer: {}", x);
}
