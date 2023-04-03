//
// Created by Joey Shapiro on 4/2/23.
//

#ifndef CILANTRO_WRAPPER_H
#define CILANTRO_WRAPPER_H

#ifdef __cplusplus
#include "library.h"
extern "C" {
#else
typedef struct CppClass CppClass;
#endif

void cpp_hello();

#ifdef __cplusplus
}
#endif

#endif //CILANTRO_WRAPPER_H
