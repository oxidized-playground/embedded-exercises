//
// Created by Sidney on 20/11/2023.
//

#ifndef CPP_CALCULATOR_H
#define CPP_CALCULATOR_H

#include <cstdint>

namespace cpp_calculator {
    class CppCalculator {
    public:
        CppCalculator() = default;
        void whothis();
        uint16_t add(uint16_t x, uint16_t y);
        int32_t subtract(int32_t x, int32_t y);
        uint32_t multiply(uint32_t x, uint32_t y);
    };
}

#endif //CPP_CALCULATOR_H
