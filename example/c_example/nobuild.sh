cd ../../src

g++ -c wrapper.cpp -o wrapper.o
g++ -c library.cpp -o library.o
ar rc libwrapper.a *.o

mv libwrapper.a ../example/c_example
cd ../example/c_example

gcc main.c -L. -lstdc++ -lwrapper -Wl,-rpath $(pwd) -o example
# fails with:
#Undefined symbols for architecture arm64:
#  "std::__1::locale::use_facet(std::__1::locale::id&) const", referenced from:
#      std::__1::ctype<char> const& std::__1::use_facet[abi:v15006]<std::__1::ctype<char>>(std::__1::locale const&) in libwrapper.a(library.o)
#  "std::__1::ios_base::getloc() const", referenced from:
#...