#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>

extern int32_t add(int32_t a, int32_t b);

int main() {
    int32_t a = 1, b = 2;
    int32_t sum = add(1, 2);
    printf("Sum = %" PRId32 "\n", sum);
    // printf("%d \n", sum); // this also works
    return 0;
}