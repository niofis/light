#pragma once

typedef struct
{
	float a;
	float r;
	float g;
	float b;
} color_t;

#define c_white {1.0f, 1.0f, 1.0f, 1.0f}
#define c_blue {1.0f, 0.0f, 0.0f, 1.0f}
#define c_red {1.0f, 1.0f, 0.0f, 0.0f}
#define c_green {1.0f, 0.0f, 1.0f, 0.0f}
#define c_black {1.0f, 0.0f, 0.0f, 0.0f}
#define c_gray {1.0f, 0.5f, 0.5f, 0.5f}
#define c_orange {1.0f, 1.0f, 0.5f, 0.0f}
#define c_purple {1.0f, 1.0f, 0.0f, 1.0f}
#define c_cyan {1.0f, 0.0f, 1.0f, 1.0f}
#define c_yellow {1.0f, 1.0f, 1.0f, 0.0f}

int
  color_to_argb(color_t *color);

void
  color_set_argb(color_t *color, float a , float r, float g, float b);

void
  color_mul_scalar(color_t* dest, const color_t *color, float s);

void
  color_add(color_t *dest, const color_t *c1, const color_t *c2);

void
  color_mul(color_t *dest, const color_t *c1, const color_t *c2);

void
  color_copy(color_t *dest, const color_t *src);
