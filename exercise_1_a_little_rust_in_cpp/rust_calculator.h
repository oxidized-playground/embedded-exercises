//
// Created by Sidney on 27/09/2023.
//

#ifndef RUST_CALCULATOR_H
#define RUST_CALCULATOR_H

#include "ICalculator.h"


class RustCalculator : public ICalculator {
public:
    RustCalculator();
    ~RustCalculator() = default;

    void whothis() override;
    uint16_t add(uint16_t x, uint16_t y) override;
    int32_t subtract(int32_t x, int32_t y) override;
    uint32_t multiply(uint32_t x, uint32_t y) override;
};


#endif //RUST_CALCULATOR_H
