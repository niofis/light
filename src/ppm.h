#pragma once
#define PPM_P6 1
#define PPM_P3 2

int ppm_create(char* filename, int width, int height, short max_color, int format, int* data);
