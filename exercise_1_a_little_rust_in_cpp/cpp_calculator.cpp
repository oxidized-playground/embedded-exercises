//
// Created by Sidney on 27/09/2023.
//

#include <iostream>

#include "cpp_calculator.h"

CppCalculator::CppCalculator() {}

void CppCalculator::whothis() {
    std::cout << "I am a C++ calculator!" << std::endl;
}

uint16_t CppCalculator::add(uint16_t x, uint16_t y) {
    return x + y;
}

int32_t CppCalculator::subtract(int32_t x, int32_t y) {
    return x - y;
}

uint32_t CppCalculator::multiply(uint32_t x, uint32_t y) {
    return x*y;
}