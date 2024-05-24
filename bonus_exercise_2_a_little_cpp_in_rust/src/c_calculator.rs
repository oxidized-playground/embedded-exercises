use super::calculator::Calculator;

pub struct CCalculator;

// Also in Rust we need to use the `extern "C"` to make the functions callable from C++.
// TODO: Define the functions in an extern "C" block to make them callable from C++.

impl Calculator for CCalculator {
    fn whothis(&self) {
        unsafe {c_whothis()};
    }
    fn add(&self, x: i16, y: i16) -> i16 {
        unsafe {c_add(x, y)}
    }

    fn subtract(&self, x: i32, y: i32) -> i32 {
        unsafe {c_subtract(x, y)}
    }

    fn multiply(&self, x: u32, y: u32) -> u32 {
        unsafe {c_multiply(x, y)}
    }
}