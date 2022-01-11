#pragma once

#include <functional>

// refer: https://blog.csdn.net/u013597671/article/details/62121035

// defer所执行的函数的类型
typedef std::function<void()> __DeferFunc;

void __deferCleanUp(__DeferFunc* func) {
	(*func)();
}

#define __Combine1(x, y) x##y
#define __Combine0(x, y) __Combine1(x, y)

#define defer __DeferFunc __Combine0(deferCleanUp_, __LINE__) \
	__attribute__((cleanup(__deferCleanUp), unused)) =
