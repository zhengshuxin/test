#include <iostream>

using namespace std;
struct A {
	int i;
	A(int ii) : i(ii) {}
};

A operator + (int n, const A & a) {
	return A(a.i + n);
}

template <class T1, class T2>
auto add(T1 x, T2 y) -> decltype(x + y) {
	return x + y;
}

int main(void) {
	auto d = add(100, 1.5);  // d 是 double 类型，d = 101.5
	auto k = add(100, A(1));  // k 是 A 类型，因为表达式“100+A(1)”是A类型的
	cout << d << endl;
	cout << k.i << endl;
	return 0;
}
