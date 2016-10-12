#pragma once

#include "includes.h"

typedef struct
{
	v3_t direction;
	v3_t origin;
} ray_t;

ray_t*
  ray_new();

void
  ray_destroy(ray_t **ray);
