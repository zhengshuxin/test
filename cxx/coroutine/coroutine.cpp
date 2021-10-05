#include <coroutine>
#include <functional>
#include <iostream>
#include <thread>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

struct Task {
    struct promise_type {
        auto get_return_object() { return Task{}; }
        auto initial_suspend() { return std::suspend_never{}; }
        auto final_suspend() { return std::suspend_never{}; }
        void unhandled_exception() { std::terminate(); }
        void return_void() {}
    };
};

using call_back = std::function<void(int)>;

void Add100ByCallback(int init, call_back f) { //init是传入的初始值，add之后的结果由回调函数f通知
	f(init + 100);
	return;

	std::thread t([init, f]() {
		//printf("%s(%d) called\r\n", __func__, __LINE__);
		//std::this_thread::sleep_for(std::chrono::seconds(1)); // sleep一下，假装很耗时
		f(init + 100); // 耗时的计算完成了，调用回调函数
	});
	t.detach();
}

struct Add100AWaitable {
	Add100AWaitable(int init):init_(init) {}
	bool await_ready() const {
		printf("%s called\r\n", __func__);
		return false;
	}
	int await_resume() {
		printf("%s called\r\n", __func__);
		return result_;
	}
	void await_suspend(std::coroutine_handle<> handle) {
		printf("%s called\r\n", __func__);
		// 定义一个回调函数，在此函数中恢复协程
		auto f = [handle, this](int value) mutable {
			result_ = value;
			handle.resume(); // 这句是关键
		};
		Add100ByCallback(init_, f); 
	}
	int init_; // 将参数存在这里
	int result_; // 将返回值存在这里
};

Task Add100ByCoroutine(int init, call_back f) {
	int n = 0;
	for (int i = 0; i < 100; i++) {
		n = co_await Add100AWaitable(init + i);
	}
	f(n);
}

int main(void) {
	Add100ByCoroutine(10, [](int value){ std::cout<<"get result from coroutine: "<<value<<"\n"; });
	Add100ByCoroutine(10, [](int value){ std::cout<<"get result from coroutine: "<<value<<"\n"; });
	sleep(20);
	return 0;
}
