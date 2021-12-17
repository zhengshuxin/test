all: thread thread2 future
cflags = -std=c++17

thread: thread.cpp
	g++ $(cflags) thread.cpp -o thread -lpthread
thread2: thread2.cpp
	g++ $(cflags) thread2.cpp -o thread2 -lpthread
future: future.cpp
	g++ $(cflags) future.cpp -o future -lpthread

clean cl:
	@rm -f thread future
	@echo "clean ok!"
