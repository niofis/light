#pragma once

#include "vector3.h"
#include "color.h"

typedef struct
{
	v3_t position;
	color_t color;
} point_light_t;

point_light_t*
  point_light_new(int num);

void
  point_light_destroy(point_light_t **point_light);

