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
    PRINT_CONSTANT((gint) ASTAL_WP_DEVICE_TYPE_AUDIO);
    PRINT_CONSTANT((gint) ASTAL_WP_DEVICE_TYPE_VIDEO);
    PRINT_CONSTANT(ASTAL_WP_MAJOR_VERSION);
    PRINT_CONSTANT((gint) ASTAL_WP_MEDIA_CLASS_AUDIO_MICROPHONE);
    PRINT_CONSTANT((gint) ASTAL_WP_MEDIA_CLASS_AUDIO_RECORDER);
    PRINT_CONSTANT((gint) ASTAL_WP_MEDIA_CLASS_AUDIO_SPEAKER);
    PRINT_CONSTANT((gint) ASTAL_WP_MEDIA_CLASS_AUDIO_STREAM);
    PRINT_CONSTANT((gint) ASTAL_WP_MEDIA_CLASS_VIDEO_RECORDER);
    PRINT_CONSTANT((gint) ASTAL_WP_MEDIA_CLASS_VIDEO_SINK);
    PRINT_CONSTANT((gint) ASTAL_WP_MEDIA_CLASS_VIDEO_SOURCE);
    PRINT_CONSTANT((gint) ASTAL_WP_MEDIA_CLASS_VIDEO_STREAM);
    PRINT_CONSTANT(ASTAL_WP_MICRO_VERSION);
    PRINT_CONSTANT(ASTAL_WP_MINOR_VERSION);
    PRINT_CONSTANT((gint) ASTAL_WP_SCALE_CUBIC);
    PRINT_CONSTANT((gint) ASTAL_WP_SCALE_LINEAR);
    PRINT_CONSTANT(ASTAL_WP_VERSION);
    return 0;
}
