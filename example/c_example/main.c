#include "../../src/wrapper.h"
#include <stdio.h>

// gcc main.c -L. -lCilantro -Wl,-rpath $(pwd) -o example
int main(int argc, char **argv)
{
    cpp_hello(); // this cant be found by debugger, but lib can
    return 0;
}

//<alert>hi!</alert>