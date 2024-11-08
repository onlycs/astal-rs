// Generated by gir (https://github.com/gtk-rs/gir @ 0cdde9fbfd9d)
// from ../../gir/gir-files (@ 611247c05351+)
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
    PRINT_CONSTANT(ASTAL_APPS_MAJOR_VERSION);
    PRINT_CONSTANT(ASTAL_APPS_MICRO_VERSION);
    PRINT_CONSTANT(ASTAL_APPS_MINOR_VERSION);
    PRINT_CONSTANT(ASTAL_APPS_VERSION);
    return 0;
}
