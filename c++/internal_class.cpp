#include <stdio.h>

class MyClass {
public:    
    MyClass();
    ~MyClass() {}

    void func1();

private:    
    class impl;    
    impl* pimpl;
};

// MyClass.cc
class MyClass::impl {
public:    
    void func1() {
        printf(">>in impl\r\n");
    }

private:
    int a;    
    int b;
};

MyClass::MyClass() {
    pimpl = new impl;
}

void MyClass::func1() {    
    printf(">>in MyClass\r\n");
    pimpl->func1();
}

int main(void) {
    MyClass my;
    my.func1();
    return 0;
}
