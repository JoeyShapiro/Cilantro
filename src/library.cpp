#include "library.h"

#include <iostream>

class Test {
public:
	std::string something;
};

void hello() {
	Test hi = Test();
	hi.something = "thing";
	std::cout << "class that isnt exposed: " << hi.something << std::endl;
	std::cout << "Hello, World!" << std::endl;
}

void use(void (*draw)()) {
	printf("<c++_wrapper>\n");
	draw();
	printf("</c++_wrapper>\n");
}
