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
 * If you're done, try to add the CppCalculator as a bonus exercise!
 *
 * If you're curious. Rust actually builds the C++ library using a build.rs file in the main dir.
 * Alternatively, you can also include a .lib file
 */
fn main() {
    let calculators: Vec<Box<dyn Calculator>> = vec![
        Box::new(RustCalculator),
        Box::new(CCalculator),
        // Bonus: Box::new(CppCalculator),
    ];

    for calculator in calculators.iter() {
        calculator.whothis();
        println!("4 + 5: {}", calculator.add(4, 5));
        println!("4 - 5: {}", calculator.subtract(4, 5));
        println!("4 * 5: {}", calculator.multiply(4, 5));
    }
}
