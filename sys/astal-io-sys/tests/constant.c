// Generated by gir (https://github.com/gtk-rs/gir @ 26e721588f2f)
// from /usr/share/gir-1.0 (@ ???)
// from ../../../gobject/gir-files (@ ???)
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    float: "%f", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT((gint) ASTAL_IO_APP_ERROR_NAME_OCCUPIED);
    PRINT_CONSTANT((gint) ASTAL_IO_APP_ERROR_TAKEOVER_FAILED);
    PRINT_CONSTANT(ASTAL_IO_MAJOR_VERSION);
    PRINT_CONSTANT(ASTAL_IO_MICRO_VERSION);
    PRINT_CONSTANT(ASTAL_IO_MINOR_VERSION);
    PRINT_CONSTANT(ASTAL_IO_VERSION);
    return 0;
}
