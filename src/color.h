#ifndef COLOR_H
#define COLOR_H

struct color
{
	float a;
	float r;
	float g;
	float b;
};

int color_to_argb(struct color* color);

void color_init(struct color* color, float a , float r, float g, float b);
void color_mul_scalar(struct color* color, float s);
#endif
