#include <thread>
#include <iostream>
#include <condition_variable>
#include <mutex>
#include <deque>

std::deque<int> dq;
std::mutex mutex;
std::condition_variable cond;

static void producer(void) {
	std::thread::id id = std::this_thread::get_id();
	std::cout << "Thread producer is " <<  id << std::endl;

	int i = 0;
	while (true) {
		std::unique_lock<std::mutex> locker(mutex);
		dq.push_front(i++);
		locker.unlock();
		cond.notify_one();
		std::this_thread::sleep_for(std::chrono::seconds(1));
	}
}

static void consumer(void) {
	std::thread::id id = std::this_thread::get_id();
	std::cout << "Thread consumer is " <<  id << std::endl;

	while (true) {
		std::unique_lock<std::mutex> locker(mutex);
		cond.wait(locker, [=] { return !dq.empty(); });
		std::cout << "consume: " << dq.front() << std::endl;
		dq.pop_front();
		locker.unlock();
	}
}

int main(void) {
	std::thread t1(consumer);
	std::thread t2(producer);

	t1.join();
	t2.join();
	return 0;
}
