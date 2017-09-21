#include "includes.h"

int
color_to_argb(color_t *color)
{
	int argb = 0;
    
	argb = (int) ((color->a < 1.0f ? color->a : 1.0f) * 255);
	argb = argb << 8;
	argb |= (int) ((color->r < 1.0f ? color->r : 1.0f) * 255);
	argb = argb << 8;
	argb |= (int) ((color->g < 1.0f ? color->g : 1.0f) * 255);
	argb = argb << 8;
	argb |= (int) ((color->b < 1.0f ? color->b : 1.0f) * 255);

	return argb;
}

void
color_set_argb(color_t *color, float a, float r, float g, float b)
{
	color->a = a;
	color->r = r;
	color->g = g;
	color->b = b;
}

void
color_mul_scalar(color_t *dest, const color_t *color, float s)
{
	dest->a = color->a;
	dest->r = color->r * s;
	dest->g = color->g * s;
	dest->b = color->b * s;
}

void
color_add(color_t *dest, const color_t *c1, const color_t *c2)
{
	dest->r = c1->r + c2->r;
	dest->g = c1->g + c2->g;
	dest->b = c1->b + c2->b;
}

void
color_mul(color_t *dest, const color_t *c1, const color_t *c2)
{
	dest->r = c1->r * c2->r;
	dest->g = c1->g * c2->g;
	dest->b = c1->b * c2->b;
}

void
color_copy(color_t *dest, const color_t *src)
{
  dest->a = src->a;
  dest->r = src->r;
  dest->g = src->g;
  dest->b = src->b;
}
