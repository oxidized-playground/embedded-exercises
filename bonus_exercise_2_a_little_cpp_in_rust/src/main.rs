mod calculator;
mod rust_calculator;
mod c_calculator;
mod cpp_calculator;

use calculator::Calculator;
use rust_calculator::RustCalculator;
use c_calculator::CCalculator;
use cpp_calculator::CppCalculator;

/*
 * Welcome to the C in Rust exercise! We prepared a small piece of code for you to complete.
 * Can you get the CCalculator to use the C Calculator library?
 *
 * We've added a CPP example as well. if you're feeling adventurous, you can try to get that one to work too.
 *
 * If you're curious. Rust actually builds the C library using a build.rs file in the main dir.
 * Alternatively, you can also include a .lib file
 */
fn main() {
    let calculators: Vec<Box<dyn Calculator>> = vec![
        Box::new(RustCalculator),
        Box::new(CCalculator),
        Box::new(CppCalculator),
    ];

    for calculator in calculators.iter() {
        calculator.whothis();
        println!("4 + 5: {}", calculator.add(4, 5));
        println!("4 - 5: {}", calculator.subtract(4, 5));
        println!("4 * 5: {}", calculator.multiply(4, 5));
    }
}
