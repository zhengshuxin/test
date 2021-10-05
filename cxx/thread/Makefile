all: thread future

thread: thread.cpp
	g++ thread.cpp -o thread -lpthread
future: future.cpp
	g++ future.cpp -o future -lpthread

clean cl:
	@rm -f thread future
	@echo "clean ok!"
