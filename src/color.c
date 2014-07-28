#include "color.h"

int color_to_argb(struct color* color)
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

void color_init(struct color* color, float a, float r, float g, float b)
{
	color->a = a;
	color->r = r;
	color->g = g;
	color->b = b;
}
void color_mul_scalar(struct color* res, struct color* color, float s)
{
	res->a = color->a;
	res->r = color->r * s;
	res->g = color->g * s;
	res->b = color->b * s;
}

void color_add(struct color* res, struct color* c1, struct color* c2)
{
	res->r = c1->r + c2->r;
	res->g = c1->g + c2->g;
	res->b = c1->b + c2->b;
}

void color_mul(struct color* res, struct color* c1, struct color* c2)
{
	res->r = c1->r * c2->r;
	res->g = c1->g * c2->g;
	res->b = c1->b * c2->b;
}