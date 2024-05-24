//
// Created by Sidney on 27/09/2023.
//

#include "cpp_calculator_wrapper.h"
#include "cpp_calculator.h"

extern "C" {

namespace cpp_calculator {

CppCalculator* cpp_calculator_new() {
    return new cpp_calculator::CppCalculator();
}

void cpp_calculator_delete(CppCalculator* calc) {
    delete calc;
}

void cpp_whothis(CppCalculator* calc) {
    calc->whothis();
}

uint16_t cpp_add(CppCalculator* calc, uint16_t x, uint16_t y) {
    return calc->add(x, y);
}

int32_t cpp_subtract(CppCalculator* calc, int32_t x, int32_t y) {
    return calc->subtract(x, y);
}

uint32_t cpp_multiply(CppCalculator* calc, uint32_t x, uint32_t y) {
    return calc->multiply(x, y);
}

}

}