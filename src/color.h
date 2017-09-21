#pragma once

typedef struct
{
	float a;
	float r;
	float g;
	float b;
} color_t;

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
