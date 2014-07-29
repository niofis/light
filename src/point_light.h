#ifndef POINTLIGHT_H
#define POINTLIGHT_H

#include "vector3.h"
#include "color.h"

struct point_light
{
	struct vector3 position;
	struct color color;
};

struct point_light* point_light_new(int num);
void point_light_delete(struct point_light* point_light);

#endif
