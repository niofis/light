#pragma once
#include "includes.h"

typedef struct
{
	v3_t hit_point;
	v3_t normal;
	material_t material;
	float distance;
	int hit;
} intersection_t;
