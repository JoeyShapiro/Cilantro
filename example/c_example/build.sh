cd ../../src

gcc -lstdc++ -c wrapper.cpp -o wrapper.o
gcc -lstdc++ -c library.cpp -o library.o
ar rc libwrapper.a *.o

mv libwrapper.a ../example/c_example
cd ../example/c_example

gcc main.c -L. -lstdc++ -lwrapper -Wl,-rpath $(pwd) -o example
# works fine