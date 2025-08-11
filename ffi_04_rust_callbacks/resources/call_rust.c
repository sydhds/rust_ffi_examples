#include <stdint.h>
#include <stdio.h>

extern void rusty_cb(int32_t a);

typedef void (*rust_callback)(int32_t);
rust_callback cb;

int32_t register_callback(rust_callback callback) {
    cb = callback;
    return 1;
}

void trigger_callback() {
  cb(7); // Will call callback(7) in Rust.
}

int main(void) {
    printf("hello there\n");
    register_callback(rusty_cb);
    trigger_callback();
}