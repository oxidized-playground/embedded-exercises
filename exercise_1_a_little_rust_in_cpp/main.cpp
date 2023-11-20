#include <iostream>
#include <memory>
#include <vector>

#include "cpp_calculator.h"
#include "rust_calculator.h"

/*
 * Welcome to the Rust in C++ exercise! We prepared a small piece of code for you to complete.
 * Can you get the RustCalculator to use the Rust Calculator library?
 *
 * If you're curious. The CMakeLists includes a tool called Corrosion. It takes care of building the Rust
 * library that it's pointed at. Just remember that you need to copy the .lib file into your build folder.
 * https://github.com/corrosion-rs/corrosion
 */
int main() {
    std::vector<std::shared_ptr<ICalculator>> calculators;
    calculators.push_back(std::make_shared<CppCalculator>(CppCalculator()));
    calculators.push_back(std::make_shared<RustCalculator>(RustCalculator()));

    for(const std::shared_ptr<ICalculator> calculator : calculators) {
        calculator->whothis();
        std::cout << "4 + 5 = " << calculator->add(4, 5) << std::endl;
        std::cout << "4 - 5 = " << calculator->subtract(4, 5) << std::endl;
        std::cout << "4 * 5 = " << calculator->multiply(4, 5) << std::endl;
    }
    return 0;
}
