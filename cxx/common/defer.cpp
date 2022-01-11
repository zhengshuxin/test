#include <iostream>
#include <mutex>
#include "defer.h"

static void defer_func(void) {
	std::cout << __func__ << " called" << std::endl;
}

static void defer_func2(const char *name, int line) {
	std::cout << "func=" << name  << ", line=" << line << std::endl;
}

static void test1(void) {
	int i = 100;
	defer [=]{
		std::cout << "In test1: i=" << i << std::endl;
	};

	i++;
}

static void test2(void) {
	int i = 100;

	defer [&]{
		std::cout << "In test2: i=" << i << std::endl;
	};

	i++;
}

static void test3(void) {
	std::cout << "In test3" << std::endl;
	defer defer_func;
	defer defer_func;
}

static void test4(void) {
	defer [] { defer_func2("test2", __LINE__); };
	defer [] { defer_func2("test2", __LINE__); };
	defer [] { defer_func2("test2", __LINE__); };
}

static void test5(void) {
	std::mutex lock;
	lock.lock();

	std::cout << "Locked ok!" << std::endl;

	defer[&] {
		lock.unlock();
		std::cout << "Unlocked ok!" << std::endl;
	};
}

int main(void) {
	test1();
	test2();
	test3();
	test4();
	test5();

	return 0;
}
