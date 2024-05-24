//
// Created by Sidney on 27/09/2023.
//

#ifndef CPP_CALCULATOR_WRAPPER_H
#define CPP_CALCULATOR_WRAPPER_H

#include <cstdint>

extern "C" {

namespace cpp_calculator {

typedef class CppCalculator CppCalculator;

CppCalculator* cpp_calculator_new();
void cpp_calculator_delete(CppCalculator* calc);
void cpp_whothis(CppCalculator* calc);
uint16_t cpp_add(CppCalculator* calc, uint16_t x, uint16_t y);
int32_t cpp_subtract(CppCalculator* calc, int32_t x, int32_t y);
uint32_t cpp_multiply(CppCalculator* calc, uint32_t x, uint32_t y);

}

}

#endif //CPP_CALCULATOR_WRAPPER_H
