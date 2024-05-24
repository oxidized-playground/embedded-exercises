use super::calculator::Calculator;

#[repr(C)]
pub struct CppCalculator;

// We can't just create a cpp_calculator object in Rust and call its methods. We need to get the object
// and use it when calling the cpp wrapper methods.

extern "C" {
    fn cpp_calculator_new() -> *mut std::ffi::c_void;
    fn cpp_calculator_delete(calc: *mut std::ffi::c_void);
    fn cpp_whothis(calc: *mut std::ffi::c_void);
    fn cpp_add(calc: *mut std::ffi::c_void, x: i16, y: i16) -> i16;
    fn cpp_subtract(calc: *mut std::ffi::c_void, x: i32, y: i32) -> i32;
    fn cpp_multiply(calc: *mut std::ffi::c_void, x: u32, y: u32) -> u32;
}

impl Calculator for CppCalculator {
    fn whothis(&self) {
        todo!();
    }

    fn add(&self, x: i16, y: i16) -> i16 {
        todo!();
    }

    fn subtract(&self, x: i32, y: i32) -> i32 {
        todo!();
    }

    fn multiply(&self, x: u32, y: u32) -> u32 {
        todo!();
    }
}