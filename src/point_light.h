#pragma once

#include "includes.h"

typedef struct
{
	v3_t position;
	color_t color;
} point_light_t;

point_light_t*
  point_light_new();

void
  point_light_destroy(point_light_t **point_light);

