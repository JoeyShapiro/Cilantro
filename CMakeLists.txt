cmake_minimum_required(VERSION 3.24)
project(Cilantro)

set(CMAKE_CXX_STANDARD 23)
# disable name mangling, i think
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

add_library(Cilantro SHARED src/library.cpp src/wrapper.cpp)

# for now, uses sym links :P
# move to rust example places
# these are reversed
#configure_file(cmake-build-debug/libCilantro.dylib example/rexample/ COPYONLY)
#configure_file(cmake-build-debug/libCilantro.dylib example/rexample/ COPYONLY)

# add to c example
