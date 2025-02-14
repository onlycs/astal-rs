// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../../gir/gir-files
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
    PRINT_CONSTANT((gint) ASTAL_EXCLUSIVITY_EXCLUSIVE);
    PRINT_CONSTANT((gint) ASTAL_EXCLUSIVITY_IGNORE);
    PRINT_CONSTANT((gint) ASTAL_EXCLUSIVITY_NORMAL);
    PRINT_CONSTANT((gint) ASTAL_KEYMODE_EXCLUSIVE);
    PRINT_CONSTANT((gint) ASTAL_KEYMODE_NONE);
    PRINT_CONSTANT((gint) ASTAL_KEYMODE_ON_DEMAND);
    PRINT_CONSTANT((gint) ASTAL_LAYER_BACKGROUND);
    PRINT_CONSTANT((gint) ASTAL_LAYER_BOTTOM);
    PRINT_CONSTANT((gint) ASTAL_LAYER_OVERLAY);
    PRINT_CONSTANT((gint) ASTAL_LAYER_TOP);
    PRINT_CONSTANT(ASTAL_MAJOR_VERSION);
    PRINT_CONSTANT(ASTAL_MICRO_VERSION);
    PRINT_CONSTANT(ASTAL_MINOR_VERSION);
    PRINT_CONSTANT((gint) ASTAL_MOUSE_BUTTON_BACK);
    PRINT_CONSTANT((gint) ASTAL_MOUSE_BUTTON_FORWARD);
    PRINT_CONSTANT((gint) ASTAL_MOUSE_BUTTON_MIDDLE);
    PRINT_CONSTANT((gint) ASTAL_MOUSE_BUTTON_PRIMARY);
    PRINT_CONSTANT((gint) ASTAL_MOUSE_BUTTON_SECONDARY);
    PRINT_CONSTANT(ASTAL_VERSION);
    PRINT_CONSTANT((guint) ASTAL_WINDOW_ANCHOR_BOTTOM);
    PRINT_CONSTANT((guint) ASTAL_WINDOW_ANCHOR_LEFT);
    PRINT_CONSTANT((guint) ASTAL_WINDOW_ANCHOR_NONE);
    PRINT_CONSTANT((guint) ASTAL_WINDOW_ANCHOR_RIGHT);
    PRINT_CONSTANT((guint) ASTAL_WINDOW_ANCHOR_TOP);
    return 0;
}
