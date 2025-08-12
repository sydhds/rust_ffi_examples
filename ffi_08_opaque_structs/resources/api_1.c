#include <stdio.h>
#include <stdbool.h>
#include <stdint.h>

/*
typedef struct {
    uint32_t field1;
    uint32_t field2;
    uint32_t field3;
} FileData; */
/* extern */ struct FileData_t;
typedef struct FileData_t FileData;

extern FileData* file_data_new(void);
extern void file_data_debug(FileData *fd);
extern void file_data_set_field4(FileData *fd, uint8_t value);
extern void file_data_set_field4_v2(FileData *fd, uint8_t value);
extern void file_data_free(FileData *fd);
extern bool file_data_read(FileData *fd, char* path);

int main(void) {

    FileData* file_data = file_data_new();

    // printf("FileData debug: {} {} {}", file_data.field1, file_data.field2, file_data.field3);

    file_data_debug(file_data);
    file_data_set_field4(file_data, 250);
    file_data_set_field4_v2(file_data, 150);
    file_data_set_field4_v2(NULL, 100);
    file_data_debug(file_data);

    bool ret = file_data_read(NULL, "./api_1.c");
    if (ret) {
        printf("[NULL ptr] Successfully read api_1.c\n");
    } else {
        printf("[NULL ptr] Failed reading api_1.c\n");
    }

    bool ret2 = file_data_read(file_data, "./api_1.c");
    if(ret2) {
        // Do something
        printf("Successfully reading api_1.c file\n");
    }
    file_data_free(file_data);

}