#include <stdlib.h>
#include <stdio.h>

#include "big_values.h"

int main (int argc, char const * const argv[])
{

    // Review of const ptr vs ptr to const value
    {
        int a = 10;
        int b = 11;

        int *p_a_0 = &a; // regular pointer
        printf("*p_a_0: %d\n", *p_a_0);

        const int *p_a = &a;
        // p_a points to a const value so we cannot modify it
        // *p_a = 5;
        p_a = &b;
        printf("*p_a: %d\n", *p_a);

        int const *p_a_01 = &a; // another way to declare a pointer to a const value
        // p_a_01 is a pointer to a constant value
        // *p_a = 5;
        p_a_01 = &b;
        printf("*p_a_0: %d\n", *p_a_01);

        int *const p_a_1 = &a;
        *p_a_1 = 12;
        // p_a_1 is a const pointer so we cannot assign to a new var
        // p_a_1 = &b;
        printf("a (via p_a_1): %d\n", *p_a_1);
    }

    // Ptr of ptr
    {
        int a = 10;
        int b = 11;
        const int *p_a = &a;
        const int *p_b = &b;

        const int* *p_p_a = &p_a;
        p_p_a = &p_b; // Ok p_p_a is a pointer to a pointer to constant int
        // **p_p_a = 13; // KO
        printf("*p_p_a: %d\n", *(*p_p_a));

        int *const p_a_1 = &a;
        int *const *p_p_a_1 = &p_a_1;
        **p_p_a_1 = 13; // Ok p_p_a_1 is const pointer to a pointer
        printf("*p_p_a: %d\n", *(*p_p_a_1));

        // A pointer to a constant pointer to a constant value
        const int* const* ppa = &p_a;
        // *(*ppa) = 42; // KO: constant value
        // *ppa = &p_b; // KO: constant pointer
        ppa = &p_b;
        printf("*ppa: %d\n", *(*ppa));

        // A constant pointer to a constant pointer to a constant value
        const int *const *const ppa2 = &p_a;
        // *(*ppa) = 42; // KO: constant value
        // *ppa = &p_b; // KO: constant pointer
        // ppa2 = &p_b;
        printf("*ppa2: %d\n", *(*ppa2));
    }

    // basic
    {
        CUint256_t* bv0 = cuint256_zero();
        CUint256_t* bv1 = cuint256_one();
        printf("bv0:\n");
        fflush(stdout);
        cuint256_debug(bv0);
        printf("--\n");
        printf("bv1:\n");
        fflush(stdout);
        cuint256_debug(bv1);
        printf("--\n");

        cuint256_free(bv0);
        // Uncomment this to have a double free error
        // cuint256_free(bv0);
        cuint256_free(NULL);

        // Uncomment this to avoid mem leak in Valgrind
        cuint256_free(bv1);
    }

    // Vec
    {
        Vec_CUint256_t generated = double_rand();

        CUint256_t const* rand_0_ = generated.ptr;
        cuint256_debug(rand_0_);

        CUint256_t const* rand_0 = vec_cuint256_get(&generated, 0);

        char *f_rand_0 = cuint256_format(rand_0);
        printf("rand_0: %s\n", f_rand_0);
        // This is not recommended - unless you use libc_alloc global allocator
        // (which is using posix memalign function), which guarantees that the allocated
        // char_p::Box to be free() compatible.
        // free(f_rand_0);

        // So dev "must" use this
        free_char_p(f_rand_0);

        CUint256_t const* rand_1 = vec_cuint256_get(&generated, 1);
        // cuint256_debug(rand_1);
        cuint256_format_string_t f_rand_1 = cuint256_format_2(rand_1);
        printf("rand_1: %s\n", f_rand_1);
        cuint256_format_string_free(f_rand_1);

        // Out of bounds index (in Vec) - got NULL value
        CUint256_t const* rand_2 = vec_cuint256_get(&generated, 2);
        cuint256_debug(rand_2);

        vec_cuint256_free(generated);
    }

    {
        // CUint256_t const* x = cuint256_zero();
        // CUint256_t const* y = cuint256_one();
        CUint256_t *x = cuint256_zero();
        CUint256_t *y = cuint256_one();

        // r_x is a constant pointer on a pointer of CUint256_t
        CUint256_t *const *r_x = &x;

        BigValues_t* bvs_0 = big_values_new(r_x, &y);

        printf("x & y after big_values_new:\n");
        cuint256_debug(x);
        cuint256_debug(y);

        big_values_free(bvs_0);
        // Will leak memory if commented
        cuint256_free(x);
        cuint256_free(y);
    }

    {
        CUint256_t* x1 = cuint256_zero();
        CUint256_t* y1 = cuint256_one();
        BigValues_t* bvs_1 = big_values_new_2(&x1, &y1);
        printf("x1 & y1 after big_values_new_2:\n");
        cuint256_debug(x1);
        cuint256_debug(y1);
        big_values_free(bvs_1);
    }

    {
        Vec_CUint256_t gen_0 = double_rand();

        // CUint256_t* gen_0_0 = vec_cuint256_get(&gen_0, 0);
        // CUint256_t* gen_0_1 = vec_cuint256_get(&gen_0, 1);
        CUint256_t *gen_0_0 = vec_cuint256_get(&gen_0, 0);
        CUint256_t *gen_0_1 = vec_cuint256_get(&gen_0, 1);

        CUint256_t *const *r_gen_0_0 = &gen_0_0;

        BigValues_t* bvs_0 = big_values_new(r_gen_0_0, &gen_0_1);
        big_values_debug(bvs_0);

        big_values_free(bvs_0);
        vec_cuint256_free(gen_0);
    }

    // Will fail
    /*
    {
        Vec_CUint256_t gen_0 = double_rand();

        CUint256_t* gen_0_0 = vec_cuint256_get(&gen_0, 0);
        CUint256_t* gen_0_1 = vec_cuint256_get(&gen_0, 1);

        BigValues_t* bvs_1 = big_values_new_2(&gen_0_0, &gen_0_1);
        big_values_debug(bvs_1);

        big_values_free(bvs_1);
        vec_cuint256_free(gen_0);
    }
    */
}