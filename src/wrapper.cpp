//
// Created by Joey Shapiro on 4/2/23.
//

#include <cstdio>
#include "wrapper.h"

void cpp_hello()
{
    hello();
}

void cpp_use(void (*draw)())
{
    printf("<c_wrapper>\n");
    use(draw);
    printf("</c_wrapper>\n");
}