// Generated by gir (https://github.com/gtk-rs/gir @ 0cdde9fbfd9d)
// from ../../gir/gir-files (@ e6660f4e9430+)
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
    PRINT_CONSTANT((gint) ASTAL_MPRIS_LOOP_NONE);
    PRINT_CONSTANT((gint) ASTAL_MPRIS_LOOP_PLAYLIST);
    PRINT_CONSTANT((gint) ASTAL_MPRIS_LOOP_TRACK);
    PRINT_CONSTANT((gint) ASTAL_MPRIS_LOOP_UNSUPPORTED);
    PRINT_CONSTANT(ASTAL_MPRIS_MAJOR_VERSION);
    PRINT_CONSTANT(ASTAL_MPRIS_MICRO_VERSION);
    PRINT_CONSTANT(ASTAL_MPRIS_MINOR_VERSION);
    PRINT_CONSTANT((gint) ASTAL_MPRIS_PLAYBACK_STATUS_PAUSED);
    PRINT_CONSTANT((gint) ASTAL_MPRIS_PLAYBACK_STATUS_PLAYING);
    PRINT_CONSTANT((gint) ASTAL_MPRIS_PLAYBACK_STATUS_STOPPED);
    PRINT_CONSTANT((gint) ASTAL_MPRIS_SHUFFLE_OFF);
    PRINT_CONSTANT((gint) ASTAL_MPRIS_SHUFFLE_ON);
    PRINT_CONSTANT((gint) ASTAL_MPRIS_SHUFFLE_UNSUPPORTED);
    PRINT_CONSTANT(ASTAL_MPRIS_VERSION);
    return 0;
}
