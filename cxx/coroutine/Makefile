coroutine: coroutine.o
	g++ -o coroutine coroutine.o -lpthread
coroutine.o: coroutine.cpp
	g++ -c coroutine.cpp -fcoroutines

c2: c2.o
	g++ -o c2 c2.o -lpthread
c2.o: c2.cpp
	g++ -c c2.cpp -fcoroutines
clean cl:
	rm -f coroutine coroutine.o
	rm -f c2 c2.o
