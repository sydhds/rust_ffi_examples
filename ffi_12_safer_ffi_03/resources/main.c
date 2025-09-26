#include <stdlib.h>
#include <stdio.h>

#include "rust_path.h"

int main (int argc, char const * const argv[])
{
    printf("Hello there!\n");

    // basic
    {
        RustPath_t *rp_0 = rust_path_default();
        printf("rp_0:\n");
        fflush(stdout);
        rust_path_debug(rp_0);

        RustPath_t *rp_1 = rust_path_default2();
        printf("rp_1:\n");
        fflush(stdout);
        rust_path_debug(rp_1);

        rust_path_free(rp_0);
        rust_path_free(rp_1);
    }

    // Result 1
    {
        RustPath_t *rp_0 = rust_path_default();
        printf("R1 rp_0:\n");
        fflush(stdout);
        rust_path_debug(rp_0);

        RustPath_t *rp_1 = rust_path_default2();
        printf("R1 rp_1:\n");
        fflush(stdout);
        rust_path_debug(rp_1);

        CResult_RustPath_char_ptr_t res = rust_path_canonicalize(&rp_0);

        // Can print directly
        // rust_path_debug(res.ok);
        if (res.ok != NULL) {
            printf("Got a Ok\n");
            rust_path_debug(res.ok);
        } else {
            printf("Got an Err\n");
            printf("%s\n", res.err);
        }

        cresult_free(res);

        CResult_RustPath_char_ptr_t res_1 = rust_path_canonicalize(&rp_1);

        // Can print directly
        // rust_path_debug(res.ok);
        if (res_1.ok != NULL) {
            printf("Got a Ok\n");
            rust_path_debug(res_1.ok);
        } else {
            printf("Got an Err\n");
            printf("%s\n", res_1.err);
        }

        cresult_free(res_1);

        rust_path_free(rp_0);
        rust_path_free(rp_1);

    }

    // Result 2
    {
        RustPath_t *rp_0 = rust_path_default();
        printf("R2 rp_0:\n");
        fflush(stdout);
        rust_path_debug(rp_0);

        RustPath_t *rp_1 = rust_path_default2();
        printf("R2 rp_1:\n");
        fflush(stdout);
        rust_path_debug(rp_1);

        CResult2_RustPath_IoError_ptr_t res = rust_path_canonicalize_2(&rp_0);

        if (res.ok != NULL) {
            printf("R2 - rp_0 - Got a Ok\n");
            rust_path_debug(res.ok);
        } else {
            printf("R2 - rp_1 - Got an Err\n");
            // printf("%s\n", res.err);
            io_error_debug(&res.err);
        }

        cresult2_free(res);

        CResult2_RustPath_IoError_ptr_t res_1 = rust_path_canonicalize_2(&rp_1);

        // Can print directly
        // rust_path_debug(res.ok);
        if (res_1.ok != NULL) {
            printf("R2 - rp_1 - Got a Ok\n");
            rust_path_debug(res_1.ok);
        } else {
            printf("R2 - rp_1 - Got an Err\n");
            // printf("%s\n", res_1.err);
            io_error_debug(&res_1.err);
        }


        cresult2_free(res_1);

        rust_path_free(rp_0);
        rust_path_free(rp_1);

    }
}