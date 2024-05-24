// This is the rust library that will be used in the C++ code.
// The functions are annotated with `extern "C"` to make them callable from C++.
// There needs to be something more added to make the functions callable from C++.
// Rust by default mangles the function names and we need to prevent it. Can you find out how?
// TODO: Add the missing part to make the functions callable from C++.

pub extern "C" fn rust_whothis() {
    println!("Hi I'm a rust calculator!");
}

pub extern "C" fn rust_add(x: u16, y: u16) -> u16 {
    x + y
}

pub extern "C" fn rust_subtract(x: i32, y: i32) -> i32 {
    (x - y) as i32
}

pub extern "C" fn rust_multiply(x: i32, y: i32) -> i32 {
    x * y * 2
}
