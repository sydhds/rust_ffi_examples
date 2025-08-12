#include <stdarg.h>
#include <stdint.h>
#include <stdio.h>

void foo(int n, ...) {
    va_list args;
    va_start(args, n);  
    for (int i = 0; i < n; i++) 
        printf("%d ", va_arg(args, int));
    printf("\n");
    va_end(args);
}