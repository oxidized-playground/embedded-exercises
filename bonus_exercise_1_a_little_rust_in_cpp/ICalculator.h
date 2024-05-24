//
// Created by Sidney on 27/09/2023.
//

#ifndef CPP_ICALCULATOR_H
#define CPP_ICALCULATOR_H

#include <cstdint>

class ICalculator {
public:
    virtual ~ICalculator() = default;

    virtual void whothis() = 0;
    virtual uint16_t add(uint16_t x, uint16_t y) = 0;
    virtual int32_t subtract(int32_t x, int32_t y) = 0;
    virtual uint32_t multiply(uint32_t x, uint32_t y) = 0;
};

#endif //CPP_ICALCULATOR_H