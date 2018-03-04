extern crate libc;
use libc::{c_int};

#[repr(C)]
struct RustObject {
    number: c_int
}

#[link(name = "hello")]
extern {
    fn c_hello() -> c_int;
	fn c_print_object(object: *mut RustObject) -> c_int;
}

fn main() {
	let mut rust_object = Box::new(RustObject { number: 42 });

    println!("Hello world from Rust!");

	unsafe {
		c_hello();
		c_print_object(&mut *rust_object);
	}
}
