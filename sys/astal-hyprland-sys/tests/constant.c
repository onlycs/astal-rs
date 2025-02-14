// Generated by gir (https://github.com/gtk-rs/gir @ 26e721588f2f)
// from ../../gir/gir-files (@ 29f90b13f748+)
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
    PRINT_CONSTANT((guint) ASTAL_HYPRLAND_FULLSCREEN_CURRENT);
    PRINT_CONSTANT((guint) ASTAL_HYPRLAND_FULLSCREEN_FULLSCREEN);
    PRINT_CONSTANT((guint) ASTAL_HYPRLAND_FULLSCREEN_MAXIMIZED);
    PRINT_CONSTANT((guint) ASTAL_HYPRLAND_FULLSCREEN_NONE);
    PRINT_CONSTANT(ASTAL_HYPRLAND_MAJOR_VERSION);
    PRINT_CONSTANT(ASTAL_HYPRLAND_MICRO_VERSION);
    PRINT_CONSTANT(ASTAL_HYPRLAND_MINOR_VERSION);
    PRINT_CONSTANT((gint) ASTAL_HYPRLAND_MONITOR_TRANSFORM_FLIPPED);
    PRINT_CONSTANT((gint) ASTAL_HYPRLAND_MONITOR_TRANSFORM_FLIPPED_ROTATE_180_DEG);
    PRINT_CONSTANT((gint) ASTAL_HYPRLAND_MONITOR_TRANSFORM_FLIPPED_ROTATE_270_DEG);
    PRINT_CONSTANT((gint) ASTAL_HYPRLAND_MONITOR_TRANSFORM_FLIPPED_ROTATE_90_DEG);
    PRINT_CONSTANT((gint) ASTAL_HYPRLAND_MONITOR_TRANSFORM_NORMAL);
    PRINT_CONSTANT((gint) ASTAL_HYPRLAND_MONITOR_TRANSFORM_ROTATE_180_DEG);
    PRINT_CONSTANT((gint) ASTAL_HYPRLAND_MONITOR_TRANSFORM_ROTATE_270_DEG);
    PRINT_CONSTANT((gint) ASTAL_HYPRLAND_MONITOR_TRANSFORM_ROTATE_90_DEG);
    PRINT_CONSTANT(ASTAL_HYPRLAND_VERSION);
    return 0;
}
