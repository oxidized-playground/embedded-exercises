use super::calculator::Calculator;

pub struct RustCalculator;

impl Calculator for RustCalculator {
    fn whothis(&self) {
        println!("Hi I'm a rust calculator!");
    }
    fn add(&self, x: i16, y: i16) -> i16 {
        x + y
    }

    fn subtract(&self, x: i32, y: i32) -> i32 {
        x - y
    }

    fn multiply(&self, x: u32, y: u32) -> u32 {
        x * y
    }
}