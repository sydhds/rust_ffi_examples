#ifndef MY_LIB_TEST_HEADER
#define MY_LIB_TEST_HEADER

#include <stdio.h>
#include <stdlib.h>

typedef enum {
    SUCCESS,
    FAIL
} status_t;

struct opaque_t;
typedef struct opaque_t* handle_t;

status_t mylib_create(handle_t* handle);
status_t mylib_destroy(handle_t handle);
status_t mylib_set(handle_t handle, int value);
status_t mylib_dump(handle_t handle) ;

#endif
