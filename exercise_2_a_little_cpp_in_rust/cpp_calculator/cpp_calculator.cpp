//
// Created by Sidney on 27/09/2023.
//

#include <iostream>

#include "cpp_calculator.h"

// you should "C" that there is something missing here

namespace cpp_calculator {

void cpp_whothis() {
    std::cout << "I am a C++ calculator!" << std::endl;
}

uint16_t cpp_add(uint16_t x, uint16_t y) {
    return x + y;
}

int32_t cpp_subtract(int32_t x, int32_t y) {
    return x - y;
}

uint32_t cpp_multiply(uint32_t x, uint32_t y) {
    return x*y;
}



}