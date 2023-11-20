//
// Created by Sidney on 27/09/2023.
//

#ifndef CPP_CALCULATOR_H
#define CPP_CALCULATOR_H

#include <cstdint>

#include "ICalculator.h"

class CppCalculator : public ICalculator {
public:
    CppCalculator();
    ~CppCalculator() = default;

    void whothis() override;
    uint16_t add(uint16_t x, uint16_t y) override;
    int32_t subtract(int32_t x, int32_t y) override;
    uint32_t multiply(uint32_t x, uint32_t y) override;
};

#endif //CPP_CALCULATOR_H
