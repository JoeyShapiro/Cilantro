#include "../../src/wrapper.h"
#include <stdio.h>

void multiply() { printf("hello"); }

// gcc main.c -L. -lstdc++ -lCilantro -Wl,-rpath $(pwd) -o example
int main(int argc, char **argv)
{
    cpp_hello(); // this cant be found by debugger, but lib can
    cpp_use(&multiply);
    return 0;
}

//<alert>hi!</alert>