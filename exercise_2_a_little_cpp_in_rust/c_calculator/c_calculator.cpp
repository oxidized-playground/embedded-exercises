//
// Created by Sidney on 27/09/2023.
//

#include <iostream>

#include "c_calculator.h"

// you should "C" that there is something missing here

namespace c_calculator {

void c_whothis() {
    printf("I am a C calculator!\n");
}

uint16_t c_add(uint16_t x, uint16_t y) {
    return x + y;
}

int32_t c_subtract(int32_t x, int32_t y) {
    return x - y;
}

uint32_t c_multiply(uint32_t x, uint32_t y) {
    return x*y;
}



}