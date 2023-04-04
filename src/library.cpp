#include "library.h"

#include <iostream>

void hello() {
    std::cout << "Hello, World!" << std::endl;
}

void use(void (*draw)()) {
    printf("<c++_wrapper>\n");
    draw();
    printf("</c++_wrapper>\n");
}
