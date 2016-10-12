#pragma once
#include "includes.h"

typedef struct
{
	color_t color;
} material_t;

material_t*
  material_new();

void
material_destroy(material_t **material);
