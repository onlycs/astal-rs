// Generated by gir (https://github.com/gtk-rs/gir @ 0cdde9fbfd9d)
// from ../../gir/gir-files (@ e6660f4e9430)
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
    PRINT_CONSTANT((gint) ASTAL_NOTIFD_CLOSED_REASON_CLOSED);
    PRINT_CONSTANT((gint) ASTAL_NOTIFD_CLOSED_REASON_DISMISSED_BY_USER);
    PRINT_CONSTANT((gint) ASTAL_NOTIFD_CLOSED_REASON_EXPIRED);
    PRINT_CONSTANT((gint) ASTAL_NOTIFD_CLOSED_REASON_UNDEFINED);
    PRINT_CONSTANT(ASTAL_NOTIFD_MAJOR_VERSION);
    PRINT_CONSTANT(ASTAL_NOTIFD_MICRO_VERSION);
    PRINT_CONSTANT(ASTAL_NOTIFD_MINOR_VERSION);
    PRINT_CONSTANT((gint) ASTAL_NOTIFD_URGENCY_CRITICAL);
    PRINT_CONSTANT((gint) ASTAL_NOTIFD_URGENCY_LOW);
    PRINT_CONSTANT((gint) ASTAL_NOTIFD_URGENCY_NORMAL);
    PRINT_CONSTANT(ASTAL_NOTIFD_VERSION);
    return 0;
}
