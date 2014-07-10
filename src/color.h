#ifndef COLOR_H
#define COLOR_H

typedef struct color
{
	float a;
	float r;
	float g;
	float b;
};

int color_to_argb(struct color* color);

void color_init(struct color* color, float a , float r, float g, float b);

#endif