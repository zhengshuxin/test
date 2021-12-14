#include <stdlib.h>
#include <stdio.h>
#include <utility>
#include <string>
#include <vector>
#include <iostream>

class demo {
public:
	demo(void) : num(new int(0)) {
		printf("construct!\r\n");
	}

	demo(const demo& d):  num(new int(*d.num)) {
		printf("copy construct\r\n");
	}

	demo(demo&& d) : num(d.num) {
		d.num = NULL;
		printf("move construct\r\n");
	}

	~demo(void) {
		printf(">>delete num=%p\r\n", num);
		delete num;
	}

	int* num;
};

static void test(void) {
	std::string str = "Hello";
	std::vector<std::string> v;
	v.push_back(str);
	std::cout << "After copy, str is \"" << str << "\"\n";
	v.push_back(std::move(str));
//	v.emplace_back(std::move(str));
	std::cout << "After move, str is \"" << str << "\"\n";

	printf("\r\n");

	demo d1;

	demo &&d2 = std::move(d1);
	printf("%s(%d): d1.num=%p, d2.num=%p\n", __func__, __LINE__, d1.num, d2.num);

	printf("\r\n");
	demo d3 = d2;
	printf("%s(%d): d1.num=%p, d2.num=%p, d3.num=%p\n", __func__, __LINE__, d1.num, d2.num, d3.num);

	printf("\r\n");
	demo d4 = (demo&&) d2;
	printf("%s(%d): d1.num=%p, d2.num=%p, d3.num=%p, d4.num=%p\n", __func__, __LINE__, d1.num, d2.num, d3.num, d4.num);

	printf("\r\n");
	demo d5 = std::move(d4);
	printf("%s(%d): d4.num=%p, d5.num=%p\r\n", __func__, __LINE__, d4.num, d5.num);

	printf("\r\n");
}

static void test1(void) {
	printf("demo1:\r\n");
	demo d1;
	printf("d1.num=%p\r\n\r\n", d1.num);

	printf("demo2:\r\n");
	demo d2 = d1;
	printf("d1.num=%p, d2.num=%p\r\n\r\n", d1.num, d2.num);

	printf("demo3:\r\n");
	demo d3 = std::move(d1);
	printf("d1.num=%p, d3.num=%p\r\n\r\n", d1.num, d3.num);
}

int main(void) {
	printf("------------------------------------------------\r\n");
	test();
	printf("------------------------------------------------\r\n");

	test1();
	printf("------------------------------------------------\r\n");

	return 0;
}
