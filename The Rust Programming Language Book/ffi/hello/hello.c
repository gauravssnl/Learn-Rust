#include <stdio.h>

// declare Rust function that we will use here
extern void hello_from_rust();

int main() {
    hello_from_rust();
    return 0;
}