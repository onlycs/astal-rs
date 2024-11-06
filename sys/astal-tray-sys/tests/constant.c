// Generated by gir (https://github.com/gtk-rs/gir @ 74c0d5542b5c)
// from ../../gir/gir-files (@ 1783d05ebac3+)
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
    PRINT_CONSTANT((gint) ASTAL_TRAY_CATEGORY_APPLICATION);
    PRINT_CONSTANT((gint) ASTAL_TRAY_CATEGORY_COMMUNICATIONS);
    PRINT_CONSTANT((gint) ASTAL_TRAY_CATEGORY_HARDWARE);
    PRINT_CONSTANT((gint) ASTAL_TRAY_CATEGORY_SYSTEM);
    PRINT_CONSTANT(ASTAL_TRAY_MAJOR_VERSION);
    PRINT_CONSTANT(ASTAL_TRAY_MICRO_VERSION);
    PRINT_CONSTANT(ASTAL_TRAY_MINOR_VERSION);
    PRINT_CONSTANT((gint) ASTAL_TRAY_STATUS_ACTIVE);
    PRINT_CONSTANT((gint) ASTAL_TRAY_STATUS_NEEDS_ATTENTION);
    PRINT_CONSTANT((gint) ASTAL_TRAY_STATUS_PASSIVE);
    PRINT_CONSTANT(ASTAL_TRAY_VERSION);
    return 0;
}
