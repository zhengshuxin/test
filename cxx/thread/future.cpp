#include <thread>
#include <future>
#include <chrono>
#include <iostream>

static void thread_promise(std::promise<int>& p) {
	std::this_thread::sleep_for(std::chrono::seconds(1));
	int n = 123;
	p.set_value(n);

	try {
		p.set_value(n);
	} catch(std::exception& e) {
		std::cout << "set => " << e.what() << std::endl;
	}
}

static void thread_future(std::future<int>& f, int timeout) {
	std::future_status status = f.wait_for(std::chrono::seconds(timeout));
	switch (status) {
	case std::future_status::deferred:
		std::cout << "deferred" << std::endl;
		return;
	case std::future_status::ready:
		std::cout << "ready" << std::endl;
		break;
	case std::future_status::timeout:
		std::cout << "timeout" << std::endl;
		return;
	default:
		std::cout << "unknown status" << std::endl;
		return;
	}

	int n = f.get();
	std::cout << "get: " << n << std::endl;

	try {
		n = f.get();
		std::cout << "got " << n << std::endl;
	} catch(std::exception& e) {
		std::cout << "get => " << e.what() << std::endl;
	}

	std::cout << "all over, the last one is " << n << std::endl;
}

static void future1(void) {
	std::promise<int> promise;
	std::future<int>  future = promise.get_future();

	std::thread t1(thread_promise, std::ref(promise));
	std::thread t2(thread_future, std::ref(future), 1);

	t1.join();
	t2.join();
}

static void future2(void) {
	std::promise<int> promise;
	std::thread([](std::promise<int>& p) {
		std::this_thread::sleep_for(std::chrono::seconds(1));
		//p.set_value_at_thread_exit(4);
		p.set_value(4);
	}, std::ref(promise)).detach();

	std::future<int> f = promise.get_future();
	int n = f.get();
	std::cout << "future get: " << n << std::endl;
}

static void future3(void) {
	int val = 100;
	std::packaged_task<int(int)> task([](int n) {
		std::this_thread::sleep_for(std::chrono::seconds(1));
		return 4 + n;
	});

	std::thread t(std::ref(task), val) ;
	t.detach();

	std::future<int> f = task.get_future();
	int n = f.get();
	std::cout << "future get: " << n << std::endl;
}

int main(void) {
	future1();
	future2();
	future3();
	return 0;
}
